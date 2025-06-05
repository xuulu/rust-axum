use sea_orm::{Database, DatabaseConnection, ColumnTrait, DbErr, EntityTrait, QuerySelect, Value, sea_query::Expr, Iterable, PrimaryKeyTrait};
use sea_orm::sea_query::Func;
use crate::config::Settings;

#[derive(Clone)]
pub struct PgsqlPool(DatabaseConnection);

impl PgsqlPool {
    pub async fn init() -> Self {
        let mut options = sea_orm::ConnectOptions::new(
            format!(
                "postgres://{}:{}@{}:{}/{}",
                Settings::get("DB_USERNAME").unwrap(),
                Settings::get("DB_PASSWORD").unwrap(),
                Settings::get("DB_HOST").unwrap(),
                Settings::get("DB_PORT").unwrap(),
                Settings::get("DB_DATABASE").unwrap()
            )
        );
        options.max_connections(20).min_connections(5);
        let conn = Database::connect(options).await.expect("数据库连接失败");
        PgsqlPool(conn)
    }

    /// 提供内部连接引用
    pub fn get_conn(&self) -> &DatabaseConnection {
        &self.0
    }

    /// 通用查询：  
    /// 通过主键查询（适用于所有实现 EntityTrait 的模型）
    ///
    /// ```
    /// let model = db.find_by_id::<Use>(1).await.unwrap();
    /// ```
    ///
    pub async fn find_by_id<E>(
        &self,
        id: i32,
    ) -> Result<Option<<E as EntityTrait>::Model>, DbErr>
    where
        E: EntityTrait,
        E::PrimaryKey: PrimaryKeyTrait<ValueType = i32>
    {
        E::find_by_id(id).one(&self.0).await
    }


    /// 获取主键最大值（适用于数字类型主键）
    /// 
    /// ```
    ///  let max_id = db.get_max_id::<Use>().await.unwrap();
    /// ```
    /// 
    pub async fn get_max_id<E>(&self) -> Result<Option<i32>, DbErr>
    where
        E: EntityTrait,
    {
        let pk_col = E::PrimaryKey::iter().next().expect("模型未定义主键");

        let stmt = E::find()
            .select_only()
            .expr_as(Func::max(Expr::col(pk_col.clone())), "max_id");

        let result = stmt.into_tuple::<Option<i32>>().one(&self.0).await?;

        Ok(result.flatten())
    }
    
    
}
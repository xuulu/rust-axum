use deadpool_redis::{Config as RedisConfig, Connection, Pool, Runtime};
use redis::{AsyncCommands, RedisResult};
use serde::de::DeserializeOwned;
use serde::Serialize;
use crate::config::Settings;

#[derive(Clone)]
pub struct RedisPool(Pool);

impl RedisPool {
    pub fn init() -> Self {
        let redis_host= Settings::get("REDIS_HOST").unwrap();
        let redis_password= Settings::get("REDIS_PASSWORD").unwrap();
        let redis_db= Settings::get("REDIS_db").unwrap();

        let redis_url = format!("redis://:{}@{}/{}", redis_password, redis_host, redis_db);

        let cfg = RedisConfig::from_url( redis_url);
        let pool = cfg.create_pool(Some(Runtime::Tokio1)).expect("创建 Redis 池失败");
        RedisPool(pool)
    }
    
    /// 获取 Redis 连接池
    pub async fn get_conn(&self) -> Connection {
        self.0.get().await.expect("获取 Redis 连接失败")
    }
    
    
    ///  获取字符串值
    pub async fn get(&self,key: &str) -> RedisResult<Option<String>> {
        let mut conn = self.get_conn().await;
        conn.get(key).await
    }
     
    /// 设置字符串值
    pub async fn set(&self,key: &str, value: &str) -> RedisResult<()> {
        let mut conn = self.get_conn().await;
        conn.set(key, value).await
    }
    
    /// 设置字符串值 带时间（秒数） 默认 12 小时
    pub async fn set_ex(&self,key: &str, value: &str, expiration: Option<u64>) -> RedisResult<()> {
        let mut conn = self.get_conn().await;
        let ex=expiration.unwrap_or(60 * 60 * 12); // 默认12小时
        conn.set_ex(key, value, ex).await
    }
    
    /// 获取json值
    pub async fn get_json<T: DeserializeOwned>(&self,key: &str) -> RedisResult<Option<T>> {
        let value = self.get(key).await?;
        
        match value { 
            Some(str) => {
                let v = serde_json::from_str(&str).unwrap();
                Ok(Some(v))
            },
            None => Ok(None),
        }
    }
    
    /// 设置json值
    pub async fn set_json<T: serde::Serialize>(
        &self,
        key: &str,
        value: &T,
        expiration: Option<u64>,
    ) -> RedisResult<()> {
        let value = serde_json::to_string(value).unwrap();

        self.set_ex(key, &value, expiration).await
        
    }
    
    /// 旁路缓存 Cache-Aside-Pattern
    pub async fn get_or_set_json<T, F, Fut>(
        &self,
        key: &str, // 缓存键
        expire_secs: Option<u64>,   //  过期时间（秒）
        load_fn: F, // 加载数据的函数
    ) -> Result<Option<T>, anyhow::Error>
    where
        T: Serialize + DeserializeOwned+Send+Sync+'static,
        F: FnOnce() -> Fut,
        Fut: Future<Output = Option<T>>,
    {
        // 尝试从缓存获取
        if let Ok(Some(cached)) = self.get_json(key).await {
            return Ok(Some(cached));
        }
        
        
        // 未命中，加载新数据
        match load_fn().await {
            Some(new_data) => {
                self.set_json(key, &new_data, expire_secs).await?;
                Ok(Some(new_data))
            }
            None => Ok(None),
        }
    }
}


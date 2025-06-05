use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use sea_orm::entity::prelude::*;
use sea_orm::QueryOrder;
use serde::Deserialize;
use crate::models::apilist;
use crate::models::prelude::Apilist;
use crate::state::AppStateArc;

/// 获取全部接口
pub async fn get_all_api_lists(
    State(state): State<AppStateArc>,
) -> impl IntoResponse {
    let api_list = Apilist::find()
        .order_by_asc(apilist::Column::Id)
        .all(state.db.get_conn())
        .await
        .unwrap();

    Json(api_list)
}

/// 搜索参数
#[derive(Debug, Deserialize)]
pub struct SearchParams {
    q: Option<String>,
}
/// 模糊搜索
pub async fn get_search_api_lists(
    State(state): State<AppStateArc>,
    Query(params): Query<SearchParams>,
) -> impl IntoResponse {
    let q = match params.q {
        Some(q) if !q.is_empty() => q,
        _ => return Json(vec![]),
    };

    let api_list = Apilist::find()
        .filter(
            apilist::Column::Name.contains(&q)
                .or(apilist::Column::Path.contains(&q))
                .or(apilist::Column::Introduce.contains(&q))
        )
        .order_by_asc(apilist::Column::Id)
        .all(state.db.get_conn())
        .await
        .unwrap();

    Json(api_list)
}


/// 获取id
pub async fn get_api_list_by_id(
    State(state): State<AppStateArc>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    
    let api_list = state.db.find_by_id::<Apilist>(id).await.unwrap();
    
    match api_list {
        Some(model) => Ok(Json(model)),
        None => Err(StatusCode::NOT_FOUND)
    }
}


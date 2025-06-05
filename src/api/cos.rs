use crate::models::prelude::{DataCosImage, DataCosVideo};
use crate::response::{ApiResponse, AppError, JsonResponse, ValidQuery};
use crate::state::AppStateArc;
use axum::extract::State;
use rand::Rng;
use sea_orm::EntityTrait;

#[derive(serde::Deserialize)]
pub struct QueryParams {
    cos_image: Option<bool>,
    cos_video: Option<bool>,
}

#[axum::debug_handler]
pub async fn cos(
    State(state): State<AppStateArc>,
    ValidQuery(params): ValidQuery<QueryParams>,
) -> ApiResponse {
    let (redis, db) = (state.redis.clone(), state.db.clone());
    
    let url:Option<String> = match (params.cos_image,params.cos_video) {
        (Some(true), None) | (Some(true), Some(false)) => {
            // cos_image 为 true
            // 尝试从缓存获取
            let max_id = redis.get_or_set_json("cos_image", None, || async { db.get_max_id::<DataCosImage>().await.unwrap() }
            ).await?.ok_or(AppError::NotFound)?;
            // 获取随机 ID
            let random_id = rand::rng().random_range(1..=max_id);
            // 获取模型
            let model = db.find_by_id::<DataCosImage>(random_id).await?.ok_or(AppError::NotFound)?;
            // 获取图片 URL
            Some(format!("https://cdn.cdnjson.com/pic.html?url={}",model.url))
        }
        (None, Some(true)) | (Some(false), Some(true)) => {
            // cos_video 为 true
            // 尝试从缓存获取
            let max_id = redis.get_or_set_json(
                "cos_video",
                None,
                || async { db.get_max_id::<DataCosVideo>().await.unwrap() }
            ).await?.ok_or(AppError::NotFound)?;
            // 获取随机 ID
            let random_id = rand::rng().random_range(1..=max_id);
            // 获取模型
            let model = db.find_by_id::<DataCosVideo>(random_id).await?.ok_or(AppError::NotFound)?;
            Some(model.url)
        }
        _ => None
    };
    
    if url.is_none() {
        return Ok(JsonResponse::error(400,
                               Some(serde_json::Value::String("请检查参数是否正确!".to_string()))
        ))
    }

    let data = serde_json::json!({
            "title": "逆天cos",
            "url": url
        });
    
    Ok(JsonResponse::success(data))
}

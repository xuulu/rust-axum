use axum::extract::rejection::QueryRejection;
use axum::response::{IntoResponse, Response};


use crate::response::JsonResponse;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Not found")]
    NotFound,

    #[error("InternalError")]
    InternalError,

    #[error("ValidationError")]
    ValidationError,

    #[error("ValidationError：{0}")]
    Query(#[from] QueryRejection),

    #[error("JSON解析失败: {0}")]
    JsonParse(#[from] serde_json::Error),

    #[error("{0}")]
    Path(#[from] axum::extract::rejection::PathRejection),

    #[error("{0}")]
    Reqwest(#[from] reqwest::Error),
    
    #[error("{0}")]
    SeaOrm(#[from] sea_orm::DbErr),

    #[error("{0}")]
    Redis(#[from] redis::RedisError),

    #[error("{0}")]
    DeadpoolRedis(#[from] deadpool_redis::PoolError),

    #[error("{0}")]
    Scraper(#[from] Box<dyn std::error::Error + Send + Sync>),

    #[error("anyhow，{0}")]
    Anyhow(#[from] anyhow::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (code, detail) = match self {
            AppError::NotFound => (404, "Not Found".to_string()),
            AppError::InternalError => (500, "InternalError".to_string()),
            AppError::ValidationError => (400, "ValidationError".to_string()),
            AppError::Query(e) => (400, e.to_string()),
            AppError::Path(ref e) => (422, e.to_string()),
            AppError::Reqwest(ref e) => (500, e.to_string()),
            AppError::SeaOrm(ref e) => (500, e.to_string()),
            AppError::Redis(ref e) => (500, e.to_string()),
            AppError::DeadpoolRedis(ref e) => (500, e.to_string()),
            _ => (500, "Internal Server Error".to_string()),
        };
        JsonResponse::error(code, Some(serde_json::Value::String(detail))).into_response()
    }
}

// #[derive(Serialize, Debug)]
// pub struct ErrorResponse {
//     /// HTTP状态码，如 400、500
//     code: u16,
//     /// 状态描述（自动根据状态码生成），如"Not Found"
//     message: String,
//     /// 响应数据
//     detail: Option<serde_json::Value>,
// }
//
// impl ErrorResponse {
//     /// 创建错误响应
//     pub fn error(code: u16, detail: Option<serde_json::Value>) -> Self {
//         Self {
//             code,
//             message: StatusCode::from_u16(code)
//                 .ok() // 将 Result 转换为 Option
//                 .and_then(|status| status.canonical_reason()) // 获取标准描述
//                 .unwrap_or("Internal Server Error")
//                 .parse()
//                 .unwrap(), // 无效状态码时返回默认值
//             detail,
//         }
//     }
// }
//
// /// 实现Axum响应转换
// impl IntoResponse for ErrorResponse {
//     fn into_response(self) -> Response {
//         let status_code =
//             StatusCode::from_u16(self.code).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
//
//         (
//             status_code,
//             [(header::CONTENT_TYPE, "application/json; charset=utf-8")],
//             Json(self),
//         )
//             .into_response()
//     }
// }

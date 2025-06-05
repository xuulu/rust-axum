use axum::http::{header, StatusCode};
use axum::Json;
use axum::response::{IntoResponse,Response};
use serde::Serialize;

/// 统一 Json 结构体
#[derive(Serialize)]
pub struct JsonResponse {
    /// HTTP状态码，如 200、404
    code: u16,
    /// 状态描述（自动根据状态码生成），如 "OK"、"Not Found"
    message: String,
    /// 响应数据
    # [serde(skip_serializing_if = "Option::is_none")]
    data: Option<serde_json::Value>,
    /// 错误信息
    # [serde(skip_serializing_if = "Option::is_none")]
    detail: Option<serde_json::Value>,
}

impl JsonResponse {
    /// 创建成功响应
    pub fn success(data: serde_json::Value) -> Self {
        Self {
            code: 200,
            message: "OK".to_string(),
            data: Some(data),
            detail: None,
        }
    }
    /// 创建错误响应
    pub fn error(code: u16, detail: Option<serde_json::Value>) -> Self {
        Self {
            code,
            message: StatusCode::from_u16(code)
                .ok() // 将 Result 转换为 Option
                .and_then(|status| status.canonical_reason()) // 获取标准描述
                .unwrap_or("Internal Server Error")
                .parse()
                .unwrap(), // 无效状态码时返回默认值
            data: None,
            detail,
        }
    }
}

/// 实现Axum响应转换
impl IntoResponse for JsonResponse {
    fn into_response(self) -> Response {
        let status_code =
            StatusCode::from_u16(self.code).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        (
            status_code,
            [(header::CONTENT_TYPE, "application/json; charset=utf-8")],
            Json(self),
        ).into_response()
    }
}



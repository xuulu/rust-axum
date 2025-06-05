use std::net::SocketAddr;
use axum::extract::{ConnectInfo, State};
use axum::http::HeaderMap;
use serde::Deserialize;
use crate::state::AppStateArc;
use crate::response::{
    ApiResponse,
    JsonResponse,
    ValidQuery
};

#[derive(Deserialize)]
pub struct QueryParams {
    ip: Option<String>,
}

#[axum::debug_handler]
pub async fn ip(
    headers:HeaderMap,
    State(state): State<AppStateArc>,
    ValidQuery(params): ValidQuery<QueryParams>,
) -> ApiResponse {
    // 获取用户ip
    let ip = params.ip.unwrap_or_else(
        || headers.get("X-Forwarded-For")
            .and_then(|value| value.to_str().ok())
            .map(|s| s.to_string())
            .unwrap()
    );
    
    // 获取用户头部
    let user_agent = headers.get("User-Agent").and_then(|value| value.to_str().ok()).map(|s| s.to_string());
    
    let url = format!(
        "http://opendata.baidu.com/api.php?query={}&co=&resource_id=6006&t=1433920989928&ie=utf8&oe=utf-8&format=json"
        ,ip
    );

    let response = state.http.get_json(url.as_str()).await?;

    let data = serde_json::json!({
        "title": "IP查询",
        "ip": ip,
        "location": response["data"][0]["location"],
        "user_agent": user_agent
    });


    Ok(JsonResponse::success(data))
}




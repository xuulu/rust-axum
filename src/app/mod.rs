pub(crate) mod logger;

use axum::{routing, Router};
use crate::state::AppState;
use tower_http::cors::{CorsLayer, AllowOrigin, AllowHeaders, AllowMethods, ExposeHeaders};

pub async fn create_app() -> Router {
    tracing::info!("创建 App");

    // 构建全局状态
    let state = AppState::new().await;

    // 构建 CORS 中间件
    let cors = CorsLayer::new()
        .allow_origin(AllowOrigin::mirror_request()) // 允许所有来源，也可以指定具体域名
        .allow_methods(AllowMethods::mirror_request()) // 允许所有方法
        .allow_headers(AllowHeaders::mirror_request()) // 允许所有请求头
        .expose_headers(ExposeHeaders::list([])); // 可选：暴露响应头


    Router::new()
        .route("/api", routing::get(async || "Hello Rust!"))
        .nest("/api",crate::api::routers(state.clone()))
        .nest("/api/v1",crate::api_frontend::routers(state.clone()))
        .with_state(state.clone())
        .layer(cors)
}



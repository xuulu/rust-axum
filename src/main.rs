mod config;
mod app;
mod state;
mod api;
mod response;
mod models;
mod api_frontend;


#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok(); // 加载环境变量
    let _guard = crate::app::logger::init();   // 初始化日志
    config::Settings::read_env().expect("无法读取环境变量");    // 初始化配置
    let app = app::create_app();    // 初始化app
    
    // 使用 Hyper 运行我们的应用程序, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(
        format!("{}:{}",
                config::Settings::get("SERVER_HOST").unwrap_or("0.0.0.0".to_string()),
                config::Settings::get("SERVER_PORT").unwrap_or("3000".to_string()))
    ).await.unwrap();
    
    // 打印地址
    tracing::info!("正在监听 http://{}", listener.local_addr().unwrap());
    
    // 启动服务器
    axum::serve(
        listener,
        app.await.into_make_service()
    ).await.unwrap();
}

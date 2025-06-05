mod database;
mod http;
mod redis;
use std::sync::Arc;

pub use database::PgsqlPool;
pub use redis::RedisPool;

pub struct AppState {
    pub db: PgsqlPool,
    pub http: http::HttpClient,
    pub redis: RedisPool,
}


impl AppState {

    pub async fn new() -> Arc<Self> {
        let db = PgsqlPool::init().await;
        let http = http::HttpClient::init();
        let redis = RedisPool::init();
        Arc::new(Self {
            db,
            http,
            redis
        })
    }
    
    
}


pub type AppStateArc = Arc<AppState>;




use std::env;

// #[derive(Debug)]
pub struct Settings {
    pub server_host: String,
    pub server_port: Option<u16>,

    // 数据库配置
    pub db_host: String,
    pub db_port: u16,
    pub db_username: String,
    pub db_password: String,
    pub db_database: String,
}

impl Settings {
    
    /// 读取环境变量并返回 Config 结构。
    pub fn read_env() -> Result<Self, env::VarError> {
        
        tracing::info!("正在检查环境...");
        
        if env::var("DEBUG")? == "true" {
            tracing::info!("当前为开发环境");
            dotenvy::from_filename("env.development").ok();
        } else {
            tracing::info!("当前为生产环境");
            dotenvy::from_filename("env.production").ok();
        }


        Ok(Self {
            server_host: env::var("SERVER_HOST")?,
            server_port: env::var("SERVER_PORT")
                .ok()
                .and_then(|s| s.parse::<u16>().ok()),
            db_host: env::var("DB_HOST")?,
            db_port: env::var("DB_PORT")?.parse().unwrap(),
            db_username: env::var("DB_USERNAME")?,
            db_password: env::var("DB_PASSWORD")?,
            db_database: env::var("DB_DATABASE")?,
        })
    }
    
    pub fn get(key: &str) -> Option<String> {
        env::var(key).ok()
    }
    /// 检查当前是否为调试模式
    pub fn get_debug() -> bool {
        env::var("DEBUG").map(|v| v == "true").unwrap_or(false)
    }
    
}




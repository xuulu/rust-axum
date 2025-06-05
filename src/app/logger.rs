use tracing_appender::non_blocking;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};
use crate:: config::Settings;

pub fn init() -> Option<WorkerGuard> {
    // 从环境变量中读取 当前环境
    let env = Settings::get_debug();

    // 日志过滤器，支持 RUST_LOG 动态设置
    let filter_layer = EnvFilter::from_default_env();
 
    if env == true {
        let console_layer = fmt::layer()
            .with_thread_names(true)
            .with_thread_ids(true)
            .with_target(true);

        tracing_subscriber::registry()
            .with(filter_layer)
            .with(console_layer)
            .init();

        None
    } 
    else {
        let file_appender = tracing_appender::rolling::daily("logs", "server.log");
        let (non_blocking_writer, guard) = non_blocking(file_appender);

        let file_layer = fmt::layer()
            .with_writer(non_blocking_writer)
            .with_thread_names(true)
            .with_thread_ids(true)
            .with_target(true);

        tracing_subscriber::registry()
            .with(filter_layer)
            .with(file_layer)
            .init();

        Some(guard) // 返回 guard 保持日志写出
    }
    

}


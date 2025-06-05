mod error;
mod response;
mod extractors;


// JSON 响应
pub use response::JsonResponse;
// 错误处理
pub use error::AppError;



// 响应类型
pub type ApiResponse = Result<JsonResponse, AppError>;

// 自定义 查询参数和路径参数 提取器 统一错误格式
pub use extractors::ValidQuery;
pub use extractors::ValidPath;



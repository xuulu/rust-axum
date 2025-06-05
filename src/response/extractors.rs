
use axum::extract::{FromRequest, FromRequestParts};

use crate::response::AppError;

/// 为 `Query<T>` 提取器生成一个包装器，如果解析失败则返回 `ErrorResponse`。
#[derive(Debug,Clone,Default,FromRequestParts)]
#[from_request(via(axum::extract::Query), rejection(AppError))]
pub struct ValidQuery<T>(pub T);


/// 为 `Path<T>` 提取器生成一个包装器，如果解析失败则返回 `ErrorResponse`。
#[derive(Debug,Clone,Default,FromRequestParts)]
#[from_request(via(axum::extract::Path), rejection(AppError))]
pub struct ValidPath<T>(pub T);

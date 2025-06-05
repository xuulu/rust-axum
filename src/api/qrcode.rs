use crate::response::{AppError, ValidQuery};
use crate::state::AppStateArc;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct QueryParams {
    q: String,
    color: Option<String>,
    bgcolor: Option<String>,
    size: Option<String>,
}

#[axum::debug_handler]
pub async fn qrcode(
    State(state): State<AppStateArc>,
    ValidQuery(params): ValidQuery<QueryParams>,
) -> Result<impl IntoResponse,AppError> {
    let bgcolor = params.bgcolor.as_deref().unwrap_or("F4F4F4");
    let color = params.color.as_deref().unwrap_or("FF6B6B");
    let size = params.size.as_deref().unwrap_or("400");

    let url = format!(
        "https://qrcode.hlcode.cn/beautify/style/create?bgColor={}&\
        bodyType=1&content={}&down=0&embedPosition=0&embedText=&\
        embedTextColor=%23000000&embedTextSize=38&eyeInColor=%23000000&\
        eyeOutColor=%23000000&eyeType=8&eyeUseFore=1&fontFamily=0&\
        foreColor={}&foreColorImage=&foreColorTwo=&foreType=0&frameColor=&\
        gradientWay=0&level=H&logoShadow=0&logoShap=2&logoUrl=&margin=2&rotate=30\
        &size={}&format=1&qrCodeId=0",
        bgcolor, params.q, color, size,
    );
    

    // 发送请求并等待响应
    let res = state
        .http
        .get_json(&url)
        .await?;
    
    
    let res2 = res["data"].as_str().unwrap();

    // http
    let res2 = state
        .http
        .get(res2)
        .await?;
    

    // 获取图片字节流
    let bytes = res2
        .bytes()
        .await
        .map_err(|_| AppError::InternalError)?;

    // 返回图片响应
    Ok(
        (
            [(axum::http::header::CONTENT_TYPE, "image/jpeg")],
            bytes
        ).into_response()
    )
    
}

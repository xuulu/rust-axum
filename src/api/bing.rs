use axum::extract::State;
use crate::state::AppStateArc;
use crate::response::{ApiResponse,JsonResponse};

#[axum::debug_handler]
pub async fn bing(
    State(state): State<AppStateArc>
) -> ApiResponse {

    let response = state.http.get_json("http://cn.bing.com/HPImageArchive.aspx?format=js&idx=0&n=1").await?;

    let image_url = format!(
        "http://cn.bing.com{}",
        response["images"][0]["url"].as_str().unwrap()
    );

    let data = serde_json::json!({
        "title": "必应美图", 
        "url": image_url
    });
    
    
    Ok(JsonResponse::success(data))
}

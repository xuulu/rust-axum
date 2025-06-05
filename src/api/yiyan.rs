use axum::extract::State;
use crate::state::AppStateArc;
use crate::response::{ApiResponse,JsonResponse};

#[axum::debug_handler]
pub async fn yiyan(
    State(state): State<AppStateArc>
) -> ApiResponse {
    
    let data = serde_json::json!(
        {
            "title": "一言",
            "error": "正在维护中..."
        }
    );
    
    
    Ok(
        JsonResponse::error(400, Some(data))
    )
}



use crate::response::{ApiResponse, JsonResponse,AppError};
use crate::state::AppStateArc;
use axum::extract::State;

#[axum::debug_handler]
pub async fn everyday_60s(State(state): State<AppStateArc>) -> ApiResponse {
    let today = chrono::Local::now()
        .format("%Y-%m-%d")
        .to_string()
        .replace('/', "-");

    let urls = vec![
        format!("https://60s-static.viki.moe/60s/{}.json", &today),
        format!(
            "https://cdn.jsdelivr.net/gh/vikiboss/60s-static-host/static/60s/{}.json",
            &today
        ),
        format!(
            "https://raw.githubusercontent.com/vikiboss/60s-static-host/main/static/60s/{}.json",
            &today
        ),
    ];

    for url in urls {
        match state.http.get_json(&url).await {
            Ok(data) => {
                let data = serde_json::json!({
                    "title": "60s看世界",
                    "list": data["news"],
                    "image": data["image"],
                    "tip": data["tip"],
                    "date": data["date"],
                });
                return Ok(JsonResponse::success(data));
            }
            Err(_) => {}
        }
    }
    

    Err(AppError::NotFound)
}

use axum::extract::{State};
use serde::Deserialize;
use crate::state::AppStateArc;
use crate::response::{
    ApiResponse,
    JsonResponse,
    ValidQuery
};

#[derive(Deserialize)]
pub struct QueryParams {
    text: String,
}

#[axum::debug_handler]
pub async fn fanyi(
    State(state): State<AppStateArc>,
    ValidQuery(params): ValidQuery<QueryParams>,
) -> ApiResponse {
    let url = format!(
        "https://wxapp.translator.qq.com/api/translate?sourceText={}&source=auto&target=auto&platform=MQQAPP&candidateLangs=zh%7Cen&guid=wxapp_openid_1576171882_ptxba365xp"
        ,params.text
    );
    
    let response = state.http.get_json(url.as_str()).await?;
    
    let data = serde_json::json!({
        "title": "简心翻译",
        "sourceText": response["sourceText"],
        "targetText": response["targetText"],
    });


    Ok(JsonResponse::success(data))
}

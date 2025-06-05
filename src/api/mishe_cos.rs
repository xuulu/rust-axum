use std::collections::HashMap;
// mishe_cos
use crate::response::{ApiResponse, JsonResponse, ValidQuery};
use crate::state::AppStateArc;
use axum::extract::State;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct QueryParams {
    top: Option<bool>,
    new: Option<bool>,
    posts: Option<bool>,
}

#[axum::debug_handler]
pub async fn mishe_cos(
    State(state): State<AppStateArc>,
    ValidQuery(params): ValidQuery<QueryParams>,
) -> ApiResponse {
    let http = state.http.client();

    let (url, params) = if params.top == Some(true) {
        (
            "https://bbs-api.miyoushe.com/post/wapi/getImagePostTopN",
            HashMap::from([("forum_id", "49"), ("gids", "1")]),
        )
    } else if params.new == Some(true) {
        (
            "https://bbs-api.miyoushe.com/post/wapi/getForumPostList",
            HashMap::from([
                ("forum_id", "49"),
                ("gids", "2"),
                ("is_good", "false"),
                ("is_hot", "false"),
                ("page_size", "20"),
                ("sort_type", "2"),
            ]),
        )
    } else if params.posts == Some(true) {
        (
            "https://bbs-api.mihoyo.com/post/api/feeds/posts?fresh_action=1&gids=2&last_id=",
            HashMap::from([("fresh_action", "1"), ("gids", "2"), ("last_id", "")]),
        )
    } else {
        return Ok(JsonResponse::error(
            400,
            Some(serde_json::json!("请检查参数是否正确!")),
        ));
    };
    

    let response = http
        .get(url)
        .query(&params)
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;

    let mut list = vec![];
    if let Some(items) = response["data"]["list"].as_array() {
        for item in items {
            let mut images = vec![];
            if let Some(imgs) = item["image_list"].as_array() {
                for img in imgs {
                    if let Some(url) = img.get("url") {
                        images.push(url);
                    }
                }
            }

            list.push(serde_json::json!({
                "title": item["post"]["subject"],
                "coser": item["user"]["nickname"],
                "images": images
            }));
        }
    }

    let data = serde_json::json!({
        "title": "米社cos",
        "list": list,
    });

    Ok(JsonResponse::success(data))
}

use axum::extract::State;
use crate::state::AppStateArc;
use crate::response::{ApiResponse,JsonResponse};

#[axum::debug_handler]
pub async fn ys_kaci(
    State(state): State<AppStateArc>
) -> ApiResponse {
    // # 米游社表情包
    // https://bbs-api-static.miyoushe.com/misc/api/emoticon_set
    // 获取记录
    // https://api.lelaer.com/ys/getPlayerRecord.php?uid=
    // 获取分区信息(活动等)
    // URL: https://bbs-api.mihoyo.com/apihub/api/home/new?gids=2

    let url = "https://api-takumi.mihoyo.com/common/blackboard/ys_obc/v1/gacha_pool?app_sn=ys_obc";

    let response = state.http.get_json(url).await?;

    let mut list = vec![];

    for i in response["data"]["list"].as_array().unwrap() {
        list.push(
            serde_json::json!({
                "id": i["id"],
                "title": i["title"],
                "content": i["content_before_act"],
                "icon": i["pool"][0]["icon"],
            })
        );
    }


    let data = serde_json::json!({
        "title": "原神卡池",
        "list": list,
        "start_time": response["data"]["list"][0]["start_time"],
        "end_time": response["data"]["list"][0]["end_time"],
    });


    Ok(JsonResponse::success(data))
}

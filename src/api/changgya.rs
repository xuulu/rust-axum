use axum::extract::State;
use scraper::{Html,Selector};
use crate::state::AppStateArc;
use crate::response::{ApiResponse,JsonResponse};
use percent_encoding::percent_decode_str ;

pub async fn changgya(
    State(state): State<AppStateArc>
) -> ApiResponse {

    let response = state.http
        .get("https://m.api.singduck.cn/user-piece/6oGNUeM16kBuRmPct?userId=2003919010")
        .await?
        .text()
        .await?;

    // 解析HTML文档
    let document = Html::parse_document(&response);

    let json_data: serde_json::Value = document
        .select(&Selector::parse("#__NEXT_DATA__").unwrap())
        .next()
        .ok_or_else(|| anyhow::anyhow!("未找到脚本标签"))?
        .text()
        .collect::<String>()
        .parse()
        .map_err(|_| anyhow::anyhow!("JSON解析失败"))?;
    
    // 提取嵌套数据
    let pieces = json_data["props"]["pageProps"]["pieces"]
        .as_array()
        .ok_or_else(|| anyhow::anyhow!("无效的pieces格式"))?;

    let one = pieces.first().ok_or_else(|| anyhow::anyhow!("空数据列表"))?;
    
    
    // 解码URL并处理数据
    let data = serde_json::json!({
        "title": "随机唱鸭",
        "artist": one["artist"].as_str().unwrap_or_default(),
        "avatarUrl": one["avatarUrl"].as_str().unwrap_or_default(),
        "lyric": one["lyric"].as_str().unwrap_or_default(),
        "audioUrl": percent_decode_str(one["audioUrl"].as_str().unwrap_or_default()).decode_utf8().unwrap()
    });


    Ok(JsonResponse::success(data))
}

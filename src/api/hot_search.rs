use crate::response::{ApiResponse, AppError, JsonResponse, ValidQuery};
use crate::state::AppStateArc;
use axum::extract::State;
use redis::Commands;
use scraper::{Html, Selector};
use sea_orm::Iden;
use serde::Deserialize;
use tracing::info;

#[derive(Deserialize)]
pub struct QueryParams {
    q: String,
}

#[axum::debug_handler]
pub async fn hot_search(
    State(state): State<AppStateArc>,
    ValidQuery(params): ValidQuery<QueryParams>,
) -> ApiResponse {
    let http = state.http.client();
    let q = params.q.to_lowercase();
    let mut list = vec![];

    match q.as_str() {

        "抖音" | "douyin" => {
            let res = http
                .get("https://www.douyin.com/aweme/v1/web/hot/search/list")
                .header("referer", "https://www.douyin.com/hot")
                .send()
                .await?
                .json::<serde_json::Value>()
                .await?;

            // 安全获取 word_list 数组
            if let Some(word_list) = res
                .get("data")
                .and_then(|data| data.get("word_list"))
                .and_then(|list| list.as_array())
            {
                for (index, item) in word_list.iter().enumerate() {
                    // 安全提取并解码 word 字段
                    let word = item
                        .get("word")
                        .and_then(|v| v.as_str())
                        .ok_or(AppError::InternalError)?;

                    // 安全提取 hot_value 并格式化热度值
                    let hot_value = item.get("hot_value")
                            .and_then(|v| v.as_u64())
                            .and_then(|n| u64::try_from(n).ok())
                            .ok_or(AppError::InternalError)?;


                    let formatted_hot = round_to_str(hot_value);

                    // 构建结果条目
                    list.push(format!("{}、{} (热度:{})", index + 1, word, formatted_hot));
                }
            } else {
                return Err(AppError::InternalError);
            }
        }
        "快手" | "kuaishou" => {
            let res = http
                .get("https://www.kuaishou.com/brilliant")
                .header("host", "www.kuaishou.com")
                .send()
                .await?
                .text()
                .await?;
            // 解析 HTML
            let document = Html::parse_document(&res);
            // 创建 CSS 选择器定位 script 标签
            let selector = Selector::parse("script").unwrap();

            // 查找包含 __APOLLO_STATE__ 的 script 标签
            let script_text = document
                .select(&selector)
                .find_map(|script| {
                    let text = script.inner_html();
                    text.contains("__APOLLO_STATE__").then_some(text)
                })
                .unwrap();

            // 提取纯净 JSON 数据
            let json_str = script_text
                .split_once("window.__APOLLO_STATE__=") // 第一次分割
                .and_then(|(_, after)| after.rsplit_once(";(function()")) // 第二次分割
                .map(|(left, _)| left.trim()) // 去除空白
                .map(|s| s.trim_end_matches(',')) // 去除结尾的逗号
                .unwrap_or(""); // 处理未匹配的情况

            // 解析JSON 数据
            let apollo_data: serde_json::Value = serde_json::from_str(json_str)?;
            // 获取 需要的 数据
            let ks_items = &apollo_data["defaultClient"]["$ROOT_QUERY.visionHotRank({\"page\":\"brilliant\"})"]
                ["items"];

            for (index, item) in ks_items.as_array().unwrap().iter().enumerate() {
                let item_id = &item["id"].as_str().unwrap();
                let hot_item = apollo_data["defaultClient"].get(item_id).unwrap();

                let name = hot_item["name"].as_str().unwrap_or("未知");
                let hot_value = hot_item["hotValue"].as_str().unwrap_or("置顶"); // 明确类型

                list.push(format!("{}、{} (热度:{})", index + 1, name, hot_value));
            }
        }
        "头条" | "toutiao" => {
            let res = state
                .http
                .get_json("https://is-lq.snssdk.com/api/suggest_words/?business_id=10016")
                .await?;

            if let Some(words_array) = res
                .get("data")
                .and_then(|d| d.get(0))
                .and_then(|words| words.get("words"))
            {
                for (index, word) in words_array.as_array().unwrap().iter().enumerate() {
                    let word_str = word.get("word").and_then(|v| v.as_str()).ok_or(AppError::InternalError)?;

                    let hot_u32 = word.get("params")
                        .and_then(|p| p.get("fake_click_cnt"))
                        .and_then(|v| v.as_u64())
                        .and_then(|n| u64::try_from(n).ok())
                        .ok_or(AppError::InternalError)?;
                    let hot_str = round_to_str(hot_u32);

                    list.push(format!("{}、{} (热度:{})", index + 1, word_str,  hot_str));
                }
            } else {
                return Err(AppError::InternalError);
            }
        }
        "百度" | "baidu" => {
            let res = state.http.get_json("https://top.baidu.com/api/board?platform=pc&tab=realtime").await?;

            for (index, item) in res["data"]["cards"][0]["content"].as_array().unwrap().iter().enumerate() {
                let title = item["query"].as_str().unwrap_or("未知");

                let hot_value = item["hotScore"].as_str().and_then(|s| s.parse::<u64>().ok()).unwrap_or(0);
                let hot = round_to_str(hot_value);

                list.push(format!("{}、{} (热度:{})", index + 1, title, hot));
            }
            

        }
        "微博" | "weibo" => {
            let res = state.http.get_json("https://weibo.com/ajax/side/hotSearch").await?;

            for (index, item) in res["data"]["realtime"].as_array().unwrap().iter().enumerate() {
                let title = item["word"].as_str().unwrap_or("未知");
                
                let hot_value = item.get("num")
                    .and_then(|v| v.as_u64().or_else(|| v.as_str().and_then(|s| s.parse::<u64>().ok())))
                    .unwrap_or(0);
                let hot = round_to_str(hot_value);

                list.push(format!("{}、{} (热度:{})", index + 1, title, hot));
            }
        }
        "b站" | "B站" | "哔哩哔哩" | "bilibili" => {
            let res = state.http.get_json("https://app.bilibili.com/x/v2/search/trending/ranking?limit=50").await?;

            for (index, item) in res["data"]["list"].as_array().unwrap().iter().enumerate() {
                let title = &item["keyword"].as_str().unwrap_or("未知");
                
                let hot_value = item.get("hot_id")
                    .and_then(|v| v.as_u64().or_else(|| v.as_str().and_then(|s| s.parse::<u64>().ok())))
                    .unwrap_or(0);
                let hot = round_to_str(hot_value);

                list.push(format!("{}、{} (热度:{})", index + 1, title, hot));
            }
        }
        _ => return Err(AppError::InternalError),
    }

    Ok(JsonResponse::success(serde_json::json!(
        {
            "title": "热搜榜单",
            "name": q,
            "list": list,
        }
    )))
}

// 将数字四舍五入到最接近的 10000、100、10、10 或 unit 的函数
fn round_to_str(value: u64) -> String {
    if value >= 10000 {
        // 四舍五入到一万
        format!("{} 万", (value as f64 / 10000.0).round())
    } else if value >= 1000 {
        // 四舍五入到一千
        format!("{} 千", (value as f64 / 1000.0).round())
    } else {
        value.to_string()
    }
}

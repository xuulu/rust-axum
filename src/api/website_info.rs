use axum::extract::{State};
use scraper::{Html, Selector};
use sea_orm::Iden;
use serde::Deserialize;
use crate::state::AppStateArc;
use crate::response::{
    ApiResponse,
    JsonResponse,
    ValidQuery
};

#[derive(Deserialize)]
pub struct QueryParams {
    url: String,
}

#[axum::debug_handler]
pub async fn website_info(
    State(state): State<AppStateArc>,
    ValidQuery(params): ValidQuery<QueryParams>,
) -> ApiResponse {
    
    // 测速开始
    let start = std::time::Instant::now();
    
    let res = state.http.get(&params.url.as_str()).await?;
    
    // 测速结束
    let process_time = start.elapsed().as_millis();
    let process_time = format!("{}ms", process_time);
    
    
    // 获取状态码
    let code = res.status().as_u16();
    
    if !res.status().is_success() {
        let data = serde_json::json!({
            "title": "网站信息查询",
            "website_code":&code,
        });
        return Ok(JsonResponse::success(data));
    }
    
    // 解析 HTML
    let document = Html::parse_document(&res.text().await?);
    
    // 获取 title 元素
    let title_selector = Selector::parse("title").unwrap();
    let title = document.select(&title_selector)
        .next().map(|el| el.text().collect::<Vec<_>>().join(" ").trim().to_string());
    
    // 获取 icon 元素
    let icon_selector = Selector::parse("link[rel]").unwrap();
    let mut icon_href = None;
    for el in document.select(&icon_selector) {
        if let Some(rel) = el.value().attr("rel") {
            if rel.contains("icon") {
                if let Some(href) = el.value().attr("href") {
                    icon_href = Some(href.to_string());
                    break;
                }
            }
        }
    };
    // 检查是否为相对路径
    let icon = icon_href.and_then(|href| {
        if href.starts_with("http") {
            Some(href)
        } else {
            let base_url = params.url.split('/').next().unwrap();
            Some(format!("{}{}", base_url, href))
        }
    });
    
    // 获取 description
    let description_selector = Selector::parse(r#"meta[name='description']"#).unwrap();
    let description = document.select(&description_selector)
        .next().and_then(|el| el.value().attr("content"));
    
    // 获取 keywords
    let keywords_selector = Selector::parse(r#"meta[name='keywords']"#).unwrap();
    let keywords = document.select(&keywords_selector)
        .next().and_then(|el| el.value().attr("content"));
    
    let data = serde_json::json!({
        "title": "网站信息查询",
        "website_code":&code,
        "website_title": title,
        "website_icon": icon,
        "website_description": description,
        "website_keywords": keywords,
        "process_time": process_time,
    });


    Ok(JsonResponse::success(data))
}

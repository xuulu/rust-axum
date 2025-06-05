// weather
use axum::extract::{State};
use scraper::{Html, Selector};
use serde::Deserialize;
use serde_json::Value;
use crate::state::AppStateArc;
use crate::response::{ApiResponse, AppError, JsonResponse, ValidQuery};
use std::error::Error;

#[derive(Deserialize)]
pub struct QueryParams {
    q: String,
}

#[axum::debug_handler]
pub async fn weather(
    State(state): State<AppStateArc>,
    ValidQuery(params): ValidQuery<QueryParams>,
) -> ApiResponse {

    fn remove_suffix(region: &str) -> &str {
        let suffixes = ["市", "县", "区", "自治州", "旗", "自治县", "市辖区", "特区", "林区"];

        for &suffix in &suffixes {
            if region.ends_with(suffix) {
                return &region[..region.len() - suffix.len()];
            }
        }

        region
    }
    let q = remove_suffix(params.q.as_str());
    
    // 从缓存中获取 地区id
    let weather = state.redis.get_or_set_json("weathersss",  None, || async {
        let res = state.http.get_json("https://weather.cma.cn/api/map/weather/1").await.unwrap();

        let mut weathers = vec![];
        if let Some(cities) = res.get("data").and_then(|d| d.get("city")) {
            for city in cities.as_array().unwrap_or(&vec![]) {
                if let (Some(name), Some(code)) = (city.get(1), city.get(0)) {
                    let mut obj = serde_json::Map::new();
                    if let Some(name_str) = name.as_str() {
                        obj.insert(name_str.to_string(), code.clone());
                    }
                    weathers.push(serde_json::Value::Object(obj));
                }
            }
        }
        Some(weathers)
    }).await?.ok_or(AppError::InternalError)?;

    let id = weather.iter().find_map(|city| {
        city.get(q)
            .and_then(|city_name| city_name.as_str())
            .map(|city_id| {
                // // 使用 percent encoding 避免特殊字符破坏 URL 结构
                // let encoded_city_id = percent_encoding::utf8_percent_encode(
                //     city_id,
                //     percent_encoding::NON_ALPHANUMERIC,
                // );
                city_id
            })
    }).unwrap();
    
    let url = format!("https://weather.cma.cn/web/weather/{}.html", id);
    let res = state.http.get(url.as_str()).await?.text().await?;

    let list = parse_weather(&res).unwrap();


    let data = serde_json::json!({
        "title": "天气查询",
        "q":params.q,
        "id":id,
        "list": list,
    });


    Ok(JsonResponse::success(data))
}


fn parse_weather(html: &str) -> Result<Vec<serde_json::Value>, Box<dyn Error>> {
    let fragment = Html::parse_fragment(html);

    let day_selector = Selector::parse("div.day").unwrap();
    let item_selector = Selector::parse("div.day-item").unwrap();
    let bar_selector = Selector::parse("div.bar").unwrap();

    let mut days = Vec::new();

    for day_div in fragment.select(&day_selector) {
        let items: Vec<_> = day_div.select(&item_selector).collect();

        // 提取日期
        let date = items.get(0).map(|n| n.text().collect::<String>().trim().to_string());

        // 白天天气
        let weather_day = items.get(2).map(|n| n.text().collect::<String>().trim().to_string());

        // 白天风向
        let wind_day = items.get(3).map(|n| n.text().collect::<String>().trim().to_string());

        // 白天风力
        let wind_level_day = items.get(4).map(|n| n.text().collect::<String>().trim().to_string());

        // 温度范围
        let temp_div = day_div.select(&bar_selector).next();
        let high_temp = temp_div
            .as_ref()
            .and_then(|b| b.select(&Selector::parse("div.high").unwrap()).next())
            .map(|h| h.text().collect::<String>().trim().to_string())
            .unwrap_or_default();

        let low_temp = temp_div
            .as_ref()
            .and_then(|b| b.select(&Selector::parse("div.low").unwrap()).next())
            .map(|l| l.text().collect::<String>().trim().to_string())
            .unwrap_or_default();

        // 夜晚天气
        let weather_night = items.get(6).map(|n| n.text().collect::<String>().trim().to_string());
        
        // 夜晚风力
        let wind_level_night = items.get(8).map(|n| n.text().collect::<String>().trim().to_string());

        days.push(serde_json::json!({
            "date": date.unwrap_or_default().split_whitespace().collect::<Vec<_>>(),
            "weather_day": weather_day.unwrap_or_default(),
            "wind_day": wind_day.unwrap_or_default(),
            "wind_level_day": wind_level_day.unwrap_or_default(),
            "high_temp":high_temp,
            "low_temp":low_temp,
            "weather_night": weather_night.unwrap_or_default(),
            "wind_level_night": wind_level_night.unwrap_or_default(),
        }));
    }

    Ok(days)
}





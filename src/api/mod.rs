use crate::state::AppStateArc;
use axum::routing::get;
use axum::ServiceExt;

pub mod bing;
mod fanyi;
mod changgya;
mod cos;
mod yiyan;
mod everyday_60s;
mod hot_search;
mod qrcode;
mod ys_kaci;
mod website_info;
mod ip;
mod encryption;
mod mishe_cos;
mod weather;

pub fn routers(state: AppStateArc) -> axum::Router<AppStateArc> {
    axum::Router::new()
        .route("/bing", get(bing::bing))
        .route("/fanyi", get(fanyi::fanyi))
        .route("/changya", get(changgya::changgya))
        .route("/cos", get(cos::cos))
        .route("/yiyan", get(yiyan::yiyan))
        .route("/everyday_60s", get(everyday_60s::everyday_60s))
        .route("/hot_search", get(hot_search::hot_search))
        .route("/qrcode", get(qrcode::qrcode))
        .route("/ys_kaci", get(ys_kaci::ys_kaci))
        .route("/website_info", get(website_info::website_info))
        .route("/ip", get(ip::ip))
        .route("/encryption", get(encryption::encryption))
        .route("/mishe_cos", get(mishe_cos::mishe_cos))
        .route("/weather", get(weather::weather))
        .with_state(state)
}





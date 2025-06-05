mod api_list;
mod api_link;

use crate::state::AppStateArc;
use axum::routing::{get, post};

use api_list::*;
use api_link::*;

pub fn routers(state: AppStateArc) -> axum::Router<AppStateArc> {
    axum::Router::new()
        .route("/api-list", get(get_all_api_lists))
        .route("/api-list/{id}", get(get_api_list_by_id))
        .route("/api-list/search", get(get_search_api_lists))
        

        .route("/friend-links", get(get_all_links))
        .with_state(state)
}


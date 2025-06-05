use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use sea_orm::entity::prelude::*;
use sea_orm::{IntoActiveModel, QueryOrder, QuerySelect};
use serde::Serialize;
use crate::models::friend_links;
use crate::models::prelude::FriendLinks;
use crate::state::AppStateArc;


pub async fn get_all_links(
    State(state): State<AppStateArc>,
) -> impl IntoResponse {
    let links = FriendLinks::find()
        .filter(friend_links::Column::IsApproved.eq(true))
        .order_by_asc(friend_links::Column::Id)
        .all(state.db.get_conn())
        .await
        .unwrap();


    Json(links)
}
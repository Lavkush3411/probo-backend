use axum::{extract::State, response::IntoResponse, routing::post, Json, Router};

use crate::{db::db::DB, state::AppState};

pub fn user_router()->Router<AppState>{
    Router::new().route("/", post(create_user))

}

pub async fn create_user(State(db):State<DB>)->impl IntoResponse{
    Json("User Created")
}
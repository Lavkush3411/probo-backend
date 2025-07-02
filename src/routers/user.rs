use axum::{
    Json, Router,
    extract::{Path, State},
    response::IntoResponse,
    routing::get,
};
use serde_json::json;

use crate::{db::db::DB, state::AppState};

pub fn user_router() -> Router<AppState> {
    Router::new()
        .route("/users", get(get_users))
        .route("/{user_id}", get(get_user_by_id))
}

pub async fn get_user_by_id(
    State(db): State<DB>,
    Path(user_id): Path<String>,
) -> impl IntoResponse {
    match db.user.get_by_id(&user_id).await {
        Ok(user) => Json(user).into_response(),
        Err(_) => {
            Json(json!({"message":"Some Error when retrieving user details"})).into_response()
        }
    }
}

pub async fn get_users(State(db): State<DB>) -> impl IntoResponse {
    let users = db.user.get_users().await;

    match users {
        Ok(user) => Json(user).into_response(),
        Err(_) => Json("Some Error Occurred while adding user to db").into_response(),
    }
}

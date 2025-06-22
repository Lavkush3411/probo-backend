use axum::{
    Json, Router,
    extract::State,
    response::IntoResponse,
    routing::{get},
};

use crate::{
    db::{db::DB},
    state::AppState,
};

pub fn user_router() -> Router<AppState> {
    Router::new()
        .route("/users", get(get_users))
}


pub async fn get_users(State(db): State<DB>) -> impl IntoResponse {
    let users = db.user.get_users().await;

    match users {
        Ok(user) => Json(user).into_response(),
        Err(_) => Json("Some Error Occurred while adding user to db").into_response(),
    }
}

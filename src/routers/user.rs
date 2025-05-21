use axum::{
    Json, Router,
    extract::State,
    response::IntoResponse,
    routing::{get, post},
};

use crate::{
    db::{db::DB, user::UserModel},
    state::AppState,
};

pub fn user_router() -> Router<AppState> {
    Router::new()
        .route("/", post(create_user))
        .route("/users", get(get_users))
}
#[axum::debug_handler]
pub async fn create_user(State(db): State<DB>, Json(user): Json<UserModel>) -> impl IntoResponse {
    let new_user = db.user.create(&user).await;
    match new_user {
        Ok(user) => Json(user).into_response(),
        Err(_) => Json("Some Error Occurred while adding user to db").into_response(),
    }
}

pub async fn get_users(State(db): State<DB>) -> impl IntoResponse {
    let users = db.user.get_users().await;

    match users {
        Ok(user) => Json(user).into_response(),
        Err(_) => Json("Some Error Occurred while adding user to db").into_response(),
    }
}

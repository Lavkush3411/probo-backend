use axum::{response::IntoResponse, routing::post, Json, Router};

use crate::state::AppState;


pub fn opinion_router()->Router<AppState>{
 Router::new().route("/", post(create_opinion))
}

pub async fn create_opinion()->impl IntoResponse{
    Json("Ok").into_response()
}

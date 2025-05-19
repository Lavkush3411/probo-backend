use axum::{Json, Router, extract::State, response::IntoResponse, routing::post};
use serde_json::json;

use crate::{
    db::{db::DB, opinion::OpinionModel},
    state::AppState,
};

pub fn opinion_router() -> Router<AppState> {
    Router::new().route("/", post(create_opinion))
}

pub async fn create_opinion(
    State(db): State<DB>,
    Json(opinion): Json<OpinionModel>,
) -> impl IntoResponse {
    let opinion = db.opinion.insert(opinion.question).await;

    match opinion {
        Result::Ok(opinion) => Json(json!(opinion)).into_response(),
        Result::Err(err) => {
            println!("{:?}", err);
            Json("Error Occurred while creating opinion").into_response()
        }
    }
}

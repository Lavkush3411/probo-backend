use axum::{
    Json, Router,
    extract::State,
    response::IntoResponse,
    routing::{get, post},
};
use serde_json::json;

use crate::{
    db::{db::DB, opinion::OpinionModel},
    state::AppState,
};

pub fn opinion_router() -> Router<AppState> {
    Router::new()
        .route("/", post(create_opinion))
        .route("/opinions", get(get_opinions))
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

pub async fn get_opinions(State(db): State<DB>) -> impl IntoResponse {
    let opinions = db.opinion.find_many().await;
    match opinions {
        Ok(opinions) => Json(opinions).into_response(),
        Err(_) => Json("Error occurred while fetching opinions").into_response(),
    }
}

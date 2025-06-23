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
        .route("/markets", get(get_opinions))
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

pub async fn get_opinions(State(app_state): State<AppState>) -> impl IntoResponse {
    let db =app_state.db;
    let opinions = db.opinion.find_many().await;
    let opinions= match opinions {
        Ok(opinions) => opinions,
        Err(_) => return  Json("Error occurred while fetching opinions").into_response(),
    };
    let order_book= app_state.order_book.read().await;

    for op in opinions.iter(){
        
    }
    

    return  Json({}).into_response();
}

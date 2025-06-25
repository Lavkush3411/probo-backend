use axum::{
    Json, Router,
    extract::State,
    response::IntoResponse,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::prelude::FromRow;

use crate::{
    db::opinion::OpinionModel,
    state::{AppState, OrderBook},
};

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct MarketModel {
    pub id: String,
    pub question: String,
    pub description: Option<String>,
    pub result: Option<bool>,
    pub yes_price: i32,
    pub no_price: i32,
}

pub fn opinion_router() -> Router<AppState> {
    Router::new()
        .route("/", post(create_opinion))
        .route("/markets", get(get_opinions))
}

pub async fn create_opinion(
    State(app_state): State<AppState>,
    Json(opinion): Json<OpinionModel>,
) -> impl IntoResponse {
    let db = app_state.db;
    let opinion = db.opinion.insert(opinion.question).await;

    let opinion = match opinion {
        Result::Ok(opinion) => opinion,
        Result::Err(err) => {
            println!("{:?}", err);
            return Json("Error Occurred while creating opinion").into_response();
        }
    };

    let mut order_book = app_state.order_book.write().await;
    order_book.insert(opinion.id.clone().unwrap(), OrderBook::empty());

    Json(json!(opinion)).into_response()
}

pub async fn get_opinions(State(app_state): State<AppState>) -> impl IntoResponse {
    let db = app_state.db;
    let opinions = db.opinion.find_many().await;
    let opinions = match opinions {
        Ok(opinions) => opinions,
        Err(_) => return Json("Error occurred while fetching opinions").into_response(),
    };
    let order_book = app_state.order_book.read().await;
    let mut markets: Vec<MarketModel> = vec![];
    for op in opinions.iter() {
        let id = match &op.id {
            Some(id) => id,
            None => continue,
        };
        let orders = match order_book.get(id) {
            Some(orders) => orders,
            None => continue,
        };
        let yes_price = orders.favour.get(0).map(|o| o.price).unwrap_or(0) as i32;
        let no_price = orders.against.get(0).map(|o| o.price).unwrap_or(0) as i32;
        let market = MarketModel {
            id: id.clone(),
            question: op.question.clone(),
            description: op.description.clone(),
            result: op.result,
            yes_price,
            no_price,
        };
        markets.push(market);
    }

    return Json(markets).into_response();
}

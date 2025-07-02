use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::prelude::FromRow;

use crate::{
    db::{db::DB, opinion::OpinionModel, trade::TradeModel},
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
        .route("/{opinion_id}", get(get_opinion_by_id))
        .route("/depth/{opinion_id}", get(get_market_depth_by_id))
        .route("/result/{opinion_id)}", post(declare_result))
}

#[derive(Serialize, Deserialize)]
struct DeclareResultDto {
    result: bool,
}

async fn release_all_balances(db: &DB, orders: &OrderBook) -> bool {
    let mut tx = match db.pool.begin().await {
        Ok(tx) => tx,
        Err(_) => return false,
    };

    for order in orders.against.iter() {
        if db
            .user
            .release_balance(&mut *tx, &order.user_id, order.price)
            .await
            .is_err()
        {
            return false;
        }
    }

    for order in orders.favour.iter() {
        if db
            .user
            .release_balance(&mut *tx, &order.user_id, order.price)
            .await
            .is_err()
        {
            return false;
        };
    }
    if tx.commit().await.is_err() {
        return false;
    };
    return true;
}

async fn distribute_prize(
    db: &DB,
    trades: Vec<TradeModel>,
    opinion_id: &String,
    result: bool,
) -> bool {
    let mut tx = match db.pool.begin().await {
        Ok(tx) => tx,
        Err(_) => return false,
    };

    // if result is true then add amount to favour users and deduct from against

    if result {
        // favour user is winner
        for trade in trades.iter() {
            if db
                .user
                .update_balance_post_result(
                    &mut *tx,
                    &trade.favour_user_id,
                    &trade.against_user_id,
                    trade.favour_price,
                    trade.against_price,
                    trade.favour_price + trade.against_price,
                )
                .await
                .is_err()
            {
                return false;
            };
        }
    } else {
        // against user is winner
        for trade in trades.iter() {
            if db
                .user
                .update_balance_post_result(
                    &mut *tx,
                    &trade.against_user_id,
                    &trade.favour_user_id,
                    trade.against_price,
                    trade.favour_price,
                    trade.favour_price + trade.against_price,
                )
                .await
                .is_err()
            {
                return false;
            };
        }
    }

    if db
        .opinion
        .update_result(&mut *tx, opinion_id, result)
        .await
        .is_err()
    {
        return false;
    }

    return true;
}

async fn declare_result(
    State(state): State<AppState>,
    Path(opinion_id): Path<String>,
    Json(declare_result_dto): Json<DeclareResultDto>,
) -> impl IntoResponse {
    //release the hold money from state
    let db = state.db;

    if let Some(orders) = state.order_book.read().await.get(&opinion_id).cloned() {
        if release_all_balances(&db, &orders).await {
            state.order_book.write().await.remove(&opinion_id);
        }
    }

    // distribute the prize
    let trades = match db.trade.get_trades_by_opinion_id(&opinion_id).await {
        Ok(trades) => trades,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"message":"Error while getting trades list for opinion"})),
            )
                .into_response();
        }
    };

    let status = distribute_prize(&db, trades, &opinion_id, declare_result_dto.result).await;
    if !status {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"message":"Error while prize distribution"})),
        )
            .into_response();
    }

    Json(json!({"message":"Successfully distributed the prize"})).into_response()
}

async fn get_market_depth_by_id(
    State(state): State<AppState>,
    Path(opinion_id): Path<String>,
) -> impl IntoResponse {
    let order_book = state.order_book.read().await;
    let orders = match order_book.get(&opinion_id) {
        Some(orders) => orders,
        None => {
            return (
                StatusCode::NOT_FOUND,
                Json(json!({"message":"Order Book not found"})),
            )
                .into_response();
        }
    };
    return Json(json!({"order_book":orders.clone()})).into_response();
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

        // highest price in NO will be the best price (best Yes = 1000 - highest NO = Lowest Yes) for yes to buy and visa versa
        let yes_price = orders.against.last().map(|o| 1000 - o.price).unwrap_or(0) as i32;
        let no_price = orders.favour.last().map(|o| 1000 - o.price).unwrap_or(0) as i32;
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

pub async fn get_opinion_by_id(
    Path(opinion_id): Path<String>,
    State(app_state): State<AppState>,
) -> impl IntoResponse {
    let db = app_state.db;
    let opinion = match db.opinion.find_one(opinion_id).await {
        Ok(op) => op,
        Err(_) => return Json(json!({"message":"Error Occurred"})).into_response(),
    };

    Json(opinion).into_response()
}

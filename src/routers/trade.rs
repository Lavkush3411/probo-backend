use axum::{
    Extension, Json, Router,
    extract::{Query, State},
    middleware,
    response::IntoResponse,
    routing::get,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio::sync::RwLockReadGuard;

use crate::{
    db::user::UserModel,
    middlewares::auth::auth_middleware,
    state::{AppState, OrderBook, Side},
};

pub fn trade_router() -> Router<AppState> {
    Router::new()
        .route("/trades", get(get_all_trades))
        .layer(middleware::from_fn(auth_middleware))
}

#[derive(Deserialize)]
pub struct GetTradesQuery {
    active: Option<bool>,
}

pub async fn get_all_trades(
    State(state): State<AppState>,
    Extension(user): Extension<UserModel>,
    Query(query): Query<GetTradesQuery>,
) -> impl IntoResponse {
    let trades = state
        .db
        .trade
        .get_trades(user.id.clone(), query.active)
        .await;

    match trades {
        Ok(trades) => match query.active {
            Some(true) => {
                let order_book = state.order_book.read().await;
                let orders = get_orders_by_user(&order_book, &user.id.unwrap());
                Json(json!({
                    "unfulfilled": orders,
                    "fulfilled": trades
                }))
                .into_response()
            }
            _ => Json(json!({
                "unfulfilled": [],
                "fulfilled": trades
            }))
            .into_response(),
        },
        Err(_) => Json("Some Error occurred").into_response(),
    }
}

use std::collections::HashMap;
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderWithOpinion {
    pub opinion_id: String,
    pub user_id: String,
    pub quantity: u16,
    pub price: u16,
    pub side: Side,
}

pub fn get_orders_by_user(
    order_book: &RwLockReadGuard<'_, HashMap<String, OrderBook>>,
    user_id: &str,
) -> Vec<OrderWithOpinion> {
    let mut user_orders = Vec::new();

    for (opinion_id, book) in order_book.iter() {
        for order in &book.favour {
            if order.user_id == user_id {
                user_orders.push((opinion_id.clone(), order.clone()));
            }
        }
        for order in &book.against {
            if order.user_id == user_id {
                user_orders.push((opinion_id.clone(), order.clone()));
            }
        }
    }

    user_orders
        .into_iter()
        .map(|(opinion_id, order)| OrderWithOpinion {
            opinion_id: opinion_id,
            user_id: order.user_id,
            quantity: order.quantity,
            price: order.price,
            side: order.side,
        })
        .collect()
}

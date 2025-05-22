use axum::{
    Json, Router,
    extract::{Path, State},
    response::IntoResponse,
    routing::{get, post},
};
use serde_json::json;

use crate::{
    db::trade::TradeModel,
    state::{AppState, Order, OrderBook, Side},
};

pub fn order_router() -> Router<AppState> {
    Router::new()
        .route("/{opinion_id}", post(handle_order))
        .route("/order_book", get(get_order_book))
}

async fn get_order_book(State(state): State<AppState>) -> impl IntoResponse {
    let order_book = state.order_book.read().await;
    return Json(json!({"order_book":order_book.clone()})).into_response();
}

#[axum::debug_handler]
async fn handle_order(
    State(state): State<AppState>,
    Path(opinion_id): Path<String>,
    Json(order): Json<Order>,
) -> impl IntoResponse {
    let mut order_book = state.order_book.write().await;
    let db = state.db;

    let remaining = match order_book.get_mut(&opinion_id) {
        Some(book_orders) => match order.side {
            Side::Against => {
                // we will have to find a matching order price against current price to create a trade
                // if someone willing to buy NO at 80 cents then someone has to buy YES at least at 20 cents or more
                let mut trades: Vec<TradeModel> = vec![];
                let match_price = 1000 - order.price;
                let mut quantity = order.quantity;
                let mut remove = 0;
                for book_order in book_orders.favour.iter_mut().rev() {
                    //this will be sorted ascending order and we will check from highest (last) so user can get at best price
                    // and if the highest price is less than the match price then there is no possibility of a match
                    // example someone is willing to buy NO at 80 then the match price will be 20
                    // then if 20 is greater then highest available price in book order then there is no possibility of math
                    // order will have to wait in order book as cant fulfilled immediately
                    if match_price > book_order.price {
                        break;
                    }
                    if book_order.quantity > quantity {
                        let trade = TradeModel {
                            id: None,
                            quantity,
                            opinion_id: opinion_id.clone(),
                            favour_user_id: book_order.user_id.clone(),
                            favour_price: book_order.price,
                            against_price: 1000 - book_order.price,
                            against_user_id: order.user_id.clone(),
                        };
                        trades.push(trade);
                        book_order.quantity -= quantity;
                        break;
                    } else {
                        let trade = TradeModel {
                            id: None,
                            quantity: book_order.quantity,
                            opinion_id: opinion_id.clone(),
                            favour_user_id: book_order.user_id.clone(),
                            favour_price: book_order.price,
                            against_price: 1000 - order.price,
                            against_user_id: order.user_id.clone(),
                        };
                        trades.push(trade);
                        quantity -= book_order.quantity;
                        remove += 1
                    }
                }

                for _ in 0..remove {
                    book_orders.favour.pop();
                }
                Some((quantity, trades))
            }
            Side::Favour => {
                // we will have to find matching NO order
                // if user is willing to buy YES at 60 then check in orderbook if there is any order for NO at least at price 40
                // if there is order for 45 then we will give user the best price which is 55 for YES
                // to check above condition the array needs to be sorted ascending
                // we will reverse traverse reverse to give user the best price

                let mut trades: Vec<TradeModel> = vec![];
                let match_price = 1000 - order.price;
                let mut quantity = order.quantity;
                let mut remove = 0;

                for book_order in book_orders.against.iter_mut().rev() {
                    // we will break if the highest price available for NO is less than minimum match price
                    if match_price > book_order.price {
                        break;
                    }
                    if book_order.quantity > quantity {
                        let trade = TradeModel {
                            id: None,
                            quantity,
                            opinion_id: opinion_id.clone(),
                            favour_user_id: book_order.user_id.clone(),
                            favour_price: book_order.price,
                            against_price: 1000 - book_order.price,
                            against_user_id: order.user_id.clone(),
                        };
                        trades.push(trade);
                        book_order.quantity -= quantity;
                        break;
                    } else {
                        let trade = TradeModel {
                            id: None,
                            quantity: book_order.quantity,
                            opinion_id: opinion_id.clone(),
                            favour_user_id: book_order.user_id.clone(),
                            favour_price: book_order.price,
                            against_price: 1000 - order.price,
                            against_user_id: order.user_id.clone(),
                        };
                        trades.push(trade);
                        quantity -= book_order.quantity;
                        remove += 1
                    }
                }

                for _ in 0..remove {
                    book_orders.against.pop();
                }

                Some((quantity, trades))
            }
        },
        None => None,
    };

    if let Some((quantity, trades)) = remaining {
        match order.side {
            Side::Against => {
                if quantity > 0 {
                    if let Some(order_book) = order_book.get_mut(&opinion_id) {
                        order_book.against.push(Order {
                            user_id: order.user_id,
                            quantity,
                            price: order.price,
                            side: order.side,
                        });
                    }
                };
                for trade in trades.iter() {
                    db.trade.create(&trade).await.unwrap();
                }
            }
            Side::Favour => {
                if quantity > 0 {
                    if let Some(order_book) = order_book.get_mut(&opinion_id) {
                        order_book.favour.push(Order {
                            user_id: order.user_id,
                            quantity,
                            price: order.price,
                            side: order.side,
                        });
                    }
                };
                for trade in trades.iter() {
                    db.trade.create(&trade).await.unwrap();
                }
            }
        }
    } else {
        match &order.side {
            Side::Against => {
                order_book.insert(
                    opinion_id.clone(),
                    OrderBook {
                        favour: vec![],
                        against: vec![order],
                    },
                );
            }
            Side::Favour => {
                order_book.insert(
                    opinion_id.clone(),
                    OrderBook {
                        favour: vec![order],
                        against: vec![],
                    },
                );
            }
        };
    }

    Json("ok")
}

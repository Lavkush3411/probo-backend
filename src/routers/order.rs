use axum::{
    Extension, Json, Router,
    extract::{Path, State},
    http::StatusCode,
    middleware::from_fn,
    response::IntoResponse,
    routing::{get, post},
};
use serde_json::json;
use validator::Validate;

use crate::{
    db::{db::DB, trade::TradeModel, user::UserModel},
    middlewares::auth::auth_middleware,
    state::{AppState, CreateOrderDto, Order, OrderBook, Side},
};

pub fn order_router() -> Router<AppState> {
    Router::new()
        .route("/{opinion_id}", post(handle_order))
        .route("/order_book", get(get_order_book))
        .layer(from_fn(auth_middleware))
}

async fn get_order_book(State(state): State<AppState>) -> impl IntoResponse {
    let order_book = state.order_book.read().await;
    return Json(json!({"order_book":order_book.clone()})).into_response();
}

async fn hold_balance(db: &DB, user_id: &String, order: &CreateOrderDto) -> bool {
    let result = db
        .user
        .hold_balance(user_id, (&order.price) * (&order.quantity))
        .await;

    match result {
        Ok(_) => true,
        Err(_) => false,
    }
}

#[axum::debug_handler]
async fn handle_order(
    State(state): State<AppState>,
    Path(opinion_id): Path<String>,
    Extension(user): Extension<UserModel>,
    Json(order): Json<CreateOrderDto>,
) -> impl IntoResponse {
    // check if user has enough money to add this order
    if let Err(e) = order.validate() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "message": "Trade failed","errors":e })),
        )
            .into_response();
    }
    let db = &state.db;
    let user_id = user.id.expect("User Id must be part of jwt token");
    if !hold_balance(&db, &user_id, &order).await {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"message":"You cannot trade with amount more than your balance"})),
        )
            .into_response();
    }

    let remaining = match_orders(&user_id, &opinion_id, &state, &order).await;

    if create_trades_and_update_order_book(&user_id, &opinion_id, &state, remaining, &order)
        .await
        .is_err()
    {
        let mut tx = db.pool.begin().await.unwrap();
        db.user
            .release_balance(&mut *tx, &user_id, order.price * order.quantity)
            .await
            .unwrap();
        tx.commit().await.unwrap();
    };
    Json("ok").into_response()
}

/**
 * find matching order
 * get unfulfilled quantity and list of trades that are match
 */
async fn match_orders(
    user_id: &String,
    opinion_id: &String,
    state: &AppState,
    order: &CreateOrderDto,
) -> Option<(u16, Vec<TradeModel>)> {
    let mut order_book = state.order_book.write().await;

    return match order_book.get_mut(opinion_id) {
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
                    if user_id == &book_order.user_id {
                        continue;
                    }
                    if book_order.quantity > quantity {
                        let trade = TradeModel {
                            id: None,
                            quantity,
                            opinion_id: opinion_id.clone(),
                            favour_user_id: book_order.user_id.clone(),
                            favour_price: book_order.price,
                            against_price: 1000 - book_order.price,
                            against_user_id: user_id.clone(),
                        };
                        trades.push(trade);
                        book_order.quantity -= quantity;
                        quantity = 0;
                        break;
                    } else {
                        let trade = TradeModel {
                            id: None,
                            quantity: book_order.quantity,
                            opinion_id: opinion_id.clone(),
                            favour_user_id: book_order.user_id.clone(),
                            favour_price: book_order.price,
                            against_price: 1000 - book_order.price,
                            against_user_id: user_id.clone(),
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
                    if user_id == &book_order.user_id {
                        continue;
                    }
                    if book_order.quantity > quantity {
                        let trade = TradeModel::new(
                            None,
                            opinion_id.clone(),
                            user_id.clone(),
                            book_order.user_id.clone(),
                            1000 - book_order.price,
                            book_order.price,
                            quantity,
                        );
                        trades.push(trade);
                        book_order.quantity -= quantity;
                        quantity = 0;
                        break;
                    } else {
                        // price given by user is just a price to check price against book orders
                        // or we can say its maximum that one user can pay
                        // actual trade will happen on the book price to be able to give best price to the user
                        let trade = TradeModel::new(
                            None,
                            opinion_id.clone(),
                            user_id.clone(),
                            book_order.user_id.clone(),
                            1000 - book_order.price,
                            book_order.price,
                            book_order.quantity,
                        );
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
}

/**
 * if there are unfulfilled quantities and fulfilled trades or any of them
 * create trade in db and add into order book
 * */
async fn create_trades_and_update_order_book(
    user_id: &String,
    opinion_id: &String,
    state: &AppState,
    remaining: Option<(u16, Vec<TradeModel>)>,
    order: &CreateOrderDto,
) -> Result<bool, String> {
    let db = &state.db;
    let mut order_book = state.order_book.write().await;

    if let Some((quantity, trades)) = remaining {
        match order.side {
            Side::Against => {
                if quantity > 0 {
                    if let Some(order_book) = order_book.get_mut(opinion_id) {
                        order_book.against.push(Order {
                            user_id: user_id.clone(),
                            quantity,
                            price: order.price,
                            side: order.side.clone(),
                        });
                        order_book.against.sort_by_key(|o| o.price);
                    }
                };
                for trade in trades.iter() {
                    let mut tx = db.pool.begin().await.unwrap();
                    db.trade.create(&mut *tx, &trade).await.unwrap();
                    // cleaning up hold balance if trade happens at lower price then the user requested.
                    db.user
                        .release_balance(
                            &mut *tx,
                            &user_id,
                            trade.quantity * (order.price - trade.against_price),
                        )
                        .await
                        .unwrap();
                    tx.commit().await.unwrap();
                }
            }
            Side::Favour => {
                // if some quantity is remaining to fill push and sort
                if quantity > 0 {
                    if let Some(order_book) = order_book.get_mut(opinion_id) {
                        order_book.favour.push(Order {
                            user_id: user_id.clone(),
                            quantity,
                            price: order.price,
                            side: order.side.clone(),
                        });
                        order_book.favour.sort_by_key(|o| o.price);
                    }
                };
                for trade in trades.iter() {
                    let mut tx = db.pool.begin().await.unwrap();
                    db.trade.create(&mut *tx, &trade).await.unwrap();
                    db.user
                        .release_balance(
                            &mut *tx,
                            &user_id,
                            trade.quantity * (order.price - trade.favour_price),
                        )
                        .await
                        .unwrap();
                    tx.commit().await.unwrap();
                }
            }
        }
    } else {
        // if there are 0 orders for a trade
        let order = Order {
            user_id: user_id.clone(),
            quantity: order.quantity,
            price: order.price,
            side: order.side.clone(),
        };
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

    return Ok(true);
}

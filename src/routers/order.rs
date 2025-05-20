use axum::{extract::{Path, State}, response::IntoResponse, routing::post, Json, Router};

use crate::state::{AppState, Order, OrderBook, Side};

pub fn order_router()->Router<AppState>{
Router::new().route("/{opinion_id}", post(handle_order))
}

#[axum::debug_handler]
async fn handle_order(State(state):State<AppState>, Path(opinion_id):Path<String>, Json(order):Json<Order>)->impl IntoResponse{

    let mut order_book = state.order_book.write().unwrap();



    Json("ok")
}
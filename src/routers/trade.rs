use axum::{Json, Router, extract::State, response::IntoResponse, routing::get};

use crate::{db::db::DB, state::AppState};

pub fn trade_router() -> Router<AppState> {
    Router::new().route("/", get(get_all_trades))
}

pub async fn get_all_trades(State(db): State<DB>) -> impl IntoResponse {
    let trades = db.trade.get_trades().await;

    match trades {
        Ok(trades) => Json(trades).into_response(),
        Err(_) => Json("Some Error occurred").into_response(),
    }
}

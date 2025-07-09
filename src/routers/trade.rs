use axum::{
    Extension, Json, Router,
    extract::{Query, State},
    middleware,
    response::IntoResponse,
    routing::get,
};
use serde::Deserialize;

use crate::{
    db::{db::DB, user::UserModel},
    middlewares::auth::auth_middleware,
    state::AppState,
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
    State(db): State<DB>,
    Extension(user): Extension<UserModel>,
    Query(query): Query<GetTradesQuery>,
) -> impl IntoResponse {
    let trades = db.trade.get_trades(user.id, query.active).await;

    match trades {
        Ok(trades) => Json(trades).into_response(),
        Err(_) => Json("Some Error occurred").into_response(),
    }
}

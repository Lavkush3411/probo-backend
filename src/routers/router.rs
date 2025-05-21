use super::{opinion::opinion_router, order::order_router, trade::trade_router, user::user_router};
use crate::state::AppState;
use axum::Router;

pub fn index_router() -> Router<AppState> {
    Router::new()
        .nest("/opinion", opinion_router())
        .nest("/user", user_router())
        .nest("/order", order_router())
        .nest("/trade", trade_router())
}

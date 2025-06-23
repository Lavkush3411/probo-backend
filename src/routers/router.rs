use super::{auth::auth_router, opinion::opinion_router, order::order_router, trade::trade_router, user::user_router};
use crate::state::AppState;
use axum::Router;

pub fn index_router() -> Router<AppState> {
    Router::new()
        .nest("/market", opinion_router())
        .nest("/user", user_router())
        .nest("/order", order_router())
        .nest("/trade", trade_router()).nest("/auth", auth_router())
}

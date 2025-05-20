use axum::Router;
use crate::state::AppState;
use super::{opinion::opinion_router, order::order_router, user::user_router};


pub fn index_router()->Router<AppState>{
    
    Router::new().nest("/opinion",opinion_router()).nest("/user", user_router()).nest("/order", order_router())
}
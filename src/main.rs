use axum::{Json, Router, response::IntoResponse, routing::get, serve};
use dotenv::dotenv;
use routers::router::index_router;
use serde_json::json;
use state::AppState;
use tokio::net::TcpListener;
mod db;
mod middlewares;
mod routers;
mod state;
use tower_http::cors::{Any, CorsLayer};

use crate::state::OrderBook;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any) // GET, POST, etc.
        .allow_headers(Any);

    let app_state = AppState::new().await;
    let app_state = load_data(app_state).await;
    let router = Router::new()
        .route("/health-check", get(health_check))
        .merge(index_router())
        .with_state(app_state)
        .layer(cors);

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();

    serve(listener, router).await.unwrap();
}

async fn health_check() -> impl IntoResponse {
    return Json(json!({"health":"Route is Healthy"})).into_response();
}

pub async fn load_data(state: AppState) -> AppState {
    let opinions = match state.db.opinion.find_many().await {
        Ok(opinions) => opinions,
        Err(err) => {
            eprintln!("DB error while loading opinions: {:?}", err);
            panic!("DB connection error");
        }
    };

    {
        let mut order_book = state.order_book.write().await;
        for opinion in opinions {
            if let Some(id) = opinion.id.clone() {
                order_book.insert(id, OrderBook::empty());
            }
        }
    }

    state
}

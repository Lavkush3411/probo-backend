use axum::{ response::IntoResponse, routing::get, serve, Json, Router};
use dotenv::dotenv;
use routers::router::index_router;
use serde_json::json;
use state::AppState;
use tokio::net::TcpListener;
mod db;
mod state;
mod routers;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let app_state= AppState::new().await;
    let router= Router::new().route("/health-check", get(health_check)).merge(index_router().await).with_state(app_state);

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();

    serve(listener, router).await.unwrap();
}


async fn health_check() -> impl IntoResponse{
    return  Json(json!({"health":"Route is Healthy"})).into_response();
}

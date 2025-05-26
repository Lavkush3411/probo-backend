use std::{collections::HashMap, sync::Arc};

use tokio::sync::RwLock;

use crate::db::db::DB;
use axum::extract::FromRef;
use serde::{Deserialize, Serialize};

pub type SharedOrderBook = Arc<RwLock<HashMap<String, OrderBook>>>;
#[derive(Clone, FromRef)]
pub struct AppState {
    pub db: DB,
    pub order_book: SharedOrderBook,
}

impl AppState {
    pub async fn new() -> Self {
        Self {
            db: DB::new().await,
            order_book: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderBook {
    pub favour: Vec<Order>,
    pub against: Vec<Order>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    pub user_id: String,
    pub quantity: u16,
    pub price: u16,
    pub side: Side,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateOrderDto {
    pub quantity: u16,
    pub price: u16,
    pub side: Side,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Side {
    Favour,
    Against,
}

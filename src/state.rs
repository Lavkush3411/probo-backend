use axum::extract::FromRef;
use crate::db::{ db::DB};

#[derive(Clone, FromRef )]
pub struct  AppState{
   pub db: DB
}

impl AppState {
    pub async fn new()->Self{
        Self { db:DB::new().await }
    }
}
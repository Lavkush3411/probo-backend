use serde::{Deserialize, Serialize};
use sqlx::{PgPool, prelude::FromRow};

#[derive(Clone)]
pub struct Opinion {
    pool: PgPool,
}

impl Opinion {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct OpinionModel {
    pub id: String,
    pub question: String,
    pub description: String,
    pub result: Option<bool>,
}

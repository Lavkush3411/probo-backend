use std::env;

use sqlx::{Pool, Postgres, postgres::PgPoolOptions};

use super::{opinion::Opinion, trade::Trade, user::User};

#[derive(Clone)]
pub struct DB {
    pub user: User,
    pub opinion: Opinion,
    pub trade: Trade,
    pub pool: Pool<Postgres>,
}

impl DB {
    pub async fn new() -> Self {
        let db_url = env::var("DATABASE_URL").unwrap();
        println!("DB URL : {}", db_url);
        let pool = PgPoolOptions::new().connect(&db_url).await.unwrap();
        Self {
            user: User::new(pool.clone()),
            opinion: Opinion::new(pool.clone()),
            trade: Trade::new(pool.clone()),
            pool: pool.clone(),
        }
    }
}

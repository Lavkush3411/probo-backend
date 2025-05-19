use std::env;

use sqlx::{postgres::PgPoolOptions};

use super::{opinion::Opinion, user::User};

#[derive(Clone)]
pub struct DB {
   user:User,
   opinion:Opinion
}

impl DB {
    pub async fn new()->Self{
        let db_url= env::var("DATABASE_URL").unwrap();
        println!("DB URL : {}",db_url);
        let pool= PgPoolOptions::new().connect(&db_url).await.unwrap();
        Self { user:User::new(pool.clone()), opinion: Opinion::new(pool.clone())}
    }
}
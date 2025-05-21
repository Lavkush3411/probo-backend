use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, query, query_as};

#[derive(Clone)]
pub struct User {
    pool: PgPool,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct UserModel {
    id: Option<String>,
    name: String,
    email: String,
    password: String,
    created_at: Option<NaiveDateTime>,
    updated_at: Option<NaiveDateTime>,
}

impl User {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, user: &UserModel) -> Result<UserModel, sqlx::Error> {
        query_as!(
            UserModel,
            r#"--sql
        INSERT INTO users (name, email, password)
        VALUES ($1,$2,$3)
        RETURNING id, name, email, password, created_at, updated_at
        "#,
            user.name,
            user.email,
            user.password
        )
        .fetch_one(&self.pool)
        .await
    }

    pub async fn get_users(&self) -> Result<Vec<UserModel>, sqlx::Error> {
        query_as!(
            UserModel,
            r#"--sql 
        SELECT * from users"#
        )
        .fetch_all(&self.pool)
        .await
    }
}

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, query_as};

#[derive(Clone)]
pub struct User {
    pool: PgPool,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct UserModel {
    pub id: Option<String>,
    pub name: String,
    pub email: String,
    pub password: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub balance: i32
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]

pub struct CreateUserDto{
    pub name: String,
    pub email: String,
    pub password: String,
}

impl User {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, user: &CreateUserDto) -> Result<UserModel, sqlx::Error> {
        query_as!(
            UserModel,
            r#"--sql
        INSERT INTO users (name, email, password)
        VALUES ($1,$2,$3)
        RETURNING id, name, email, password, created_at, updated_at, balance
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

    pub async fn get_by_id(&self,id:String)->Result<UserModel, sqlx::Error>{
        query_as!(UserModel,
        r#"--sql
        SELECT id, name, email, password, created_at, updated_at, balance FROM users WHERE id=$1
        "#,&id).fetch_one(&self.pool).await
    }

    pub async fn get_by_email(&self, email:&String)->Result<UserModel, sqlx::Error>{
        query_as!(UserModel,
        r#"--sql 
        SELECT id, name, email, password, created_at, updated_at, balance FROM users WHERE email=$1"#,email).fetch_one(&self.pool).await
    }
}

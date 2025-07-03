use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, query, query_as};

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
    pub balance: i32,
    pub hold_balance: i32,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]

pub struct CreateUserDto {
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
        RETURNING id, name, email, password, created_at, updated_at, balance, hold_balance
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

    pub async fn get_by_id(&self, id: &String) -> Result<UserModel, sqlx::Error> {
        query_as!(
            UserModel,
            r#"--sql
        SELECT id, name, email, password, created_at, updated_at, balance, hold_balance FROM users WHERE id=$1
        "#,
            &id
        )
        .fetch_one(&self.pool)
        .await
    }

    pub async fn hold_balance(&self, id: &String, amount: u16) -> Result<(), sqlx::Error> {
        query!(
            r#"--sql
            UPDATE users set hold_balance=hold_balance+$1 , balance=balance-$1  where id=$2
                "#,
            amount as i32,
            id
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn release_balance<'a, E>(
        &self,
        executor: E,
        user_id: &String,
        amount: u16,
    ) -> Result<(), sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        query!(
            r#"--sql
        UPDATE users set hold_balance=hold_balance-$1 , balance=balance+$1  where id=$2
        "#,
            amount as i32,
            user_id
        )
        .execute(executor)
        .await?;
        Ok(())
    }

    pub async fn update_balance_post_result<'a, E>(
        &self,
        executor: E,
        winner_id: &String,
        loser_id: &String,
        winner_hold: u16,
        loser_hold: u16,
        winning_price: u16,
    ) -> Result<(), sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        query!(
            r#"--sql
            UPDATE users
            SET
              balance = balance + CASE
                WHEN id = $1 THEN $2
                ELSE 0
              END,
              hold_balance = hold_balance - CASE
                WHEN id = $1 AND $1 = $4 THEN ($3::numeric + $5::numeric)
                WHEN id = $1 THEN $3
                WHEN id = $4 THEN $5
                ELSE 0
              END
            WHERE id IN ($1, $4);"#,
            winner_id,
            winning_price as i32,
            winner_hold as i32,
            loser_id,
            loser_hold as i32
        )
        .execute(executor)
        .await?;
        Ok(())
    }

    pub async fn get_by_email(&self, email: &String) -> Result<UserModel, sqlx::Error> {
        query_as!(UserModel,
        r#"--sql 
        SELECT id, name, email, password, created_at, updated_at, balance, hold_balance FROM users WHERE email=$1"#,email).fetch_one(&self.pool).await
    }
}

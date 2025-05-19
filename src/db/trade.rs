use sqlx::{PgPool, query, query_as};

#[derive(Clone)]
pub struct Trade {
    pool: PgPool,
}

impl Trade {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self) {
        let _ = query!(
            r#"--sql
        INSERT INTO trades (opinion_id, favour_user_id,against_user_id )
        VALUES ($1,$2,$3)
        "#,
            "",
            "",
            ""
        )
        .execute(&self.pool)
        .await;
    }
}

pub struct TradeModel {
    pub id: Option<String>,
    pub opinion_id: String,
    pub favour_user_id: String,
    pub against_user_id: String,
}

use serde::{Deserialize, Serialize};
use sqlx::{PgPool, prelude::FromRow, query};

#[derive(Clone)]
pub struct Trade {
    pool: PgPool,
}

impl Trade {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, trade: &TradeModel) -> Result<(), sqlx::Error> {
        let _ = query!(
            r#"--sql
        INSERT INTO trades (opinion_id, favour_user_id,against_user_id, favour_price, against_price,quantity )
        VALUES ($1,$2,$3,$4,$5,$6)
        "#,
            &trade.opinion_id,
            &trade.favour_user_id,
            &trade.against_user_id,trade.favour_price as i64,trade.against_price as i64,trade.quantity as i64
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn get_trades(&self) -> Result<Vec<TradeModel>, sqlx::Error> {
        let trades = query!(
            r#"--sql
            SELECT  id, 
            opinion_id, 
            favour_user_id, 
            against_user_id, 
            favour_price, 
            against_price, quantity
        FROM trades
        "#
        )
        .fetch_all(&self.pool)
        .await?;

        let t = trades
            .into_iter()
            .map(|row| TradeModel {
                id: Some(row.id),
                opinion_id: row.opinion_id,
                favour_user_id: row.favour_user_id,
                against_user_id: row.against_user_id,
                favour_price: row.favour_price.try_into().unwrap(), // i16 -> u16
                against_price: row.against_price.try_into().unwrap(), // i16 -> u16
                quantity: row.quantity.try_into().unwrap(),
            })
            .collect();

        Ok(t)
    }
}

#[derive(Serialize, Deserialize, Debug, Default, FromRow)]
pub struct TradeModel {
    pub id: Option<String>,
    pub opinion_id: String,
    pub favour_user_id: String,
    pub against_user_id: String,
    pub favour_price: u16,
    pub against_price: u16,
    pub quantity: u16,
    // pub created_at: Option<NaiveDateTime>,
}

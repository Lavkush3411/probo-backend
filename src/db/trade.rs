use serde::{Deserialize, Serialize};
use sqlx::{Executor, PgPool, Postgres, prelude::FromRow, query};

#[derive(Clone)]
pub struct Trade {
    pool: PgPool,
}

impl Trade {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create<'a, E>(&self, executor: E, trade: &TradeModel) -> Result<(), sqlx::Error>
    where
        E: Executor<'a, Database = Postgres>,
    {
        query!(
            r#"--sql
        INSERT INTO trades (opinion_id, favour_user_id,against_user_id, favour_price, against_price,quantity )
        VALUES ($1,$2,$3,$4,$5,$6)
        "#,
            &trade.opinion_id,
            &trade.favour_user_id,
            &trade.against_user_id,trade.favour_price as i64,trade.against_price as i64,trade.quantity as i64
        )
        .execute(executor)
        .await?;

        Ok(())
    }

    pub async fn get_trades(
        &self,
        user_id: Option<String>,
        active: Option<bool>,
    ) -> Result<Vec<TradeModel>, sqlx::Error> {
        // if true then result then gets active trades
        // false then get closed trades
        let trades = query!(
            r#"--sql
            SELECT  t.id, 
            opinion_id, 
            favour_user_id, 
            against_user_id, 
            favour_price, 
            against_price, quantity
        FROM trades t JOIN opinions o ON t.opinion_id = o.id WHERE ($1::text IS NULL OR favour_user_id=$1 OR against_user_id=$1) AND (($2::bool = true AND o.result IS NULL) OR
        ($2::bool = false AND o.result IS NOT NULL))
        "#,
            user_id,
            active.unwrap_or(true)
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

    pub async fn get_trades_by_opinion_id(
        &self,
        opinion_id: &String,
    ) -> Result<Vec<TradeModel>, sqlx::Error> {
        let trades = query!(
            r#"--sql
            SELECT  id, 
            opinion_id, 
            favour_user_id, 
            against_user_id, 
            favour_price, 
            against_price, quantity
        FROM trades
        WHERE opinion_id = $1
        "#,
            opinion_id
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

impl TradeModel {
    pub fn new(
        id: Option<String>,
        opinion_id: String,
        favour_user_id: String,
        against_user_id: String,
        favour_price: u16,
        against_price: u16,
        quantity: u16,
    ) -> Self {
        Self {
            id,
            opinion_id,
            favour_user_id,
            against_user_id,
            favour_price,
            against_price,
            quantity,
        }
    }
}

use serde::{Deserialize, Serialize};
use sqlx::{Error, PgPool, prelude::FromRow, query_as};

#[derive(Clone)]
pub struct Opinion {
    pool: PgPool,
}

impl Opinion {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn find_one(&self, id: String) -> Result<OpinionModel, Error> {
        query_as!(
            OpinionModel,
            "--sql
        Select * from opinions where id=$1",
            id
        )
        .fetch_one(&self.pool)
        .await
    }

    pub async fn insert(&self, question: String) -> Result<OpinionModel, Error> {
        query_as!(
            OpinionModel,
            "--sql
        INSERT INTO opinions (question)
        VALUES ($1)
        RETURNING id, question, description, result",
            question
        )
        .fetch_one(&self.pool)
        .await
    }
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct OpinionModel {
    pub id: Option<String>,
    pub question: String,
    pub description: Option<String>,
    pub result: Option<bool>,
}

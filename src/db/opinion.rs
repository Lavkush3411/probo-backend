use serde::{Deserialize, Serialize};
use sqlx::{Error, Executor, PgPool, Postgres, prelude::FromRow, query, query_as};

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

    pub async fn find_many(&self) -> Result<Vec<OpinionModel>, Error> {
        query_as!(
            OpinionModel,
            r#"--sql 
        SELECT * FROM opinions WHERE result is NULL"#
        )
        .fetch_all(&self.pool)
        .await
    }

    pub async fn update_result<'a, E>(
        &self,
        executor: E,
        opinion_id: &String,
        result: bool,
    ) -> Result<(), Error>
    where
        E: Executor<'a, Database = Postgres>,
    {
        query!(
            r#"--sql
            UPDATE opinions
            SET result=$1
            WHERE id=$2
        "#,
            result,
            opinion_id
        )
        .execute(executor)
        .await?;
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct OpinionModel {
    pub id: Option<String>,
    pub question: String,
    pub description: Option<String>,
    pub result: Option<bool>,
}

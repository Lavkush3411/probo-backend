use sqlx::PgPool;

#[derive(Clone)]
pub struct  Opinion{
    pool:PgPool
}

impl Opinion {
    pub fn new(pool:PgPool)->Self{
        Self { pool }
    }
}
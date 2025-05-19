use sqlx::PgPool;

#[derive(Clone)]
pub struct  User {
    pool:PgPool
}

impl User {
    pub fn new(pool:PgPool)->Self{
        Self{pool}
    }
    
    pub fn create_user(&self){

    }
}


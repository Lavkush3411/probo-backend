use sqlx::PgPool;

#[derive(Clone)]
pub struct  User {
    pool:PgPool
}

pub struct UserModel{
    id:Option<String>,
}

impl User {
    pub fn new(pool:PgPool)->Self{
        Self{pool}
    }
    
    pub fn create(&self){

    }
}


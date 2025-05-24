use axum::{extract::Request, http::StatusCode, middleware::Next, response::IntoResponse, Json};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};

use crate::db::user::{ UserModel};


pub async fn auth_middleware(mut request:Request, next:Next)-> impl IntoResponse{

    let token =match request.headers().get("token") {
        Some(t) => match t.to_str() {
            Ok(t_str) => {
                let parts:Vec<&str> =t_str.split(" ").collect();
                parts[1]
            },
            Err(_) => return (StatusCode::UNAUTHORIZED, Json("Invalid token format")).into_response(),
        },
        None => return (StatusCode::UNAUTHORIZED, Json("Token is required")).into_response(),
    };
    let data = decode::<UserModel>(&token, &DecodingKey::from_secret("secret".as_ref()), &Validation::new(Algorithm::HS256));
    let user =match data {
        Ok(data)=>{data.claims},
        Err(_)=>{return (StatusCode::UNAUTHORIZED, Json("Token is invalid")).into_response();}
        
    };
    request.extensions_mut().insert(user);
    next.run(request).await.into_response()

}
use axum::{extract::State, http::StatusCode, middleware, response::IntoResponse, routing::{get, post}, Extension, Json, Router};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{db::{db::DB, user::{CreateUserDto, UserModel}}, middlewares::auth::auth_middleware, state::AppState};

#[derive(Serialize)]
struct ErrorMessage {
    message: &'static str,
}
#[derive(Serialize,Deserialize,Debug)]
pub struct LoginDto{
    pub email:String,
    pub password:String
}


pub fn auth_router()-> Router<AppState>{
    let protected = Router::new()
        .route("/active-user", get(active_user))
        .layer(middleware::from_fn(auth_middleware));
    Router::new().route("/login", post(login)).route("/signup", post(create_user)).
    merge(protected)
}

#[axum::debug_handler]
pub async fn login(State(db):State<DB>, Json(req_user):Json<LoginDto> )-> impl IntoResponse{
    let user= db.user.get_by_email(&req_user.email).await;

    let user = match  user {
        Ok(user)=>user,
        Err(_)=> return  (StatusCode::NOT_FOUND,Json(ErrorMessage{message:"User Not found"})).into_response()
        
    };

    if user.password!= req_user.password {
        return  (StatusCode::UNAUTHORIZED, Json("Wrong password")).into_response();
    }

    let token =encode::<UserModel>(&Header::default(),&user ,&EncodingKey::from_secret("secret".as_ref())).expect("Unable to serialize json web token");
    Json(json!({"token":token})).into_response()

}

#[axum::debug_handler]
pub async fn create_user(State(db): State<DB>, Json(user): Json<CreateUserDto>) -> impl IntoResponse {
    let new_user = db.user.create(&user).await;
    match new_user {
        Ok(user) => Json(user).into_response(),
        Err(_) => Json("Some Error Occurred while Creating user").into_response(),
    }
}

#[axum::debug_handler]
pub async  fn active_user(Extension(user):Extension<UserModel>)->impl IntoResponse{
    return Json(user);
}
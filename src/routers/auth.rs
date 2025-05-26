use axum::{extract::State, http::StatusCode,  response::IntoResponse, routing::post, Json, Router};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{db::{db::DB, user::UserModel}, state::AppState};

#[derive(Serialize,Deserialize,Debug)]
pub struct LoginDto{
    pub email:String,
    pub password:String
}
pub fn auth_router()-> Router<AppState>{
    Router::new().route("/login", post(login))
}

#[axum::debug_handler]
pub async fn login(State(db):State<DB>, Json(req_user):Json<LoginDto> )-> impl IntoResponse{
    let user:UserModel= db.user.get_by_email(&req_user.email).await.expect("User not found");

    if user.password!= req_user.password {
        return  (StatusCode::UNAUTHORIZED, Json("Wrong password")).into_response();
    }

    let token =encode::<UserModel>(&Header::default(),&user ,&EncodingKey::from_secret("secret".as_ref())).expect("Unable to serialize json web token");
    Json(json!({"token":token})).into_response()

}
use axum::{http::StatusCode, Json};
use jsonwebtoken::{encode, EncodingKey, Header};

use crate::model::{Claims, LoginInfo, LoginResponse};




pub async fn login_handler(Json(login):Json<LoginInfo>) ->Result<Json<LoginResponse>,StatusCode>{
    let username = &login.username;
    let password = &login.password;
    match is_valid_user(username, password){
        false=> Err(StatusCode::UNAUTHORIZED),
        true =>{
            let claims = Claims{
                sub: username.clone(),
                exp: (chrono::Utc::now()+chrono::Duration::seconds(10)).timestamp() as usize,
            };
            let token = match encode(&Header::default(), &claims, &EncodingKey::from_secret("Secret".as_ref())){
                Ok(tok)=> tok,
                Err(e)=>{
                    println!("Error generating token {:?}",e);
                    return Err(StatusCode::INTERNAL_SERVER_ERROR);
                }
            };
            Ok(Json(LoginResponse{token}))
        }
    }


}

pub fn is_valid_user(username: &str, password:&str)->bool{
username != "" && password!= ""
}

pub async fn get_info() -> Result<Json<String>,StatusCode>{

}
use std::{sync::Mutex, str::FromStr};

use crate::{helpers::ddb::DB, models::{task::{GetTask, SetTask, AccessMode}, user::{GetUser, SetUser, Role, User, LoginDTO}, user_token::UserToken}};
use actix_web::{
    get, 
    post, 
    put,
    error::ResponseError,
    web::Path,
    web::{Json, self},
    web::Data,
    HttpResponse,
    http::{header::ContentType, StatusCode}, Responder
};
use mysql::serde_json;
use serde::{Serialize, Deserialize};
use strum::{Display};

#[derive(Deserialize, Serialize)]
pub struct UserIdentifier {
    id: u32,
}

#[derive(Debug, Display)]
pub enum UserError {
    UserNotFound,
    UserUpdateFailure,
    UserCreationFailure,
    BadUserRequest
}

#[derive(Serialize, Deserialize)]
pub struct TokenBodyResponse {
    pub token: String,
    pub token_type: String,
}

impl ResponseError for UserError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match self {
            UserError::UserNotFound => StatusCode::NOT_FOUND,
            UserError::UserUpdateFailure => StatusCode::FAILED_DEPENDENCY,
            UserError::UserCreationFailure => StatusCode::FAILED_DEPENDENCY,
            UserError::BadUserRequest => StatusCode::BAD_REQUEST
        }
    }
}

#[get("/automation/user/get/{id}")]
pub async fn get_user(
    db: Data<Mutex<DB>>,
    path: Path<UserIdentifier>,
) -> Result<Json<GetUser>, UserError> {
    let id = path.into_inner().id;
    let users = db.lock().unwrap().get_user(id);

    match users.into_iter().next() {
        Some(user) => Ok(Json(user)),
        None => Err(UserError::UserNotFound)
    }
}


#[post("/automation/user/new")]
pub async fn sign_up(
    db: Data<Mutex<DB>>,
    body_bytes: actix_web::web::Bytes
) -> Result<Json<Vec<String>>, UserError> {
    let body: json::JsonValue = json::parse(std::str::from_utf8(&body_bytes).unwrap()).unwrap();
    let user = SetUser{
        username: body["username"].to_string(),
        email: body["email"].to_string(),
        password: body["password"].to_string(),
        //Role::from_str(body["role"].as_str().unwrap()).unwrap()
    };
    

    match User::signup(user, &db) {
        Ok(u) => Ok(Json(u)),
        Err(_) => Err(UserError::UserCreationFailure)
    }
}


#[post("/automation/user/login")]
pub async fn login(login_dto: web::Json<LoginDTO>, db: Data<Mutex<DB>>) -> Result<impl Responder, UserError> {
    
    match logins(login_dto.0, &db) {
        Ok(token_res) => Ok(Json(token_res)),
        Err(err) => Err(err),
    }
}


pub fn logins(
    login_dto: LoginDTO,
    db: &Data<Mutex<DB>>
) -> Result<TokenBodyResponse, UserError> {
    if let Some(logged_user) = User::login(login_dto, &db) {
        match serde_json::from_value(
            serde_json::json!({ "token": UserToken::generate_token(&logged_user), "token_type": "bearer" }),
        ) {
            Ok(token_res) => {
                if logged_user.login_session.is_empty() {
                    return Err(UserError::BadUserRequest);
                } else {
                    return Ok(token_res);
                }
            }
            Err(_) => return Err(UserError::BadUserRequest),
        }
    }
    Err(UserError::BadUserRequest)
}


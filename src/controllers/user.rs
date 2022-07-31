use std::{sync::Mutex, str::FromStr};

use crate::{helpers::ddb::DB, models::{task::{GetTask, SetTask, AccessMode}, user::{GetUser, SetUser, Role}}};
use actix_web::{
    get, 
    post, 
    put,
    error::ResponseError,
    web::Path,
    web::Json,
    web::Data,
    HttpResponse,
    http::{header::ContentType, StatusCode}
};
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
pub async fn new_user(
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

    match db.lock().unwrap().put_user(user) {
        Ok(u) => Ok(Json(u)),
        Err(_) => Err(UserError::UserCreationFailure)
    }
}

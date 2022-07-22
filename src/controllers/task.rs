use std::sync::Mutex;

use crate::{helpers::ddb::DB, models::task::{GetTask, SetTask, AccessMode}};
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
pub struct TaskIdentifier {
    name: String,
}

#[derive(Debug, Display)]
pub enum TaskError {
    TaskNotFound,
    TaskUpdateFailure,
    TaskCreationFailure,
    BadTaskRequest
}

impl ResponseError for TaskError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match self {
            TaskError::TaskNotFound => StatusCode::NOT_FOUND,
            TaskError::TaskUpdateFailure => StatusCode::FAILED_DEPENDENCY,
            TaskError::TaskCreationFailure => StatusCode::FAILED_DEPENDENCY,
            TaskError::BadTaskRequest => StatusCode::BAD_REQUEST
        }
    }
}

#[get("/automation/task/get/{name}")]
pub async fn get_task(
    db: Data<Mutex<DB>>,
    path: Path<TaskIdentifier>,
) -> Result<Json<GetTask>, TaskError> {
    let name = path.into_inner().name;
    let task = db.lock().unwrap().get_task(name);

    match task.into_iter().next() {
        Some(tsk) => Ok(Json(tsk)),
        None => Err(TaskError::TaskNotFound)
    }
}


#[post("/automation/task/new")]
pub async fn new_task(
    db: Data<Mutex<DB>>,
    body_bytes: actix_web::web::Bytes
) -> Result<Json<Vec<String>>, TaskError> {
    let body: json::JsonValue = json::parse(std::str::from_utf8(&body_bytes).unwrap()).unwrap();
    let task = SetTask{
        name: body["name"].to_string(),
        has_encryption: body["has_encryption"].as_bool().unwrap(),
        variables: body["variables"].to_string(),
        functions: body["functions"].to_string(),
        tasks: body["tasks"].to_string(),
        params: body["params"].members().map(|x| x.to_string()).collect::<Vec<String>>(),
        description: body["description"].to_string(),
        author_id: "temp".to_string(),
        access_mode: AccessMode::Public,
    };

    match db.lock().unwrap().put_task(task) {
        Ok(tsk) => Ok(Json(tsk)),
        Err(_) => Err(TaskError::TaskCreationFailure)
    }
}

// #[put("/task/{name}/start")]
// pub async fn start_task(
//     ddb_repo: Data<DDBRepository>, 
//     task_identifier: Path<TaskIdentifier>
// ) -> Result<Json<TaskIdentifier>, TaskError> {
//     //Ok(task_identifier.into_inner().name)
//     Ok(Json(task_identifier.into_inner()))
// }

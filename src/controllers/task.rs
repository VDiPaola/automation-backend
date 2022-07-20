use std::sync::Mutex;

use crate::{helpers::ddb::DB, models::task::GetTask};
use actix_web::{
    get, 
    post, 
    put,
    error::ResponseError,
    web::Path,
    web::Json,
    web::Data,
    HttpResponse,
    http::{header::ContentType, StatusCode}, middleware::Logger
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

#[get("/task/{name}")]
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


// #[derive(Deserialize, Serialize)]
// pub struct TaskIdentifier {
//     name: String,
// }

// #[derive(Debug, Display)]
// pub enum TaskError {
//     TaskNotFound,
//     TaskUpdateFailure,
//     TaskCreationFailure,
//     BadTaskRequest
// }

// impl ResponseError for TaskError {
//     fn error_response(&self) -> HttpResponse {
//         HttpResponse::build(self.status_code())
//             .insert_header(ContentType::json())
//             .body(self.to_string())
//     }

//     fn status_code(&self) -> StatusCode {
//         match self {
//             TaskError::TaskNotFound => StatusCode::NOT_FOUND,
//             TaskError::TaskUpdateFailure => StatusCode::FAILED_DEPENDENCY,
//             TaskError::TaskCreationFailure => StatusCode::FAILED_DEPENDENCY,
//             TaskError::BadTaskRequest => StatusCode::BAD_REQUEST
//         }
//     }
// }

// #[get("/task/{name}")]
// pub async fn get_task(
//     ddb_repo: Data<DDBRepository>, 
//     task_identifier: Path<TaskIdentifier>
// ) -> Result<Json<Task>, TaskError> {
//     let tsk = ddb_repo.get_task(
//         task_identifier.into_inner().name
//     ).await;

//     match tsk {
//         Some(tsk) => Ok(Json(tsk)),
//         None => Err(TaskError::TaskNotFound)
//     }
// }

// #[post("/task")]
// pub async fn create_task(
//     ddb_repo: Data<DDBRepository>,
//     request: Json<Task>
// ) -> Result<Json<Task>, TaskError> {
//     let task = Task{
//         name: request.name.clone(),
//         has_encryption: request.has_encryption.clone(),
//         variables: request.variables.clone(),
//         functions: request.functions.clone(),
//         tasks: request.tasks.clone(),
//         params: request.params.clone()
//     };

//     match ddb_repo.put_task(task).await {
//         Ok(()) => Ok(Json(task)),
//         Err(_) => Err(TaskError::TaskCreationFailure)
//     }
// }

// #[put("/task/{name}/start")]
// pub async fn start_task(
//     ddb_repo: Data<DDBRepository>, 
//     task_identifier: Path<TaskIdentifier>
// ) -> Result<Json<TaskIdentifier>, TaskError> {
//     //Ok(task_identifier.into_inner().name)
//     Ok(Json(task_identifier.into_inner()))
// }

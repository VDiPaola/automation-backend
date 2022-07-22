mod controllers;
mod models;
mod helpers;

use controllers::task::{get_task,new_task};
use controllers::user::{get_user,new_user};
use helpers::ddb::{DB};
use actix_web::{HttpServer, App, web::Data, middleware::Logger};

use dotenv::dotenv;
use std::{env, sync::Mutex};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    dotenv().ok();

    let db_user = env::var("DB_USER").expect("env var not set");
    let db_pass = env::var("DB_PASS").expect("env var not set");
    let db_port = env::var("DB_PORT").expect("env var not set");
    let db_name = env::var("DB_NAME").expect("env var not set");

    HttpServer::new(move || {
        let db: DB = DB::new(
            db_user.clone(),
            db_pass.clone(),
            db_port.clone(),
            db_name.clone()
        );
        let ddb_data = Data::new(Mutex::new(db));
        let logger = Logger::default();
        App::new()
            .wrap(logger)
            .app_data(ddb_data)
            .service(get_task)
            .service(new_task)
            .service(get_user)
            .service(new_user)
    })
    .bind(("127.0.0.1", 80))?
    .run()
    .await
}


/*
-can have a power automate request as paramter and call that with data that can be used in the automation of that flow
*/
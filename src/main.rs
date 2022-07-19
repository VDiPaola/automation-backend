mod controllers;
mod models;
mod helpers;

// use controllers::task::{
//     get_task,
//     create_task,
//     start_task,
// };
use helpers::ddb::{DB};
use actix_web::{HttpServer, App, web::Data, middleware::Logger};

use dotenv::dotenv;
use std::env;

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
        let ddb_data = Data::new(db);
        let logger = Logger::default();
        App::new()
            .wrap(logger)
            .app_data(ddb_data)
            // .service(get_task)
            // .service(create_task)
            // .service(start_task)
    })
    .bind(("127.0.0.1", 80))?
    .run()
    .await
}
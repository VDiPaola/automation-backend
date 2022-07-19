mod helpers;
mod models;
mod controllers;

use helpers::ddb::{DB};
use actix_web::{HttpServer, App, web::Data, middleware::Logger};

use dotenv::dotenv;
use std::env;
//cargo test -- --nocapture
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn get_row() {
        dotenv().ok();

        let db_user = env::var("DB_USER").expect("env var not set");
        let db_pass = env::var("DB_PASS").expect("env var not set");
        let db_port = env::var("DB_PORT").expect("env var not set");
        let db_name = env::var("DB_NAME").expect("env var not set");

        let mut db: DB = DB::new(
            db_user.clone(),
            db_pass.clone(),
            db_port.clone(),
            db_name.clone()
        );

        let name = db.get_task("test".to_string());
        println!("{:?}", name);
    }
}
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
    use crate::models::task::{SetTask, AccessMode};

    use super::*;

    fn init() -> DB{
        dotenv().ok();

        let db_user = env::var("DB_USER").expect("env var not set");
        let db_pass = env::var("DB_PASS").expect("env var not set");
        let db_port = env::var("DB_PORT").expect("env var not set");
        let db_name = env::var("DB_NAME").expect("env var not set");

        DB::new(
            db_user.clone(),
            db_pass.clone(),
            db_port.clone(),
            db_name.clone()
        )
    }

    #[test]
    fn get_test() {
        let mut db = init();
        let name = db.get_task("test".to_string());
        println!("{:?}", name);
    }

    #[test]
    fn set_test() {
        let mut db = init();
        let task = SetTask{
            name: "settest".to_string(),
            functions: "{\"a\":\"b\"}".to_string(),
            variables: "{\"b\":\"c\"}".to_string(),
            tasks: "{\"d\":\"e\"}".to_string(),
            params: vec!["something","else"].into_iter().map(String::from).collect(),
            has_encryption: false,
            description: "description".to_string(),
            author_id: "temp".to_string(),
            access_mode: AccessMode::Public,
        };
        match db.put_task(task){
            Ok(something) => println!("{:?}", something),
            Err(err) => println!("error: {:?}", err)
        };
    }
}
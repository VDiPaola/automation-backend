
use std::error::Error;

use mysql::{Pool, PooledConn, params};
use mysql::prelude::*;

use crate::models::task::{SetTask, GetTask};


pub struct DB{
    pub db_name: String,
    conn_string: String,
    conn_pool: Pool,
    connection: PooledConn
}


impl DB{
    pub fn new(db_user: String, db_pass: String, db_port: String, db_name: String) -> Self{
        //setup database connection
        let conn_string = format!("mysql://{}:{}@localhost:{}/{}", db_user, db_pass, db_port, db_name);
        let conn_pool = Pool::new(conn_string.as_str()).unwrap();
        let mut connection = conn_pool.get_conn().unwrap();

        //make sure table exists
        connection.query_drop(
            r"CREATE TABLE IF NOT EXISTS tasks (
                `id` INT NOT NULL AUTO_INCREMENT,
                `name` VARCHAR(200) NOT NULL,
                `variables` JSON NULL,
                `functions` JSON NULL,
                `tasks` JSON NOT NULL,
                `params` TEXT NULL,
                `has_encryption` BIT NOT NULL,
                UNIQUE INDEX `name_UNIQUE` (`name` ASC) VISIBLE,
                PRIMARY KEY (`id`)
            )").unwrap();


        DB{
            db_name,
            conn_string,
            conn_pool,
            connection
        }
        
    }


    pub fn get_task(&mut self, name: String) -> Vec<GetTask>{
        let task = self.connection.exec_map(
            format!("SELECT name, params, has_encryption has FROM tasks WHERE name = \"{}\"", name),
            (),
            | (name, p, has_encryption): (String,String, bool) | GetTask{
                name,
                params: p.split(",").map(String::from).collect(),
                has_encryption
            },
        ).unwrap();
        return task;
    }

    pub fn put_task(&mut self, task: SetTask) -> Result<Vec<String>,mysql::Error>{
        self.connection.exec(r"INSERT INTO tasks (name, variables, functions,tasks, params, has_encryption)
            VALUES (:name, :variables, :functions, :tasks, :params, :has_encryption)",
            params! {
                "name" => task.name,
                "variables" => task.variables,
                "functions" => task.functions,
                "tasks" => task.tasks,
                "params" => task.params.join(","),
                "has_encryption" => task.has_encryption,
            })
    }
}

use std::error::Error;
use std::str::FromStr;

use mysql::{Pool, PooledConn, params};
use mysql::prelude::*;

use crate::models::task::{SetTask, GetTask};
use crate::models::user::{GetUser, SetUser, Role};


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

        connection.query_drop(
                r"CREATE TABLE IF NOT EXISTS `users` (
                    `id` bigint unsigned NOT NULL AUTO_INCREMENT,
                    `email` varchar(191) COLLATE utf8mb4_bin DEFAULT NULL,
                    `password` longtext COLLATE utf8mb4_bin,
                    `username` varchar(191) COLLATE utf8mb4_bin DEFAULT NULL,
                    `role` varchar(191) COLLATE utf8mb4_bin DEFAULT 'User',
                    `verified` tinyint(1) DEFAULT '0',
                    `last_login` bigint DEFAULT NULL,
                    `code_value` varchar(191) COLLATE utf8mb4_bin DEFAULT NULL,
                    `code_expires_at` bigint DEFAULT NULL,
                    PRIMARY KEY (`id`),
                    UNIQUE KEY `email` (`email`),
                    UNIQUE KEY `username` (`username`),
                    UNIQUE KEY `code_value` (`code_value`)
                  ) ENGINE=InnoDB AUTO_INCREMENT=1 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_bin").unwrap();


        DB{
            db_name,
            conn_string,
            conn_pool,
            connection
        }
        
    }

    //TASKS
    pub fn get_task(&mut self, name: String) -> Vec<GetTask>{
        let task = self.connection.exec_map(
            format!("SELECT name, params, has_encryption, author_id, description FROM tasks WHERE name = \"{}\"", name),
            (),
            | (name, p, has_encryption, author_id, description): (String,String, bool, String, String) | GetTask{
                name,
                params: p.split(",").map(String::from).collect(),
                has_encryption,
                author_id,
                description
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
                "description" => task.description,
                "author_id" => task.author_id,
            })
    }

    //USERS
    pub fn get_user(&mut self, id: u32) -> Vec<GetUser>{
        let user = self.connection.exec_map(
            format!("SELECT username, role FROM users WHERE id = \"{}\"", id),
            (),
            | (username, role): (String,String) | GetUser{
                username,
                role:Role::from_str(role.as_str()).unwrap(),

            },
        ).unwrap();
        return user;
    }

    pub fn put_user(&mut self, user: SetUser) -> Result<Vec<String>,mysql::Error>{
        self.connection.exec(r"INSERT INTO users (username, role, password, email)
            VALUES (:username, :role, :password, :email)",
            params! {
                "username" => user.username,
                "role" => user.role.to_string(),
                "password" => user.password,
                "email" => user.email,
            })
    }
}
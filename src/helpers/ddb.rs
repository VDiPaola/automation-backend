
use std::error::Error;
use std::str::FromStr;

use mysql::{Pool, PooledConn, params};
use mysql::prelude::*;

use crate::models::task::{SetTask, GetTask};
use crate::models::user::{GetUser, SetUser, Role, User};


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
                    `login_session` varchar(191) COLLATE utf8mb4_bin DEFAULT NULL,
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
    pub fn get_user(&mut self, id: u32) -> Result<GetUser, ()>{
        let user = self.connection.exec_map(
            format!("SELECT username, role, login_session FROM users WHERE id = \"{}\"", id),
            (),
            | (username, role, login_session): (String,String,String) | {
                let role = Role::from_str(role.as_str()).unwrap_or(Role::User);
                GetUser{
                username,
                role,
                login_session
                }
            },
        ).unwrap();
        match user.into_iter().next(){
            Some(u) => Ok(u),
            _ => Err(())
        }
    }

    pub fn get_user_by_username_or_email(&mut self, username_or_email: String) -> Result<User, ()>{
        let user = self.connection.exec_map(
            r"SELECT username, role, login_session, email, id, password FROM users WHERE username=:uoe OR email=:uoe",
            params! {
                "uoe" => username_or_email,
            },
            | (username, role, login_session,email, id, password): (String,String,String,String,i32, String) | {
                let role = Role::from_str(role.as_str()).unwrap_or(Role::User);
                User{
                username,
                role,
                login_session,
                email,
                id,
                password
                }
            },
        ).unwrap();
        match user.into_iter().next(){
            Some(u) => Ok(u),
            _ => Err(())
        }
    }

    pub fn put_user(&mut self, user: SetUser) -> Result<Vec<String>,mysql::Error>{
        self.connection.exec(r"INSERT INTO users (username, role, password, email, login_session)
            VALUES (:username, :role, :password, :email, :login_session)",
            params! {
                "username" => user.username,
                "role" => Role::User.to_string(),
                "password" => user.password,
                "email" => user.email,
                "login_session" => String::new(),
            })
    }

    pub fn update_login_session(&mut self, username: String, login_session: String) -> Result<Vec<String>,mysql::Error>{
        self.connection.exec(r"UPDATE users
            SET login_session=:login_session WHERE username=:username LIMIT 1",
            params! {
                "login_session" => login_session,
                "username" => username
            })
    }

    pub fn get_user_by_username(&mut self, name: String) -> Result<GetUser, ()> {
        let user = self.connection.query_map(
            format!("SELECT username, role, login_session FROM users WHERE username = \"{}\"", name),
            |(username, role, login_session): (String,String, String) | {
                let role = Role::from_str(role.as_str()).unwrap_or(Role::User);
                GetUser{
                username,
                role,
                login_session
                }
            }).unwrap();

        match user.into_iter().next(){
            Some(user) => Ok(user),
            _ => Err(())
        }
    }

}
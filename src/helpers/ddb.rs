use mysql::{Pool, PooledConn, params, from_row};
use mysql::prelude::*;

use crate::models::task::Task;


pub struct DB{
    pub db_name: String,
    conn_string: String,
    conn_pool: Pool,
    connection: PooledConn
}

impl DB{
    pub fn new(db_user: String, db_pass: String, db_port: String, db_name: String) -> Self{
        let conn_string = format!("mysql://{}:{}@localhost:{}/{}", db_user, db_pass, db_port, db_name);
        let conn_pool = Pool::new(conn_string.as_str()).unwrap();
        let mut connection = conn_pool.get_conn().unwrap();

        connection.query_drop(
            r"CREATE TABLE IF NOT EXISTS tasks (
                `id` INT NOT NULL AUTO_INCREMENT,
                `name` VARCHAR(200) NOT NULL,
                `variables` JSON NULL,
                `functions` JSON NULL,
                `tasks` LONGTEXT NOT NULL,
                `params` TEXT NULL,
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


    pub fn get_task(&mut self, name: String){
        let loaded_payments = self.connection.exec_map(
            format!("SELECT name, variables FROM tasks WHERE name = \"{}\"", name),
            (),
            | (n, v): (String,String) | format!("name: {}, variables: {}", n, v),
        ).unwrap();
        println!("{:?}", loaded_payments);
    }
}

// fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
//     let url = "mysql://root:password@localhost:3307/db_name";
//     let pool = Pool::new(url)?;

//     let mut conn = pool.get_conn()?;

//     // Let's create a table for payments.
//     conn.query_drop(
//         r"CREATE TEMPORARY TABLE payment (
//             customer_id int not null,
//             amount int not null,
//             account_name text
//         )")?;

//     let payments = vec![
//         Payment { customer_id: 1, amount: 2, account_name: None },
//         Payment { customer_id: 3, amount: 4, account_name: Some("foo".into()) },
//         Payment { customer_id: 5, amount: 6, account_name: None },
//         Payment { customer_id: 7, amount: 8, account_name: None },
//         Payment { customer_id: 9, amount: 10, account_name: Some("bar".into()) },
//     ];

//     // Now let's insert payments to the database
//     conn.exec_batch(
//         r"INSERT INTO payment (customer_id, amount, account_name)
//           VALUES (:customer_id, :amount, :account_name)",
//         payments.iter().map(|p| params! {
//             "customer_id" => p.customer_id,
//             "amount" => p.amount,
//             "account_name" => &p.account_name,
//         })
//     )?;

//     // Let's select payments from database. Type inference should do the trick here.
//     let selected_payments = conn
//         .query_map(
//             "SELECT customer_id, amount, account_name from payment",
//             |(customer_id, amount, account_name)| {
//                 Payment { customer_id, amount, account_name }
//             },
//         )?;

//     // Let's make sure, that `payments` equals to `selected_payments`.
//     // Mysql gives no guaranties on order of returned rows
//     // without `ORDER BY`, so assume we are lucky.
//     assert_eq!(payments, selected_payments);
//     println!("Yay!");

//     Ok(())
// }
use std::{str::FromStr, fmt, sync::Mutex};

use actix_web::web::Data;
use serde::{Serialize, Deserialize};
use bcrypt::{hash, verify, DEFAULT_COST};

use crate::{helpers::ddb::DB, controllers::user::UserError};

use super::user_token::UserToken;

#[derive(Serialize, Deserialize, Debug)]
pub enum Role {
    User,
    Member,
    Admin
}

//from str to enum
impl FromStr for Role {

    type Err = ();

    fn from_str(input: &str) -> Result<Role, Self::Err> {
        match input {
            "User"  => Ok(Role::User),
            "Member"  => Ok(Role::Member),
            "Admin"  => Ok(Role::Admin),
            _      => Err(()),
        }
    }
}

//from enum to string
impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub role: Role,
    pub password: String,
    pub email: String,
    pub login_session: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SetUser {
    pub username: String,
    pub password: String,
    pub email: String,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct GetUser {
    pub username: String,
    pub role: Role,
    pub login_session: String,
}

pub struct LoginDTO{
    pub username: String,
    pub email: String,
    pub password: String,
    pub login_session:String,
}


impl User {
        pub fn signup(user: SetUser, db: &Data<Mutex<DB>>) -> Result<String, String> {
            let mut db_aquire = db.lock().unwrap();
            if db_aquire.get_user_by_username(user.username.clone()).is_err() {
                let hashed_pwd = hash(&user.password, DEFAULT_COST).unwrap();
                let user = SetUser {
                    password: hashed_pwd,
                    ..user
                };
                match db_aquire.put_user(user) {
                    Ok(u) => Ok(UserError::UserCreationFailure.to_string()),
                    Err(_) => Err(format!("Problem registering user"))
                }
            } else {
                Err(format!("User '{}' is already registered", &user.username))
            }
        }

        pub fn is_valid_login_session(user_token: &UserToken, db: &Data<Mutex<DB>>) -> bool {
            let user = db.lock().unwrap().get_user_by_username(user_token.user.clone()).unwrap();
            if user.login_session == user_token.login_session{
                return true;
            }
            false
        }
}

// impl User {

//     pub fn login(login: LoginDTO, conn: &Connection) -> Option<LoginInfoDTO> {
//         if let Ok(user_to_verify) = users
//             .filter(username.eq(&login.username_or_email))
//             .or_filter(email.eq(&login.username_or_email))
//             .get_result::<User>(conn)
//         {
//             if !user_to_verify.password.is_empty()
//                 && verify(&login.password, &user_to_verify.password).unwrap()
//             {
//                 if let Some(login_history) = LoginHistory::create(&user_to_verify.username, conn) {
//                     if LoginHistory::save_login_history(login_history, conn).is_err() {
//                         return None;
//                     }
//                     let login_session_str = User::generate_login_session();
//                     if User::update_login_session_to_db(
//                         &user_to_verify.username,
//                         &login_session_str,
//                         conn,
//                     ) {
//                         return Some(LoginInfoDTO {
//                             username: user_to_verify.username,
//                             login_session: login_session_str,
//                         });
//                     }
//                 }
//             } else {
//                 return Some(LoginInfoDTO {
//                     username: user_to_verify.username,
//                     login_session: String::new(),
//                 });
//             }
//         }

//         None
//     }

//     pub fn logout(user_id: i32, conn: &Connection) {
//         if let Ok(user) = users.find(user_id).get_result::<User>(conn) {
//             Self::update_login_session_to_db(&user.username, "", conn);
//         }
//     }

//     pub fn is_valid_login_session(user_token: &UserToken, conn: &Connection) -> bool {
//         users
//             .filter(username.eq(&user_token.user))
//             .filter(login_session.eq(&user_token.login_session))
//             .get_result::<User>(conn)
//             .is_ok()
//     }

//     pub fn find_user_by_username(un: &str, conn: &Connection) -> QueryResult<User> {
//         users.filter(username.eq(un)).get_result::<User>(conn)
//     }

//     pub fn generate_login_session() -> String {
//         Uuid::new_v4().to_simple().to_string()
//     }

//     pub fn update_login_session_to_db(
//         un: &str,
//         login_session_str: &str,
//         conn: &Connection,
//     ) -> bool {
//         if let Ok(user) = User::find_user_by_username(un, conn) {
//             diesel::update(users.find(user.id))
//                 .set(login_session.eq(login_session_str.to_string()))
//                 .execute(conn)
//                 .is_ok()
//         } else {
//             false
//         }
//     }
// }
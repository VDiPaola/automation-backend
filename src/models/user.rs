use std::{str::FromStr, fmt, sync::Mutex};

use actix_web::web::Data;
use serde::{Serialize, Deserialize};
use bcrypt::{hash, verify, DEFAULT_COST};
use uuid::Uuid;

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


#[derive(Serialize, Deserialize)]
pub struct LoginDTO{
    pub username_or_email: String,
    pub password: String,
    pub login_session:String,
}


impl User {
        pub fn signup(user: SetUser, db: &Data<Mutex<DB>>) -> Result<Vec<String>, UserError> {
            let mut db_aquire = db.lock().unwrap();
            if db_aquire.get_user_by_username(user.username.clone()).is_err() {
                let hashed_pwd = hash(&user.password, DEFAULT_COST).unwrap();
                let user = SetUser {
                    password: hashed_pwd,
                    ..user
                };
                match db_aquire.put_user(user) {
                    Ok(u) => Ok(u),
                    Err(_) => Err(UserError::UserCreationFailure)
                }
            } else {
                Err(UserError::UserCreationFailure)
            }
        }

        pub fn is_valid_login_session(user_token: &UserToken, db: &Data<Mutex<DB>>) -> bool {
            let user = db.lock().unwrap().get_user_by_username(user_token.user.clone()).unwrap();
            if user.login_session == user_token.login_session{
                return true;
            }
            false
        }

        pub fn generate_login_session() -> String {
            Uuid::new_v4().simple().to_string()
        }

        pub fn logout(username:String, db: &Data<Mutex<DB>>) -> bool{
            let mut db_aquire = db.lock().unwrap();
            match db_aquire.update_login_session(username, String::new()){
                Ok(_) => return true,
                Err(_) => return false
            }
        }

        pub fn login(login: LoginDTO,db: &Data<Mutex<DB>>) -> Option<GetUser> {
            //check user exists
            let mut db_aquire = db.lock().unwrap();
            if let Ok(user_to_verify) = db_aquire.get_user_by_username_or_email(login.username_or_email)
            {
                //verify password
                if !user_to_verify.password.is_empty()
                    && verify(&login.password, &user_to_verify.password).unwrap()
                {
                    //generate login session
                    let login_session_str = User::generate_login_session();
                    if let Ok(_) = db_aquire.update_login_session(user_to_verify.username.clone(), login_session_str.clone()){
                        //valid login
                        return Some(GetUser {
                            username: user_to_verify.username,
                            login_session: login_session_str,
                            role: user_to_verify.role
                        });
                    }

                }
            }

            None
        }
        

}

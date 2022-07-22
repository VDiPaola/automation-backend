use std::{collections::HashMap, str::FromStr, fmt};

use serde::{Serialize, Deserialize};

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

#[derive(Serialize, Deserialize, Debug)]
pub struct SetUser {
    pub username: String,
    pub role: Role,
    pub password: String,
    pub email: String,
    
}


#[derive(Serialize, Deserialize, Debug)]
pub struct GetUser {
    pub username: String,
    pub role: Role
}
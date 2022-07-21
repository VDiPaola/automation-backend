use std::collections::HashMap;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SetTask {
    pub name: String,
    pub variables: String,
    pub functions: String,
    pub tasks: String,
    pub params: Vec<String>,
    pub has_encryption: bool,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct GetTask {
    pub name: String,
    pub params: Vec<String>,
    pub has_encryption: bool,
}
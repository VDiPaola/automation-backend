use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize)]
pub struct Task {
    pub name: String,
    pub variables: HashMap<String,String>,
    pub functions: HashMap<String,String>,
    pub tasks: Vec<String>,
    pub params: Vec<String>
}
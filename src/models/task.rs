use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct SetTask {
    pub name: String,
    pub variables: String,
    pub functions: String,
    pub tasks: String,
    pub params: Vec<String>
}

#[derive(Serialize, Debug)]
pub struct GetTask {
    pub name: String,
    pub params: Vec<String>
}
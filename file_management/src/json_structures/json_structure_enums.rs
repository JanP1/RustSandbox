use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ToDoList {
    pub id: i32,
    pub title: String,
    pub tasks: Vec<Task>
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub description: String,
    pub done: bool,
}

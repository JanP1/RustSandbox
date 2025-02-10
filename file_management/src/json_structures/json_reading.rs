use std::fs::File;
use std::io::Read;
use super::json_structure_enums::ToDoList;

pub fn read_todo_from_file(filename: &str) -> std::io::Result<ToDoList> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let todo_list: ToDoList = serde_json::from_str(&contents).expect("Invalid JSON format");
    Ok(todo_list)
}

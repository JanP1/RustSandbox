use file_management::json_structures::json_reading::read_todo_from_file;

fn main() {

    match read_todo_from_file("../test.json") {
        Ok(todo_list) => println!("{:#?}", todo_list),
        Err(e) => eprintln!("Error reading file {}", e),
    }
}

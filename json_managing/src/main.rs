use serde::Deserialize;
use std::fs::File;
use std::io::Read;

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct MyData {
    name: String,
    age: u32,
    // You can expand this struct depending on the actual JSON structure
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Path to your JSON file
    let file_path = "test.json";
    
    // Attempt to read the JSON file and handle errors if any
    match read_json_file(file_path) {
        Ok(data) => {
            // Print the data to the console (for now)
            println!("{:?}", data);
        }
        Err(e) => {
            // Print an error message and exit gracefully
            eprintln!("Error reading JSON file: {}", e);
        }
    }

    // Return Ok() to indicate the program finished successfully
    Ok(())
}

fn read_json_file(file_path: &str) -> Result<MyData, Box<dyn std::error::Error>> {
    // Open the file and handle errors
    let mut file = File::open(file_path).map_err(|e| {
        Box::new(e) as Box<dyn std::error::Error> // Box the error type for uniform handling
    })?;
    
    // Read the file into a string
    let mut contents = String::new();
    file.read_to_string(&mut contents).map_err(|e| {
        Box::new(e) as Box<dyn std::error::Error>
    })?;
    
    // Deserialize the JSON string into our MyData struct
    let data: MyData = serde_json::from_str(&contents).map_err(|e| {
        Box::new(e) as Box<dyn std::error::Error>
    })?;
    
    Ok(data)
}

use std::fs;

pub fn get_input_as_string() -> String {

    match fs::read_to_string("input.txt") {
        Err(_) => {
            println!("Unable to load file.");
            String::new()
        },
        Ok(data) => data,
    }

}
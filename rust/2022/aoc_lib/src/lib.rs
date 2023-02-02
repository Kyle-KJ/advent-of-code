use std::fs;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

pub fn get_input_as_string() -> String {

    match fs::read_to_string("input.txt") {
        Err(_) => {
            println!("Unable to load file.");
            String::new()
        },
        Ok(data) => data,
    }

}
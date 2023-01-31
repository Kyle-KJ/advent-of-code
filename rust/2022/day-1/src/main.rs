
use std::io::{BufReader, prelude::*};
use std::fs::File;
use std::convert::TryFrom;

fn main() {

    // Pseudocode:

    // Read input file
    // Create vector
    // Iterate line by line
    // Sum up values until empty newline
    // Upon newline / end of file, save sum into vector
    // Vector count = number of elves
    // Search vector for highest value

    // Implementation:

    // Open the file
    let file: File = File::open("input.txt")
        .expect("File not found.");
    
    // Create String object to hold data
    //let mut data = String::new();

    // Read file into String object
    //file.read_to_string(&mut data)
    //    .expect("Error reading file.");

    // Read line by line
    let buf_reader: BufReader<File> = BufReader::new(file);

    // Print extracted file contents
    // println!("{}", data);

    let mut calorie_vector: Vec<i32> = Vec::new();

    
    // Initialise sum counter
    let mut calorie_total: i32 = 0;

    // for each line in input
    for line in buf_reader.lines() {

        //println!("Line: {:?}", line);
        
        // if empty line, reset counter and push sum to vector
        let line_data: String = line.expect("Couldn't read line data.");
        //println!("Line data: {}", line_data);

        if line_data.eq("") {
            calorie_vector.push(calorie_total);
            calorie_total = 0;
        }
        else {
            // Convert line string into int
            let int_convert: i32 = line_data.parse().unwrap();
            calorie_total += int_convert;
        }

    }

    // Answer Part 1
    let vector_max: Option<&i32> = calorie_vector.iter().max();
    match vector_max {
        Some(max) => println!("Max value: {}", max),
        None => println!("Vector is empty"),
    }

    // Answer Part 2

    // Sum the three largest values from the vector

    // Sort the vector
    // Slice 3 highest values
    // Sum the values

    calorie_vector.sort();

    let vector_len: usize = calorie_vector.len();

    //let vector_len_int: i32 = usize::try_from(vector_len).unwrap();

    let vector_index: usize = vector_len - 3;

    let mut sum_val: i32 = 0;

    for val in &calorie_vector[vector_index..] {
        println!("Value: {}", val);
        sum_val += val;
    }

    println!("Sum of top 3: {}", sum_val);
}

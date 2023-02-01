use std::io::{BufReader, prelude::*};
use std::fs::File;

fn main() {
    // Day 2

    // Opponents Choices:
    // A = Rock, B = Paper, C = Scissors

    // My Choices:
    // X = Rock = 1, Y = Paper = 2, Z = Scissors = 3

    // Read file
    // For each line, determine outcome and score

    let file: File = File::open("input.txt").expect("Couldn't open file.");

    let buf_reader: BufReader<File> = BufReader::new(file);

    let mut total_points: u32 = 0;

    for line in buf_reader.lines() {
        
        let mut line_data: String = line.expect("Couldn't read line.");

        // line_data.trim();

        line_data.retain(|c| c != ' ');

        // Take first and last value from string

        //let opp_choice: char = &line_data.get(0).unwrap().expect("");
        //let my_choice: char = &line_data.get(1);

        let my_choice: char = line_data.pop().unwrap();
        let opp_choice: char = line_data.pop().unwrap();


        // Calculate winner / points

        total_points += rock_paper_scissors(opp_choice, my_choice);
        
    }

    println!("Total Points: {}", total_points);

}

fn rock_paper_scissors(opp_choice: char, my_choice: char) -> u32 {
    // Initialise points total
    let mut points: u32 = 0;

    match my_choice {
        'X' => {
            points += 1;
            match opp_choice {
                'A' => points += 3, // Draw
                'B' => (),          // Loss
                'C' => points += 6, // Win
                _ => ()
            }
        },
        'Y' => {
            points += 2;
            match opp_choice {
                'A' => points += 6, // Win
                'B' => points += 3, // Draw
                'C' => (),          // Loss
                _ => ()
            }
        },
        'Z' => {
            points += 3;
            match opp_choice {
                'A' => (),          // Loss
                'B' => points += 6, // Win
                'C' => points += 3, // Draw
                _ => ()
            }
        },
        _ => ()
    };

    points

}

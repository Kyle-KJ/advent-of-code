use std::io::{BufReader, prelude::*};
use std::fs::File;

fn main() {
    // Day 2

    // Opponents Choices:
    // A = Rock, B = Paper, C = Scissors

    // Read file
    let file: File = File::open("input.txt").expect("Couldn't open file.");
    let buf_reader: BufReader<File> = BufReader::new(file);

    // Initialise output variables
    let mut total_points_one: u32 = 0;
    let mut total_points_two: u32 = 0;

    for line in buf_reader.lines() {
        
        let mut line_data: String = line.expect("Couldn't read line.");

        // Remove spaces from string
        line_data.retain(|c| c != ' ');

        // Extract values from string
        let my_choice: char = line_data.pop().unwrap();
        let opp_choice: char = line_data.pop().unwrap();

        total_points_one += rock_paper_scissors_one(opp_choice, my_choice);
        total_points_two += rock_paper_scissors_two(opp_choice, my_choice);

    }

    println!("Total Points (Part 1): {}", total_points_one);
    println!("Total Points (Part 2): {}", total_points_two);

}

fn rock_paper_scissors_one(opp_choice: char, my_choice: char) -> u32 {
    
    // Part 1 Logic
    // X = Rock = 1, Y = Paper = 2, Z = Scissors = 3

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

fn rock_paper_scissors_two(opp_choice: char, my_choice: char) -> u32 {

    // Part 2 Logic
    // X = Lose, Y = Draw, Z = Win

    let mut points: u32 = 0;

    match my_choice {
        'X' => {    // Lose
            points += 0;
            match opp_choice {
                'A' => points += 3, // Scissors
                'B' => points += 1, // Rock
                'C' => points += 2, // Paper
                _ => ()
            }
        },
        'Y' => {    // Draw
            points += 3;
            match opp_choice {
                'A' => points += 1, // Rock
                'B' => points += 2, // Paper
                'C' => points += 3, // Scissors
                _ => ()
            }
        },
        'Z' => {    // Win
            points += 6;
            match opp_choice {
                'A' => points += 2, // Paper
                'B' => points += 3, // Scissors
                'C' => points += 1, // Rock
                _ => ()
            }
        },
        _ => ()
    };

    points

}
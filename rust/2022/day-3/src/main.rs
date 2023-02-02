// Day 3 - Rucksack Reorganization

// Part 1
// Each line (rucksack) must be split evenly into two parts
// Find which letter (case-sensitive) is repeated in both parts
// Convert letter into value (a-z: 1-26, A-Z: 27-52)
// Find sum of all values

// Part 2
// TBC

extern crate aoc_lib;

fn main() {

    let data: String = aoc_lib::get_input_as_string();

    let backpacks: Vec<&str> = data.split('\n').collect();

    let mut repeated_letters: Vec<char> = Vec::new();

    for backpack in backpacks.iter() {

        let half_index: usize = backpack.len() / 2;

        let (first, last) = backpack.split_at(half_index);

        //println!("First Half: {}", first);
        //println!("Last Half: {}", last);

        'forloop: for c in first.chars() {
            match last.find(c) {
                None => (),
                Some(v) => {
                    //let repeat_letter: char = first.chars().nth(v).unwrap();
                    let repeat_letter: char = match last.chars().nth(v) {
                        None => ' ',
                        Some(x) => x,
                    };
                    //println!("Repeat Letter: {}", repeat_letter);
                    repeated_letters.push(repeat_letter);
                    break 'forloop;
                }
            }
        }
    }

    //println!("Repeated letters: {:?}", repeated_letters);
    //println!("Backpack Count: {}", repeated_letters.iter().count());

    // Convert letters into values

    let mut total: u32 = 0;

    for letter in repeated_letters {
        let mut char_int: u32 = u32::from(letter);
        //println!("Letter: {}", letter);
        if letter.is_uppercase() {
            char_int -= 38;
        }
        else if letter.is_lowercase() {
            char_int -= 96;
        }
        //println!("Value: {}", char_int);
        total += char_int;
    }

    println!("Total Sum: {}", total);

}

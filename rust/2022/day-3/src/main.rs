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
        total += letter_to_value(letter);
    }

    println!("Total Sum: {}", total);


    // Part 2

    // Group backpacks into threes and search for a common letter

    let mut group_count: u8 = 0;

    let mut backpack_groups: Vec<Vec<&str>> = Vec::new();

    let mut backpack_group: Vec<&str> = Vec::new();

    for backpack in backpacks.iter() {
        group_count += 1;
        backpack_group.push(backpack);
        
        if group_count == 3 {
            //println!("Backpacks Grouped: {:?}", backpack_group);
            backpack_groups.push(backpack_group);
            backpack_group = Vec::new();
            group_count = 0;
        }
    }

    //println!("All Groups: {:?}", backpack_groups);

    //let mut repeated_group_letter: Vec<char> = Vec::new();

    let mut total_2: u32 = 0;

    for group in backpack_groups.iter() {
        'forloop2: for letter in group[0].chars() {
            match group[1].find(letter) {
                None => (),
                Some(_) => {
                    match group[2].find(letter) {
                        None => (),
                        Some(_) => {
                            total_2 += letter_to_value(letter);
                            break 'forloop2
                        }
                    }
                }
            }
        }
    }

    println!("Total Sum (pt 2): {}", total_2);

}

fn letter_to_value(letter: char) -> u32 {
    let mut char_int: u32 = u32::from(letter);
    //println!("Letter: {}", letter);
    
    // Convert letter to value by manipulating unicode number
    // a-z (1-26)
    if letter.is_lowercase() {
        char_int -= 96;
    }
    // A-Z (27-52)
    else if letter.is_uppercase() {
        char_int -= 38;
    };

    //println!("Value: {}", char_int);

    char_int
}
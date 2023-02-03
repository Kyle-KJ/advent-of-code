// Day 4
extern crate aoc_lib;

fn main() {
    
    let data: String = aoc_lib::get_input_as_string();

    let pairs: Vec<(&str, &str)> = data
                            .split('\n')
                            .filter_map(|line| {
                                line.split_once(",")
                            })
                            .collect();

    //println!("Pairs: {:?}", pairs);

    let mut full_overlaps: u32 = 0;
    let mut partial_overlaps: u32 = 0;

    for pair in pairs {
        let (l, r) = pair;
        let (l_low, l_high) = get_bounds(l);
        let (r_low, r_high) = get_bounds(r);

        if l_low >= r_low && l_high <= r_high {         // Left contained within Right
            full_overlaps += 1;
        }
        else if l_low <= r_low && l_high >= r_high {    // Right contained within Left
            full_overlaps += 1;
        }
        else if l_low <= r_low && l_high >= r_low {
            partial_overlaps += 1;
        }
        else if l_low <= r_high && l_high >= r_high {
            partial_overlaps += 1;
        } 

    }

    let total_overlaps: u32 = full_overlaps + partial_overlaps; 

    println!("Full Overlaps: {}", full_overlaps);
    println!("Partial Overlaps: {}", partial_overlaps);
    println!("Total Overlaps: {}", total_overlaps);
}


fn get_bounds(range: &str) -> (u32, u32) {

    let bounds: Vec<&str> = range.split('-').collect();

    let low = bounds[0].parse().unwrap();
    let high = bounds[1].parse().unwrap();

    (low, high)

}

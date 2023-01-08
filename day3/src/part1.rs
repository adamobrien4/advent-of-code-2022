use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

/**
 * Day 3 Advent of Code solution - Rust
 */

fn main() -> std::io::Result<()> {
    // Setup Variables for use in looping file contents
    // Compartment 1
    let mut comp1;
    // Compartment 2
    let mut comp2;

    let mut comp1_chars;
    let mut found_elements = Vec::<char>::new();

    // Fetch lines from fild
    if let Ok(lines) = read_lines("./src/rucksacks.txt") {
        // Loop each line in lines
        for line in lines {
            // Check no error reading line
            if let Ok(rucksack) = line {
                // Reset variables
                comp1_chars = Vec::new();

                // Split rucksack in middle
                comp1 = &rucksack[0..rucksack.len() / 2];
                comp2 = &rucksack[rucksack.len() / 2..];

                // Loop comp1 and add to comp1_chars vector
                for char in comp1.chars() {
                    comp1_chars.push(char);
                }

                // Loop comp2 and find char which is common
                for char in comp2.chars() {
                    if comp1_chars.contains(&char) {
                        found_elements.push(char);
                        break;
                    }
                }
            }
        }
    }

    println!("Found {} elements", found_elements.len());

    let mut total_priority_sum = 0;

    // Loop all common chars and calculate priority score
    for char in found_elements {
        print!("{} ", char);

        let mut val = char as i32 - 'A' as i32;

        println!("val: {}", val);

        // Setup priority score based on char
        if val > 26 {
            // Is Lowercase
            val -= 31;
        } else {
            // Is Uppercase
            val += 27;
        }

        // Add value to total priority
        total_priority_sum += val;
    }

    println!("--- Total Priority Sum: {} ---", total_priority_sum);

    return Ok(());
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    return Ok(io::BufReader::new(file).lines());
}

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

/**
 * Day 3 Advent of Code solution - Rust
 */

fn main() -> std::io::Result<()> {
    let mut found_elements = Vec::<char>::new();
    let mut common_elements = Vec::<char>::new();
    let mut temp_elements;

    let mut line_number = 0;

    // Fetch lines from fild
    if let Ok(lines) = read_lines("./src/rucksacks.txt") {
        // Loop each line in lines
        for line in lines {
            // Check no error reading line
            if let Ok(rucksack) = line {
                // Reset temp_elements vector
                temp_elements = Vec::new();

                println!("Line number: {}", line_number);

                if line_number % 3 == 0 {
                    // Push common element from last 3 elves if exists
                    if common_elements.len() > 0 {
                        found_elements.push(common_elements[0]);
                    }
                    common_elements = Vec::new();
                }

                for char in rucksack.chars() {
                    // Is first elf of new group
                    // Add all rucksack contents to common vector
                    if line_number % 3 == 0 {
                        common_elements.push(char);
                    } else {
                        // Is on 2nd or 3rd elf
                        if common_elements.contains(&char) {
                            println!("{} is common", char);
                            temp_elements.push(char);
                        }
                    }
                }

                if temp_elements.len() > 0 {
                    println!("Found elems in temp: {:?}", temp_elements);
                    common_elements = temp_elements.clone();
                }
            }
            line_number += 1;
        }
    }

    found_elements.push(common_elements[0]);

    println!("Found {:?} elements", found_elements.len());
    println!("Common elem is {:?}", found_elements);

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

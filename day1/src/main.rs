use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

/**
 * Day 1 Advent of Code solution - Rust
 */

fn main() -> std::io::Result<()> {
    // Vector to hold all elf total calories
    let mut elf_total_calories = Vec::new();

    // Fetch lines from fild
    if let Ok(lines) = read_lines("./src/elf-rations.txt") {
        // Counter to track current elf total
        let mut total = 0;

        // Loop each line in lines
        for line in lines {
            // Check no error reading line
            if let Ok(calorie) = line {
                // Is newline?
                if calorie == "" {
                    // Line break, next elf
                    println!("Elf had {} calories", total);
                    // Add total to calories vector
                    elf_total_calories.push(total);
                    // Reset total to 0 for next elf
                    total = 0;
                } else {
                    // Add to current elf's total calories
                    total += calorie.parse::<i32>().unwrap();
                }
            }
        }
    }

    // Sort elf calories (default: Asc)
    elf_total_calories.sort();
    // Make Descending
    elf_total_calories.reverse();

    println!(
        "\n --- Highest Calorie Count: {} ---",
        elf_total_calories[0]
    );

    println!(
        "\n --- Top 3 Elves Total: {} ---",
        elf_total_calories[0..3].iter().sum::<i32>()
    );
    return Ok(());
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    return Ok(io::BufReader::new(file).lines());
}

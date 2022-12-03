use std::io::{self, BufRead};
use std::{fs::File, path::Path};

/**
 * Day 1 Advent of Code solution - Rust
 *
 * X: Rock
 * Y: Paper
 * Z: Scissors
 *
 * Points for Round
 * Rock: 1
 * Paper: 2
 * Scissors: 3
 *
 * plus
 *
 * 0: Loss
 * 3: Draw
 * 6: Win
 *
 */

fn main() -> std::io::Result<()> {
    // Vector to hold round scores
    let mut round_scores = Vec::new();

    // Points given for using the related action (Rock, Paper, or Scissors)
    let action_points = [1, 2, 3];
    let action_names = ["Rock", "Paper", "Scissors"];
    let action_results = ["Loss", "Draw", "Win"];

    // Fetch lines from file
    if let Ok(lines) = read_lines("./src/strategy.txt") {
        // Loop each line in lines
        for line in lines {
            // Check no error reading line
            if let Ok(line) = line {
                let chars = line.chars();

                let mut opponent_play = 0;
                let mut outcome = 0;

                for char in chars {
                    if char == ' ' {
                        // Skip space character
                        continue;
                    }
                    if char > 'C' {
                        // Outcome
                        outcome = char as i32 - 'X' as i32;
                    } else {
                        // Opponent
                        opponent_play = char as i32 - 'A' as i32;
                    }
                }

                if let Ok(result) = calculate_rps_result(opponent_play, outcome) {
                    println!(
                        "Opponent: {}, Mine: {}, Result: {}",
                        action_names[opponent_play as usize],
                        action_names[result as usize],
                        action_results[outcome as usize]
                    );

                    // Points awarded for playing a particular action + Result * 3
                    // Loss 0 * 3, Draw 1 * 3, Win 2 * 3
                    round_scores.push(action_points[result as usize] + (outcome * 3));
                } else {
                    println!("Error in calculate_rps_result");
                }
            }
        }
    }

    println!("Round Scores Len: {}", round_scores.len());
    println!(
        "--- Total Points Scores: {}---",
        round_scores.iter().sum::<i32>()
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

fn calculate_rps_result(opponent: i32, outcome: i32) -> io::Result<i32> {
    // Returns: Action to play
    // 0: Rock
    // 1: Paper
    // 2: Scissors

    let result;

    if outcome == 0 {
        // Lose
        result = (opponent + 2) % 3;
    } else if outcome == 1 {
        // Draw
        result = opponent;
    } else {
        // Win
        result = (opponent + 1) % 3;
    }

    return Ok(result);
}

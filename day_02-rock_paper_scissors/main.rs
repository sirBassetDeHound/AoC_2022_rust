use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {
    let file = File::open("./input.txt")
        .expect("File not found");

    let buf_reader = BufReader::new(file);

    let mut game_variations: HashMap<&str, u32> = HashMap::new();

    // Elf wins - selection + 0
    game_variations.insert("A Z", 3); // Scissors 3 + 0
    game_variations.insert("B X", 1); // Rock 1 + 0
    game_variations.insert("C Y", 2); // Paper 2 + 0

    // User wins - selection + 6
    game_variations.insert("C X", 7); // Rock 1 + 6
    game_variations.insert("A Y", 8); // Paper 2 + 6
    game_variations.insert("B Z", 9); // Scissors 3 + 6

    // Game drawn - selection + 3
    game_variations.insert("A X", 4); // Rock 1 + 3
    game_variations.insert("B Y", 5); // Paper 2 + 3
    game_variations.insert("C Z", 6); // Scissors 3 + 3

    let mut total_points_solution1:u32 = 0;
    let mut total_points_solution2:u32 = 0;
    for line in buf_reader.lines() {
        let new_line = line.unwrap();
        match game_variations.get(&new_line as &str) {
            Some(&number) => {
                total_points_solution1 = total_points_solution1 + number;
            }
            None => {
                println!("Nothing found")
            }
        }
        let round_end: Vec<&str> = new_line.split(" ").collect();
        let mut elf_selection = String::new();
        for (index, el) in round_end.iter().enumerate() {
            if index == 0 {
                elf_selection = el.to_string();
            }
            if index == 1 {
                if el.to_string() == "X" {
                    // lose
                    if elf_selection == "A" {
                        total_points_solution2 = total_points_solution2 + 3
                    }
                    if elf_selection == "B" {
                        total_points_solution2 = total_points_solution2 + 1
                    }
                    if elf_selection == "C" {
                        total_points_solution2 = total_points_solution2 + 2
                    }
                }
                if el.to_string() == "Y" {
                    // draw
                    if elf_selection == "A" {
                        total_points_solution2 = total_points_solution2 + 4
                    }
                    if elf_selection == "B" {
                        total_points_solution2 = total_points_solution2 + 5
                    }
                    if elf_selection == "C" {
                        total_points_solution2 = total_points_solution2 + 6
                    }
                }
                if el.to_string() == "Z" {
                    // win
                    if elf_selection == "C" {
                        total_points_solution2 = total_points_solution2 + 7
                    }
                    if elf_selection == "B" {
                        total_points_solution2 = total_points_solution2 + 9
                    }
                    if elf_selection == "A" {
                        total_points_solution2 = total_points_solution2 + 8
                    }
                }
            }

        };
    }
    println!("Total points: {}", total_points_solution1); // 10404
    println!("New total score: {}", total_points_solution2); // 10334
}
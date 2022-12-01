use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() -> std::io::Result<()> {
    let file = File::open("./input.txt")
        .expect("File not found");

    let buf_reader = BufReader::new(file);

    let mut total:u32 = 0;
    let mut most_calories:u32 = 0;
    let mut second_most_calories:u32 = 0;
    let mut third_most_calories:u32 = 0;

    for line in buf_reader.lines() {
        let new_line = line.unwrap();
        if !new_line.is_empty() {
            total = total + new_line.parse::<u32>().unwrap();
        }else if total > 0 {
            if total > most_calories {
                second_most_calories = most_calories;
                third_most_calories = second_most_calories;
                most_calories = total;
            } else if total > second_most_calories {
                third_most_calories = second_most_calories;
                second_most_calories = total;
            }else if total > third_most_calories {
                third_most_calories = total;
            }
            total = 0;
        }
    }
    // Part 1
    println!("Part 1 - Elf with most calories");
    println!("{}", most_calories);
    println!("-------------------");

    // Part 2
    println!("Part 2 - Top 3 elves with most calories");
    println!("{}", most_calories);
    println!("{}", second_most_calories);
    println!("{}", third_most_calories);
    println!("Total Elves Calories:");
    println!("{}", most_calories + second_most_calories + third_most_calories);

    Ok(())

}
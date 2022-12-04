use std::fs::File;
use std::str;
use std::io::{BufReader, BufRead};

fn main() {
    let file = File::open("input.txt")
        .expect("File not found");

    let buf_reader = BufReader::new(file);

    let mut total_fully_contained:i32 = 0;
    let mut total_partially_contained:i32 = 0;
    for line in buf_reader.lines() {
        let new_line = line.unwrap();

        let split_lines: Vec<&str> = new_line.split(|c| c == '-' || c == ',').collect();

        if split_lines[1].parse::<i32>().unwrap() > split_lines[3].parse::<i32>().unwrap() {
            if split_lines[0].parse::<i32>().unwrap() <= split_lines[2].parse::<i32>().unwrap() {
                total_fully_contained = total_fully_contained + 1;
            } else if split_lines[0].parse::<i32>().unwrap() <= split_lines[3].parse::<i32>().unwrap() {
                total_partially_contained = total_partially_contained + 1;
            }
        } else if split_lines[1].parse::<i32>().unwrap() == split_lines[3].parse::<i32>().unwrap() {
            total_fully_contained = total_fully_contained + 1;
        } else if split_lines[2].parse::<i32>().unwrap() <= split_lines[0].parse::<i32>().unwrap() {
            total_fully_contained = total_fully_contained + 1;
        } else if split_lines[1].parse::<i32>().unwrap() == split_lines[2].parse::<i32>().unwrap() {
            total_partially_contained = total_partially_contained + 1;
        } else if split_lines[1].parse::<i32>().unwrap() <= split_lines[3].parse::<i32>().unwrap(){
            if split_lines[1].parse::<i32>().unwrap() >= split_lines[2].parse::<i32>().unwrap() {
                total_partially_contained = total_partially_contained + 1;
            }
        }
    }
    println!("Total fully contained: {}", total_fully_contained); // 494
    println!("Total partially contained: {}", total_partially_contained + total_fully_contained); // 833
}
use std::fs::File;
use std::str;
use std::io::{BufReader, BufRead};

fn main() {

    // let mut second_start_position = vec![
    //     vec!["Z", "N"],
    //     vec!["M", "C", "D"],
    //     vec!["P"],
    // ];

    get_initial_positions();
    get_second_positions();

}

fn get_initial_positions() {
    let file = File::open("input.txt")
        .expect("File not found");

    let buf_reader = BufReader::new(file);
    let mut start_position = vec![
        vec!["D", "H", "N", "Q", "T", "W", "V", "B"],
        vec!["D", "W", "B"],
        vec!["T", "S", "Q", "W", "J", "C"],
        vec!["F", "J", "R", "N", "Z", "T", "P"],
        vec!["G", "P", "V", "J", "M", "S", "T"],
        vec!["B", "W", "F", "T", "N"],
        vec!["B", "L", "D", "Q", "F", "H", "V", "N"],
        vec!["H", "P", "F", "R"],
        vec!["Z", "S", "M", "B", "L", "N", "P", "H"]
    ];

    for line in buf_reader.lines() {
        let new_line = line.unwrap();
        let split_line: Vec<&str> = new_line.split(" ").collect();
        let move_amount:usize = split_line[1].parse::<usize>().unwrap();
        let move_from:usize = split_line[3].parse::<usize>().unwrap();
        let move_to:usize = split_line[5].parse::<usize>().unwrap();

        let mut n = 1;
        while n <= move_amount {
            let move_from_length = start_position[move_from-1].len();
            let move_value = &start_position[move_from-1].remove(move_from_length-1);
            start_position[move_to-1].push(move_value);
            n += 1;
        }
    }

    println!("New start_position {:?}", start_position); // PSNRGBTFT
}

fn get_second_positions() {
    let file = File::open("input.txt")
        .expect("File not found");

    let buf_reader = BufReader::new(file);

    let mut second_start_position = vec![
        vec!["D", "H", "N", "Q", "T", "W", "V", "B"],
        vec!["D", "W", "B"],
        vec!["T", "S", "Q", "W", "J", "C"],
        vec!["F", "J", "R", "N", "Z", "T", "P"],
        vec!["G", "P", "V", "J", "M", "S", "T"],
        vec!["B", "W", "F", "T", "N"],
        vec!["B", "L", "D", "Q", "F", "H", "V", "N"],
        vec!["H", "P", "F", "R"],
        vec!["Z", "S", "M", "B", "L", "N", "P", "H"]
    ];

    for line in buf_reader.lines() {
        let new_line = line.unwrap();
        let split_line: Vec<&str> = new_line.split(" ").collect();
        let move_amount: usize = split_line[1].parse::<usize>().unwrap();
        let move_from: usize = split_line[3].parse::<usize>().unwrap();
        let move_to: usize = split_line[5].parse::<usize>().unwrap();

        let mut n = 1;
        let mut num = 1;
        let mut box_holder: Vec<&str> = Vec::new();
        while n <= move_amount {
            let move_from_length = second_start_position[move_from - 1].len();
            let move_value = &second_start_position[move_from - 1].remove(move_from_length - 1);
            box_holder.push(move_value);
            n += 1;
        }

        let box_holder_len = box_holder.len();
        while num <= box_holder_len {
            let move_from_length = box_holder.len();
            let move_value = box_holder.remove(move_from_length - 1);
            second_start_position[move_to - 1].push(move_value);
            num += 1
        }
    }

    println!("second_start_position {:?}", second_start_position); // BNTZFPMMW
}
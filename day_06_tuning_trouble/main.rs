use std::fs::File;
use std::str;
use std::io::{BufReader, BufRead};
use std::collections::HashSet;

fn main() {

    get_test_position_markers();

}

fn get_test_position_markers() {
    let file = File::open("input.txt")
        .expect("File not found");

    let buf_reader = BufReader::new(file);

    for line in buf_reader.lines() {
        let new_line = line.unwrap();

        let string_array = new_line.as_bytes()
            .chunks(1)
            .map(str::from_utf8)
            .collect::<Result<Vec<&str>, _>>()
            .unwrap();

        let mut start_slice:usize = 0;
        let mut end_slice:usize = 4;
        let mut start_message_slice:usize = 0;
        let mut end_message_slice:usize = 14;

        let mut found:usize = 0;
        let sliced_array = &string_array[start_slice..end_slice];
        let v: HashSet<_> = sliced_array.iter().cloned().collect();
        if v.len() != 4 {
            while end_slice <= string_array.len() && found == 0{
                start_slice += 1;
                end_slice += 1;
                let sliced_array = &string_array[start_slice..end_slice];
                let v: HashSet<_> = sliced_array.iter().cloned().collect();
                if v.len() == 4 {
                    found = end_slice;
                    println!("Found: {}", found); // 1965
                }
            }
        }

        let mut found_message:usize = 0;
        let sliced_message_array = &string_array[start_message_slice..end_message_slice];
        let v: HashSet<_> = sliced_message_array.iter().cloned().collect();
        if v.len() != 14 {
            while end_message_slice <= string_array.len() && found_message == 0{
                start_message_slice += 1;
                end_message_slice += 1;
                let sliced_message_array = &string_array[start_message_slice..end_message_slice];
                let v: HashSet<_> = sliced_message_array.iter().cloned().collect();
                if v.len() == 14 {
                    found_message = end_message_slice;
                    println!("Found message: {}", found_message); // 2773
                }
            }
        }
    }
}
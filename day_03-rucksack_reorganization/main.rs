use std::fs::File;
use std::str;
use std::io::{BufReader, BufRead};

fn main() {
    let file = File::open("./input.txt")
        .expect("File not found");

    let buf_reader = BufReader::new(file);

    let mut total_priority_size:u32 = 0;
    let mut total_elf_group_priority_size:u32 = 0;
    let mut current_elf_badge_group = Vec::new();
    for line in buf_reader.lines() {
        let new_line = line.unwrap();

        let line_len = &new_line.len();

        let rucksack_compartment_contents = new_line.as_bytes()
            .chunks(line_len/2)
            .map(str::from_utf8)
            .collect::<Result<Vec<&str>, _>>()
            .unwrap();

        let rucksack_second_compartment_contents = rucksack_compartment_contents[1].as_bytes()
            .chunks(1)
            .map(str::from_utf8)
            .collect::<Result<Vec<&str>, _>>()
            .unwrap();

        let mut item_found:&str = "";
        for item in rucksack_second_compartment_contents {
            if rucksack_compartment_contents[0].contains(&item) {
                item_found = &item;
            }
        }

        static ASCII_LOWER: [&str; 26] = [
            "a", "b", "c", "d", "e",
            "f", "g", "h", "i", "j",
            "k", "l", "m", "n", "o",
            "p", "q", "r", "s", "t",
            "u", "v", "w", "x", "y",
            "z",
        ];
        if ASCII_LOWER.contains(&item_found) {
            let index:usize = ASCII_LOWER.iter().position(|&r| &r == &item_found).unwrap();
            let index_position:u32 = index as u32;
            total_priority_size = total_priority_size + index_position + 1;
        } else {
            let index:usize = ASCII_LOWER.iter().position(|&r| &r == &item_found.to_lowercase()).unwrap();
            let index_position:u32 = index as u32;
            total_priority_size = total_priority_size + index_position + 27;
        }

        // Elf badge group
        current_elf_badge_group.push(new_line);
        if current_elf_badge_group.len() == 3 {
            current_elf_badge_group.sort_by(|a, b| b.len().cmp(&a.len()));

            let chars = current_elf_badge_group[0].as_bytes()
                .chunks(1)
                .map(str::from_utf8)
                .collect::<Result<Vec<&str>, _>>()
                .unwrap();

            let mut char_found:bool = false;
            let mut found_char_str:&str = "";
            for char in chars {
                if !char_found &&
                    current_elf_badge_group[1].contains(char) &&
                    current_elf_badge_group[2].contains(char) {
                        char_found = true;
                        found_char_str = char;
                    }
            }
            if ASCII_LOWER.contains(&found_char_str) {
                let index:usize = ASCII_LOWER.iter().position(|&r| &r == &found_char_str).unwrap();
                let index_position:u32 = index as u32;
                total_elf_group_priority_size = total_elf_group_priority_size + index_position + 1;
            } else {
                let index:usize = ASCII_LOWER.iter().position(|&r| &r == &found_char_str.to_lowercase()).unwrap();
                let index_position:u32 = index as u32;
                total_elf_group_priority_size = total_elf_group_priority_size + index_position + 27;
            }

            // Empty Vec array
            while current_elf_badge_group.len() > 0 {
                current_elf_badge_group.remove(0);
            }
        }
    }

    println!("-------------------------");
    println!("total_priority_size: {:?}", total_priority_size); // 7727
    println!("-------------------------");
    println!("total_priority_size: {:?}", total_elf_group_priority_size);  // 2609
}

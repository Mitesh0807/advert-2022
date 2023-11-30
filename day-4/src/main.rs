use std::fs::File;
use std::io::prelude::*;
fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let lines: Vec<&str> = contents.lines().collect();
    let length = lines.len();
    let mut count = 0;
    for i in 0..length {
        let line = lines[i];
        let parts: Vec<&str> = line.split(",").collect();
        let part1 = parts[0];
        let part2 = parts[1];
        let range_of_first_elf: Vec<&str> = part1.split("-").collect();
        let range_of_second_elf: Vec<&str> = part2.split("-").collect();
        let first_elf_start = range_of_first_elf[0].parse::<u32>().unwrap();
        let first_elf_end = range_of_first_elf[1].parse::<u32>().unwrap();
        let second_elf_start = range_of_second_elf[0].parse::<u32>().unwrap();
        let second_elf_end = range_of_second_elf[1].parse::<u32>().unwrap();
        // 7-10 , 11-13
        // 11-13 , 7-10
        if first_elf_start <= second_elf_start && first_elf_end >= second_elf_end && line != "" {
            count += 1;
            continue;
        }
        if first_elf_start >= second_elf_start && first_elf_end <= second_elf_end && line != "" {
            count += 1;
        }
    }
    println!("count: {}", count);
}

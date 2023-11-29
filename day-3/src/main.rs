use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let lines: Vec<&str> = contents.lines().collect();
    let mut priority_sum: u32 = 0;
    for line in lines {
        if line != "" {
            let length = line.len();
            let first_compatment = line.split_at(length / 2).0;
            let second_compatment = line.split_at(length / 2).1;
            let first_set: HashSet<char> = first_compatment.chars().collect();
            let second_set: HashSet<char> = second_compatment.chars().collect();
            let intersection: HashSet<char> =
                first_set.intersection(&second_set).copied().collect();
            let priority = intersection.iter().next().unwrap();
            let new_priority = match priority {
                'a'..='z' => *priority as u32 - 96,
                'A'..='Z' => *priority as u32 - 38,
                _ => 0,
            };
            priority_sum += new_priority;
        }
    }
    println!("{}", priority_sum);
}

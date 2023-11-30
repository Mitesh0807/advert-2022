use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let lines: Vec<&str> = contents.lines().collect();
    let length = lines.len();
    let mut priority_sum: u32 = 0;
    let mut prority_sum_of_badge: u32 = 0;
    for (index, line) in lines.clone().into_iter().enumerate() {
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
        // if index % 3 == 0 && index != 0 {
        //     let first_set: HashSet<char> = lines[index - 3].chars().collect();
        //     let second_set: HashSet<char> = lines[index - 2].chars().collect();
        //     let third_set: HashSet<char> = lines[index - 1].chars().collect();
        //     let intersection: HashSet<char> =
        //         first_set.intersection(&second_set).copied().collect();
        //     let intersection_2: HashSet<char> =
        //         intersection.intersection(&third_set).copied().collect();
        //     let priority = intersection_2.iter().next().unwrap();
        //     let new_priority = match priority {
        //         'a'..='z' => *priority as u32 - 96,
        //         'A'..='Z' => *priority as u32 - 38,
        //         _ => 0,
        //     };
        //     prority_sum_of_badge += new_priority;
        // }
    }
    let mut i = 0;
    while i < length {
        let first_set: HashSet<char> = lines[i].chars().collect();
        let second_set: HashSet<char> = lines[i + 1].chars().collect();
        let third_set: HashSet<char> = lines[i + 2].chars().collect();
        let intersection: HashSet<char> = first_set.intersection(&second_set).copied().collect();
        let intersection_2: HashSet<char> =
            intersection.intersection(&third_set).copied().collect();
        let priority = intersection_2.iter().next().unwrap();
        let new_priority = match priority {
            'a'..='z' => *priority as u32 - 96,
            'A'..='Z' => *priority as u32 - 38,
            _ => 0,
        };
        prority_sum_of_badge += new_priority;
        i += 3;
    }
    println!("{}", priority_sum);
    println!("{}", prority_sum_of_badge);
}

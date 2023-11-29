use std::fs::File;
use std::io::Read;
fn main() {
    let file = File::open("input.txt");
    let mut contents = String::new();
    match file {
        Ok(mut f) => {
            f.read_to_string(&mut contents).unwrap();
            let v: Vec<&str> = contents.split("\n").collect();
            let mut highest_calories = 0;
            let mut total_calories_per_elf = 0;
            let mut top_three_calories = [0, 0, 0];
            let mut u32_vec: Vec<u32> = Vec::new();
            for i in 0..v.len() {
                if v[i] == "" {
                    if total_calories_per_elf > highest_calories {
                        highest_calories = total_calories_per_elf;
                        top_three_calories[2] = top_three_calories[1];
                        top_three_calories[1] = top_three_calories[0];
                        top_three_calories[0] = total_calories_per_elf;
                    } else if total_calories_per_elf > top_three_calories[2] {
                        top_three_calories[2] = total_calories_per_elf;
                    } else if total_calories_per_elf > top_three_calories[1] {
                        top_three_calories[2] = top_three_calories[1];
                        top_three_calories[1] = total_calories_per_elf;
                    }
                    total_calories_per_elf = 0;
                } else {
                    u32_vec.push(v[i].parse::<u32>().unwrap());
                    total_calories_per_elf += v[i].parse::<u32>().unwrap();
                }
            }
            let top_three_calories_sum: u32 = top_three_calories.iter().sum();
            println!("{}", top_three_calories_sum);
            println!("{}", highest_calories);
        }
        Err(e) => {
            println!("{}", e);
        }
    }
}

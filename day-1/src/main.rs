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
            for i in 0..v.len() {
                if v[i] == "" {
                    if total_calories_per_elf > highest_calories {
                        highest_calories = total_calories_per_elf;
                    }
                    total_calories_per_elf = 0;
                } else {
                    total_calories_per_elf += v[i].parse::<u32>().unwrap();
                }
            }
            println!("{}", highest_calories);
        }
        Err(e) => {
            println!("{}", e);
        }
    }
}

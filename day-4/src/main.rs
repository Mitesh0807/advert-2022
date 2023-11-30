use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let lines: Vec<&str> = contents.lines().collect();
    let length = lines.len();
    println!(" length: {}", length);
}

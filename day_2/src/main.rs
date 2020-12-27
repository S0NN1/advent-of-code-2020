use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("day_2/files/input.txt").unwrap();
    let reader = BufReader::new(file);
    let array: Vec<String> = reader.lines().flatten().collect();
    let mut valid = 0;
    let mut valid2 = 0;
    for string in &array {
        let data = string.replace(" ", "-");
        let array2: Vec<&str> = data.split("-").collect();
        let min = array2[0].parse::<i32>().unwrap();
        let max = array2[1].parse::<i32>().unwrap();
        let letter = array2[2].chars().next().unwrap();
        let line = array2[3].parse::<String>().unwrap();
        let count = line.matches(letter).count() as i32;
        //part 1
        if count >= min && count <= max {
            valid += 1;
        }
        //part 2
        if line.chars().nth((min - 1) as usize).unwrap() == letter && line.chars().nth((max - 1) as usize).unwrap() != letter {
            valid2 += 1;
        } else if line.chars().nth((min - 1) as usize).unwrap() != letter && line.chars().nth((max - 1) as usize).unwrap() == letter {
            valid2 += 1;
        }
    }
    println!("{}", valid);
    println!("{}", valid2);
}
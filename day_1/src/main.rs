use std::io::{BufReader, BufRead};
use std::fs::File;
fn main() {
    let file = File::open("../files/input.txt").unwrap();
    let check = 2020;
    let mut solution = 0;
    let reader = BufReader::new(file);
    let array:Vec<i32> = reader.lines().flatten().map(|l| l.parse::<i32>()).flatten().collect();
    for num in &array {
        let other_num = check-num;
        if array.contains(&other_num) {
            solution = (check - num) * num;
            break;
        }
    }
    println!("{}", solution);
}
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("day_1/files/input.txt").unwrap();
    let check = 2020;
    let mut solution = 0;
    let mut solution2 = 0;
    let reader = BufReader::new(file);
    let array: Vec<i32> = reader.lines().flatten().map(|l| l.parse::<i32>()).flatten().collect();
    for num in &array {
        let other_num = check - num;
        //part 1
        if array.contains(&other_num) {
            solution = (check - num) * num;
            break;
        }
        //part 2
        for num2 in &array {
            let other_num2 = check - num - num2;
            if array.contains(&other_num2) {
                solution2 = num * num2 * other_num2;
                break;
            }
        }
    }
    println!("{}", solution);
    println!("{}", solution2);
}
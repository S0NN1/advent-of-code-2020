use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("day_3/files/input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut lines: Vec<String> = reader.lines().flatten().collect();
    let num_of_lines = lines.len();
    let mut i = 0;
    let mut trees: [i64;5] = [0;5];
    calculate_tree(num_of_lines, lines.as_mut_slice(), 1, 1, &mut trees, 0);
    calculate_tree(num_of_lines, lines.as_mut_slice(), 1, 3, &mut trees, 1);
    calculate_tree(num_of_lines, lines.as_mut_slice(), 1, 5, &mut trees, 2);
    calculate_tree(num_of_lines, lines.as_mut_slice(), 1, 7, &mut trees, 3);
    calculate_tree(num_of_lines, lines.as_mut_slice(), 2, 1, &mut trees, 4);
    let solution = trees[0]*trees[1]*trees[2]*trees[3]*trees[4];
    println!("{}", solution);
}

fn calculate_tree(num_of_lines: usize, lines: &mut [String], down: usize, right: i32, trees: &mut [i64], index: usize) {
    let mut num = num_of_lines;
    let mut i = 0;
    let mut current_position = 0;
    let mut next_position ;
    if down==2 { num = num_of_lines - 1}
    while i != num - down {
        next_position = current_position + right;
        let next_line = lines[i + down].parse::<String>().unwrap();
        let length = next_line.len() as i32;
        if next_position >= length {
            next_position = next_position - length;
        }
        let char = next_line.chars().nth(next_position as usize).unwrap();
        current_position = next_position;
        if char == '#' {
            trees[index] += 1;
        }
        i += down;
    }
}

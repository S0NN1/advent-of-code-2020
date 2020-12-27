use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("day_4/files/input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut valid = 0;
    let check = [
        "byr".to_string(),
        "iyr".to_string(),
        "eyr".to_string(),
        "hgt".to_string(),
        "hcl".to_string(),
        "ecl".to_string(),
        "pid".to_string()];
    let mut lines: Vec<_> = Vec::new();
    // divide lines removing spaces and replacing empty lines with "/"
    for line in reader.lines() {
        let temp = line.unwrap();
        if temp.contains(" ") {
            let it = temp.split_whitespace().to_owned();
            for g in it {
                lines.push(g.to_string());
            }
        } else if temp.contains(":") {
            lines.push(temp);
        } else {
            lines.push("/".to_string());
        }
    }
    let mut index = 0;
    let mut count = 0;
    let mut field = "kek";
    for line in &lines {
        if line != "/" {
            let it2 = line.split(':').to_owned();
            for f in it2 {
                if index % 2 == 0 {
                    if check.contains(&String::from(f)) {
                        field = f;
                    }
                } else {
                    if field == "byr" {
                        let byr = f.parse::<i32>().unwrap();
                        if byr >= 1920 && byr <= 2002 {
                            count += 1;
                        }
                    }
                    else if field == "iyr" {
                        let iyr = f.parse::<i32>().unwrap();
                        if iyr >= 2010 && iyr <= 2020 {
                            count += 1;
                        }
                    }
                    else if field == "eyr" {
                        let eyr = f.parse::<i32>().unwrap();
                        if eyr >= 2020 && eyr <= 2030 {
                            count += 1;
                        }
                    }
                    else if field == "hgt" {
                        if f.contains("cm") {
                            let num_cm = f.split('c').to_owned().next().unwrap().parse::<i32>().unwrap();
                            if num_cm >= 150 && num_cm <= 193 {
                                count += 1;
                            }
                        } else if f.contains("in") {
                            let num_in = f.split('i').to_owned().next().unwrap().parse::<i32>().unwrap();
                            if num_in >= 59 && num_in <= 76 {
                                count += 1;
                            }
                        }
                    }
                   else  if field == "hcl" {
                        let mut symbols = 0;
                        let mut first = true;
                       let check = "abcdef";
                       let check2 = "0123456789";
                        if f.chars().nth(0).unwrap() == '#' {
                            for c in f.chars() {
                                if c == '#' && first {
                                    first = false;
                                } else {
                                    if check.contains(c) || check2.contains(c) {
                                        symbols += 1;
                                    }
                                }
                            }
                            if symbols == 6 {
                                count += 1;
                            }
                        }
                    }
                    if field == "ecl" {
                        if f == "amb" || f == "blu" || f == "brn" || f == "gry" || f == "grn" || f == "hzl" || f == "oth" {
                            count += 1;
                        }
                    }
                    if field == "pid" {
                        let mut digits = 0;
                        for c in f.chars() {
                            if c.is_numeric() {
                                digits += 1;
                            }
                        }
                        if digits == 9 {
                            count += 1;
                        }
                    }
                }
                index += 1;
            }
        } else {
            if count >= 7 {
                valid += 1;
            }
            count = 0;
            index = 0;
        }
    }
    if count >= 7 {
        valid += 1;
    }
    println!("{}", valid);
    // let kek: Vec<_> = reader.lines().map(|line| line.unwrap())
    //     .map(|line| if line.contains(" ") {line.split_whitespace().next().unwrap().to_owned()} else { line.to_owned() }).collect();
}
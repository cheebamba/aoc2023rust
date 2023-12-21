use std::fs;

pub fn run() {
    let mut sum: i32 = 0;

    let input = fs::read_to_string("./src/input/day1.txt").expect("File not found");

    let lines = input.split("\n").collect::<Vec<&str>>();

    let mut num1: i32;
    let mut num2: i32;

    for row in lines {
        let rev = row.chars().rev().collect::<String>();
        num1 = return_first_digit(row);
        num2 = return_first_digit(rev.as_str());
        sum += format!("{num1}{num2}").parse::<i32>().unwrap();
    }

    println!("Day 1: {sum}");
}

fn return_first_digit(string: &str) -> i32 {
    for (_i, char) in string.chars().enumerate() {
        if char.is_numeric() {
            let digit = char.to_digit(10).unwrap() as i32;
            return digit;
        }
    }
    return 0;
}

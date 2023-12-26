use std::fs;

pub fn run() {
    day1();
    day1p2();
}

pub fn day1() {
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

pub fn day1p2() {
    let mut sum: u32 = 0;

    let input = fs::read_to_string("./src/input/day1.txt").expect("File not found");

    let lines = input.split("\n").collect::<Vec<&str>>();

    let mut num: u32;

    for row in lines {
        num = return_first_digit_string(row);
        sum += num;
    }
    println!("Day 1p2: {sum}");
}

fn return_first_digit_string(string: &str) -> u32 {
    let mut thisnumber = 0;
    let mut firstnumber = 0;
    let mut lastnumber = 0;
    for (i, _c) in string.chars().enumerate() {
        let right = &string[i..];
        match right {
            right if right.starts_with("one") => thisnumber = 1,
            right if right.starts_with("two") => thisnumber = 2,
            right if right.starts_with("three") => thisnumber = 3,
            right if right.starts_with("four") => thisnumber = 4,
            right if right.starts_with("five") => thisnumber = 5,
            right if right.starts_with("six") => thisnumber = 6,
            right if right.starts_with("seven") => thisnumber = 7,
            right if right.starts_with("eight") => thisnumber = 8,
            right if right.starts_with("nine") => thisnumber = 9,
            right if right.as_bytes()[0].is_ascii_digit() => {
                thisnumber = right.chars().nth(0).unwrap().to_digit(10).unwrap();
            }
            &_ => (),
        }
        if firstnumber == 0 {
            firstnumber = thisnumber;
        }
        lastnumber = thisnumber;
    }
    return firstnumber * 10 + lastnumber;

    // let re_string = Regex::new(r"[0-9]|one|two|three|four|five|six|seven|eight|nine");
    // let number_array = HashMap::from([
    //     ("one", 1),
    //     ("two", 2),
    //     ("three", 3),
    //     ("four", 4),
    //     ("five", 5),
    //     ("six", 6),
    //     ("seven", 7),
    //     ("eight", 8),
    //     ("nine", 9),
    // ]);
    // let mut first_number = 0;
    // let mut last_number = 0;
    // for number in re_string.expect("").captures_iter(string) {
    //     let number_string = number.get(0).unwrap().as_str();
    //     let is_digit: bool = number_string.parse::<i32>().is_ok();
    //     if first_number == 0 {
    //         if is_digit {
    //             first_number = number_string.parse::<i32>().unwrap();
    //         } else {
    //             first_number = *number_array.get(number_string).unwrap();
    //         }
    //     }
    //     if is_digit {
    //         last_number = number_string.parse::<i32>().unwrap();
    //     } else {
    //         last_number = *number_array.get(number_string).unwrap();
    //     }
    // }
    // let this_number = format!("{first_number}{last_number}")
    //     .parse::<i32>()
    //     .unwrap();
    // return this_number;
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

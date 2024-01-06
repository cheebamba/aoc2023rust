use regex::Regex;
use std::fs;

pub fn run() {
    day3();
    day3p2();
}

fn day3() {
    let input = fs::read_to_string("./src/input/day3.txt").expect("File not found");
    let lines = input.split("\n").collect::<Vec<&str>>();
    let lines_len = lines.len().to_string().parse::<usize>().unwrap();
    let re_symbols = Regex::new(r"[^.a-z0-9\n]").unwrap();
    let re_numbers = Regex::new(r"[0-9]+").unwrap();

    let mut prev_symbols: Vec<i32> = Vec::new();
    let mut mid_symbols: Vec<i32> = Vec::new();
    let mut next_symbols: Vec<i32> = Vec::new();
    let mut this_number: Vec<(i32, i32, i32)> = Vec::new();
    let mut index = 0 as usize;
    let mut _sum: i32 = 0;

    while index < lines_len {
        let line = lines[index];
        for symbol in re_symbols.captures_iter(line) {
            let this_symbol = symbol.get(0).unwrap().start() as i32;
            if index < 1 {
                mid_symbols.push(this_symbol);
            } else {
                next_symbols.push(this_symbol);
            }
        }

        if index > 0 {
            _sum += sum_for_numbers(
                index,
                &this_number,
                &prev_symbols,
                &next_symbols,
                &mid_symbols,
            );
            prev_symbols = mid_symbols.clone();
            mid_symbols = next_symbols.clone();
            next_symbols = vec![];
        }
        this_number = vec![];
        for number in re_numbers.captures_iter(line) {
            let start = number.get(0).unwrap().start() as i32;
            let end = number.get(0).unwrap().end() as i32;
            let value = number.get(0).unwrap().as_str().parse::<i32>().unwrap();
            this_number.push((start, end, value));
        }
        if index + 1 == lines_len {
            _sum += sum_for_numbers(
                index,
                &this_number,
                &prev_symbols,
                &next_symbols,
                &mid_symbols,
            );
        }
        index += 1;
    }
    println!("Day 3: {}", _sum);
}

fn day3p2() {
    let input = fs::read_to_string("./src/input/day3.txt").expect("File not found");
    let lines = input.split("\n").collect::<Vec<&str>>();
    let lines_len = lines.len().to_string().parse::<usize>().unwrap();
    let re_symbols = Regex::new(r"[^.a-z0-9\n]").unwrap();
    let re_numbers = Regex::new(r"[0-9]+").unwrap();

    let mut prev_numbers: Vec<(i32, i32, i32)> = Vec::new();
    let mut mid_numbers: Vec<(i32, i32, i32)> = Vec::new();
    let mut next_numbers: Vec<(i32, i32, i32)> = Vec::new();
    let mut this_symbol: Vec<i32> = Vec::new();
    let mut index = 0 as usize;
    let mut _sum: i32 = 0;

    while index < lines_len {
        let line = lines[index];
        for number in re_numbers.captures_iter(line) {
            let this_number = (
                number.get(0).unwrap().start() as i32,
                number.get(0).unwrap().end() as i32,
                number.get(0).unwrap().as_str().parse::<i32>().unwrap(),
            );
            if index < 1 {
                mid_numbers.push(this_number);
            } else {
                next_numbers.push(this_number);
            }
        }

        if index > 0 {
            _sum += lookfornumbers(
                index,
                &this_symbol,
                &prev_numbers,
                &next_numbers,
                &mid_numbers,
            );
            prev_numbers = mid_numbers.clone();
            mid_numbers = next_numbers.clone();
            next_numbers = vec![];
        }
        this_symbol = vec![];
        for symbol in re_symbols.captures_iter(line) {
            this_symbol.push(symbol.get(0).unwrap().start() as i32);
        }
        if index + 1 == lines_len {
            _sum += lookfornumbers(
                index,
                &this_symbol,
                &prev_numbers,
                &next_numbers,
                &mid_numbers,
            );
        }
        index += 1;
    }
    println!("Day 3p2: {}", _sum);
}

fn lookfornumbers(
    index: usize,
    this_symbol: &Vec<i32>,
    prev_numbers: &Vec<(i32, i32, i32)>,
    next_numbers: &Vec<(i32, i32, i32)>,
    mid_numbers: &Vec<(i32, i32, i32)>,
) -> i32 {
    let mut _sum = 0;
    let mut count = 0;
    for symbol in this_symbol {
        let mut product: i32 = 0;
        let position = symbol;
        if index > 1 {
            for number in prev_numbers {
                let value = number.2;
                if check_adjacent_row(&number.0, &number.1, &position) {
                    if product == 0 {
                        product = value;
                    } else {
                        product = product * value;
                    }
                    count += 1;
                };
            }
        }
        for number in next_numbers {
            let value = number.2;
            if check_adjacent_row(&number.0, &number.1, &position) {
                if product == 0 {
                    product = value;
                } else {
                    product = product * value;
                }
                count += 1;
            };
        }
        for number in mid_numbers {
            let value = number.2;
            if check_this_row(&number.0, &number.1, &position) {
                if product == 0 {
                    product = value;
                } else {
                    product = product * value;
                }
                count += 1;
            };
        }
        if count == 2 {
            _sum += product;
        }
        count = 0;
    }
    return _sum;
}

fn sum_for_numbers(
    index: usize,
    this_number: &Vec<(i32, i32, i32)>,
    prev_symbols: &Vec<i32>,
    next_symbols: &Vec<i32>,
    mid_symbols: &Vec<i32>,
) -> i32 {
    let mut _sum = 0;
    'outer: for number in this_number {
        let start = number.0;
        let end = number.1;
        let value = number.2;
        if index > 1 {
            for symbol in prev_symbols {
                if check_adjacent_row(&start, &end, symbol) {
                    _sum += value;
                    continue 'outer;
                };
            }
        }
        for symbol in next_symbols {
            if check_adjacent_row(&start, &end, symbol) {
                _sum += value;
                continue 'outer;
            };
        }
        for symbol in mid_symbols {
            if check_this_row(&start, &end, symbol) {
                _sum += value;
                continue 'outer;
            };
        }
    }
    return _sum;
}

fn check_adjacent_row(start: &i32, end: &i32, position: &i32) -> bool {
    if position <= end && *position >= start - 1 {
        return true;
    } else {
        false
    }
}

fn check_this_row(start: &i32, end: &i32, position: &i32) -> bool {
    if *position == start - 1 || position == end {
        return true;
    } else {
        false
    }
}

use regex::Regex;
use std::fs;

pub fn run() {
    day4();
    day4p2();
}

fn day4p2() {
    let input = fs::read_to_string("./src/input/day4.txt").expect("File not found");
    let lines = input.split("\n").collect::<Vec<&str>>();
    let re_numbers = Regex::new(r"[0-9]+").unwrap();
    let mut _sum = 0;
    let mut multipliers: Vec<u32> = vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1];

    for line in lines {
        let parts = line.split("|").collect::<Vec<&str>>();
        let without = parts[0].split(":").collect::<Vec<&str>>();
        let winning: Vec<_> = re_numbers
            .find_iter(without[1])
            .map(|m| m.as_str())
            .collect();
        let picked: Vec<&str> = re_numbers.find_iter(parts[1]).map(|m| m.as_str()).collect();
        let mut power: usize = 0;
        for number in picked {
            if winning.contains(&number) {
                power += 1;
            }
        }
        let mut iter: usize = 0;
        for _ in 0..multipliers[0] {
            while iter < power {
                iter += 1;
                multipliers[iter] += 1;
            }
            iter = 0;
        }
        _sum += multipliers[1];
        multipliers.remove(0);
        multipliers.push(1);
    }
    println!("Day 4 p2: {:?}", _sum);
}

fn day4() {
    let input = fs::read_to_string("./src/input/day4.txt").expect("File not found");
    let lines = input.split("\n").collect::<Vec<&str>>();
    let re_numbers = Regex::new(r"[0-9]+").unwrap();
    let base_power: i32 = 2;
    let mut _sum = 0;

    for line in lines {
        let parts = line.split("|").collect::<Vec<&str>>();
        let without = parts[0].split(":").collect::<Vec<&str>>();
        let winning: Vec<_> = re_numbers
            .find_iter(without[1])
            .map(|m| m.as_str())
            .collect();
        let picked: Vec<&str> = re_numbers.find_iter(parts[1]).map(|m| m.as_str()).collect();
        let mut power: u32 = 0;
        let mut _points = 0;
        for number in picked {
            if winning.contains(&number) {
                power += 1;
            }
        }
        if power > 0 {
            _points = base_power.pow(power - 1);
        }
        _sum += 1 * _points;
    }
    println!("Day 4: {:?}", _sum);
}

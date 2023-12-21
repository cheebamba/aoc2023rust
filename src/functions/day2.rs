use regex::Regex;
use std::fs;

pub fn run() {
    let input = fs::read_to_string("./src/input/day2.txt").expect("File not found");
    let lines = input.split("\n").collect::<Vec<&str>>();
    let mut _product = 0;
    let mut id_sum = 0;

    let re_lines = Regex::new(r"Game (\d+): (.*)").unwrap();
    let re_round = Regex::new(r"([^;]*)").unwrap();
    let re_cubes = Regex::new(r"(\d+) ([a-z]*)").unwrap();
    for line in lines {
        let Some(caps) = re_lines.captures(line) else {
            return;
        };

        let id = caps.get(1).unwrap().as_str().parse::<i32>().unwrap();
        let game_string = caps.get(2).unwrap().as_str();

        let mut _conform = true;
        let mut highest_red = 0;
        let mut highest_green = 0;
        let mut highest_blue = 0;
        for (_, [round]) in re_round.captures_iter(game_string).map(|c| c.extract()) {
            let cubes = re_cubes.captures_iter(round);
            for cube in cubes {
                let color = cube.get(2).unwrap().as_str();
                let amount = cube.get(1).unwrap().as_str().parse::<i32>().unwrap();

                match color {
                    "red" => {
                        if amount > highest_red {
                            highest_red = amount
                        }
                        if amount > 12 {
                            _conform = false;
                        };
                    }
                    "green" => {
                        if amount > highest_green {
                            highest_green = amount
                        }
                        if amount > 13 {
                            _conform = false;
                        };
                    }
                    "blue" => {
                        if amount > highest_blue {
                            highest_blue = amount
                        }
                        if amount > 14 {
                            _conform = false;
                        };
                    }
                    _ => println!("Error"),
                }
            }
        }
        _product += highest_blue * highest_green * highest_red;
        if _conform {
            id_sum += id;
        }
    }
    println!("Day 2: {}", id_sum);
    println!("Day 2p2: {}", _product);
}

use std::fs;

pub fn run() {
    day5();
}

fn day5() {
    let input = fs::read_to_string("./src/input/day5.txt").expect("File not found");
    let segments = input.split("\n\n").collect::<Vec<&str>>();
    let seeds = return_seeds_vec(segments[0]);
    let seed_to_soil = return_map_vec(segments[1]);
    let soil_to_fertilizer = return_map_vec(segments[2]);
    let fertilizer_to_water = return_map_vec(segments[3]);
    let water_to_light = return_map_vec(segments[4]);
    let light_to_temperature = return_map_vec(segments[5]);
    let temperature_to_humidity = return_map_vec(segments[6]);
    let humidity_to_location = return_map_vec(segments[7]);
}

fn return_map_vec(string: &str) {
    let map_string = string.split("map:\n").collect::<Vec<&str>>()[1];
    let map_rows = map_string.split("\n").collect::<Vec<&str>>();
    let mut map_vec: Vec<(u32, u32, u32)> = Vec::new();

    for row in map_rows.iter() {
        let row_split = row.split(" ").collect::<Vec<&str>>();
        map_vec.push((
            row_split[0].parse::<u32>().unwrap(),
            row_split[1].parse::<u32>().unwrap(),
            row_split[2].parse::<u32>().unwrap(),
        ));
    }
}

fn return_seeds_vec(string: &str) {
    let seeds_string = &string[7..];

    dbg!(seeds_string);
}

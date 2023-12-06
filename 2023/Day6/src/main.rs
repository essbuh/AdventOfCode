use std::{cmp::max, time::SystemTime};

struct RaceResult {
    time: i64,
    distance: i64,
}

fn get_distance_traveled(time_allowed: i64, time_held: i64) -> i64 {
    let speed_per_sec = 1;
    let speed = speed_per_sec * time_held;

    let time_moving = max(time_allowed - time_held, 0);
    time_moving * speed
}

fn get_values_2(line: &str) -> Vec<i64> {
    line.split_once(':').unwrap().1
        .trim().split(' ').into_iter()
        .filter(|v| !v.is_empty())
        .map(|v| v.parse::<i64>().unwrap())
        .collect()
}

fn get_values(line: &str, remove_spaces: bool) -> Vec<i64> {
    if remove_spaces {
        let without_spaces = line.replace(" ", "");
        get_values_2(&without_spaces)
    } else {
        get_values_2(line)
    }
}

fn parse_results(input: &str, remove_spaces: bool) -> Vec<RaceResult> {
    let lines : Vec<&str> = input.lines().collect();
    let times = get_values(lines[0], remove_spaces);
    let distances = get_values(lines[1], remove_spaces);

    let mut result = Vec::new();
    result.reserve(times.len());

    for i in 0..times.len() {
        result.push(RaceResult { time: times[i], distance: distances[i] });
    }

    result
}

fn get_ways_to_beat(result: &RaceResult) -> usize {
    (1..result.time).map(|time| get_distance_traveled(result.time, time))
        .filter(|time| time > &result.distance)
        .count()
}

fn get_num_ways_to_beat(results: &Vec<RaceResult>) -> usize {
    results.iter()
        .map(|result| get_ways_to_beat(result))
        .reduce(|a, b| a * b)
        .unwrap_or(0)
}

fn part_1(input: &str) {
    let now = SystemTime::now();        
    let results = parse_results(input, false);
    println!("Finished parsing results in {} ms", (now.elapsed().unwrap().as_micros() as f32 / 1000.0));

    let now = SystemTime::now();        
    let num_ways_to_beat = get_num_ways_to_beat(&results);
    println!("Part 1: {num_ways_to_beat} | took {} ms", (now.elapsed().unwrap().as_micros() as f32 / 1000.0));
}

fn part_2(input: &str) {
    let now = SystemTime::now();        
    let results = parse_results(input, true);
    println!("Finished parsing results in {} ms", (now.elapsed().unwrap().as_micros() as f32 / 1000.0));
    
    let now = SystemTime::now();        
    let num_ways_to_beat = get_num_ways_to_beat(&results);
    println!("Part 2: {num_ways_to_beat} | took {} ms", (now.elapsed().unwrap().as_micros() as f32 / 1000.0));
}

fn main() {
    let input = include_str!("input.txt");
    part_1(&input);
    part_2(&input);
}
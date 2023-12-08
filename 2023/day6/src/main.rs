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

fn get_num_ways_to_beat(results: &Vec<RaceResult>, use_quadratic: bool) -> i64 {
    if use_quadratic {
        results.iter()
            .map(|r| solve_quadratic(r.time, r.distance))
            .filter(|r| r.is_some())
            .map(|r| {
                let (a, b) = r.unwrap();
                (b - a) + 1
            })
            .reduce(|a, b| a * b)
            .unwrap_or(0) as i64
    } else {
        results.iter()
            .map(|result| get_ways_to_beat(result))
            .reduce(|a, b| a * b)
            .unwrap_or(0) as i64
    }
}

fn solve_quadratic(max_time: i64, best_distance: i64) -> Option<(i64, i64)> {
    // equation = x^2 - x(max_time) + best_distance
    // a = 1, b = max_time, c = best_distance
    let a = 1.0;
    let b = -max_time as f32;
    let c = (best_distance + 1) as f32; // add one because we need to beat not match!

    let root = b*b - 4.0*a*c;
    if root > 0.0 {
        let root = root.sqrt();
        let min = (-b - root) / (2.0 * a);
        let max = (-b + root) / (2.0 * a);
        Some((min.ceil() as i64, max.floor() as i64))
    } else {
        None
    }
}

fn part_1(input: &str) {
    let now = SystemTime::now();        
    let results = parse_results(input, false);
    println!("Finished parsing results in {} ms", (now.elapsed().unwrap().as_nanos() as f32 / 1000000.0));

    let now = SystemTime::now();        
    let num_ways_to_beat = get_num_ways_to_beat(&results, false);
    println!("Part 1: {num_ways_to_beat} | took {} ms (Brute Force)", (now.elapsed().unwrap().as_nanos() as f32 / 1000000.0));

    let now = SystemTime::now();        
    let num_ways_to_beat = get_num_ways_to_beat(&results, true);
    println!("Part 1: {num_ways_to_beat} | took {} ms (Quadratic)", (now.elapsed().unwrap().as_nanos() as f32 / 1000000.0));
}

fn part_2(input: &str) {
    let now = SystemTime::now();        
    let results = parse_results(input, true);
    println!("Finished parsing results in {} ms", (now.elapsed().unwrap().as_nanos() as f32 / 1000000.0));
    
    let now = SystemTime::now();        
    let num_ways_to_beat = get_num_ways_to_beat(&results, false);
    println!("Part 2: {num_ways_to_beat} | took {} ms (Brute Force)", (now.elapsed().unwrap().as_nanos() as f32 / 1000000.0));

    let now = SystemTime::now();        
    let num_ways_to_beat = get_num_ways_to_beat(&results, true);
    println!("Part 2: {num_ways_to_beat} | took {} ms (Quadratic)", (now.elapsed().unwrap().as_nanos() as f32 / 1000000.0));
}

fn main() {
    let input = include_str!("input.txt");
    part_1(&input);
    part_2(&input);
}

use std::cmp::max;

struct DiceCounts
{
    red: i32,
    green: i32,
    blue: i32
}

impl DiceCounts {
    fn new() -> DiceCounts {
        DiceCounts { red: 0, green: 0, blue: 0 }
    }
}

fn add_dice_counts(a: &DiceCounts, b: &DiceCounts) -> DiceCounts {
    DiceCounts {
        red: a.red + b.red,
        green: a.green + b.green,
        blue: a.blue + b.blue
    }
}

fn max_dice_counts(a: &DiceCounts, b: &DiceCounts) -> DiceCounts {
    DiceCounts {
        red: max(a.red, b.red),
        green: max(a.green, b.green),
        blue: max(a.blue, b.blue)
    }
}

fn is_roll_result_valid(dice_limits: &DiceCounts, roll_counts: &DiceCounts) -> bool {
    roll_counts.red <= dice_limits.red
        && roll_counts.green <= dice_limits.green
        && roll_counts.blue <= dice_limits.blue
}

fn parse_dice_count(line: &str) -> DiceCounts {
    let mut counts = DiceCounts::new();

    let dice_count: Vec<&str> = line.split(' ').collect();
    match dice_count[1] {
        "red" => counts.red += dice_count[0].parse::<i32>().expect("Invalid input"),
        "green" => counts.green += dice_count[0].parse::<i32>().expect("Invalid input"),
        "blue" => counts.blue += dice_count[0].parse::<i32>().expect("Invalid input"),
        _ => {},
    }

    counts
}

fn parse_single_result(line: &str) -> DiceCounts {
    line.split(',')
        .map(|x| parse_dice_count(x.trim()))
        .fold(DiceCounts::new(),
              |acc, dice| add_dice_counts(&acc, &dice))
}

fn is_illegal_roll(line: &str, dice_limits: &DiceCounts) -> bool {
    let result = parse_single_result(line);
    let is_valid = is_roll_result_valid(dice_limits, &result);

    !is_valid
}

fn get_id_if_valid(line: &str, dice_limits: &DiceCounts) -> i32 {
    let (roll_id, roll_results) = line.split_once(':')
        .expect("Line was not in proper format: {line}");

    let roll_id = roll_id.split_whitespace()
        .map(|x| x.parse::<i32>())
        .filter(|x| x.is_ok())
        .next().expect("Did not find a valid roll ID")
        .expect("Did not find a valid roll ID");

    let has_invalid_roll = roll_results.split(';')
        .any(|x| is_illegal_roll(x.trim(), dice_limits));

    if has_invalid_roll {
        0
    } else {
        roll_id
    }
}

fn get_result_1(input: &str, dice_limits: &DiceCounts) -> i32 {
    let result = input.lines()
        .map(|x| get_id_if_valid(x, &dice_limits))
        .sum();
    result
}

fn get_dice_power(line: &str) -> i32 {
    let (_, roll_results) = line.split_once(':')
        .expect("Line was not in proper format: {line}");

    let max_dices = roll_results.split(';')
        .map(|x| parse_single_result(x))
        .fold(DiceCounts::new(),
              |acc, dice| max_dice_counts(&acc, &dice));


    max_dices.red * max_dices.blue * max_dices.green
}

fn get_result_2(input: &str) -> i32 {
    input.lines()
        .map(|x| get_dice_power(x))
        .sum()
}

fn main() {
    let input = include_str!("input.txt");
    let dice_limits_1 = DiceCounts { red: 12, blue: 14, green: 13 };
    let result_1 = get_result_1(input, &dice_limits_1);
    println!("Result 1: {result_1}");

    let result_2 = get_result_2(input);
    println!("Result 2: {result_2}");
}

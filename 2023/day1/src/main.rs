use std::{env, fs};

fn parse_digit_word(line: &str) -> i32 {
    let numbers = ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

    for (i, name) in numbers.iter().enumerate() {
        if line.starts_with(name) {
            return i as i32;
        }
    }
    -1
}

fn parse_digits(contents: &str, include_digit_names: bool) -> i32 {
    let mut result = 0;

    for line in contents.lines() {
        //println!("Line: {line}");
        let mut first_digit = -1;
        let mut last_digit = -1;

        for (i, char) in line.chars().enumerate() {
            //println!(" + i: {i}, Char: {char}");

            let mut digit = -1;
            if char.is_numeric() {
                digit = char as i32 - 0x30;
                //println!("  + Found digit {digit}")
            } else if include_digit_names {
                let digit_word = &line[i..];
                //println!("  + Digit Word: {digit_word}");
                digit = parse_digit_word(digit_word);
                //println!("    + Parsed: {digit}");
            }

            if digit != -1 {
                if first_digit == -1 {
                    first_digit = digit;
                }

                last_digit = digit;
            }
        }

        //println!("Line: {line} | First: {first_digit}, Last: {last_digit}");
        if first_digit != -1 {
            assert_ne!(last_digit, -1);
            let value = (first_digit * 10) + last_digit;
            result += value;
        }
    }

    result
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];
    let contents = fs::read_to_string(file_name)
        .expect("Should have read the file");

    {
        let result = parse_digits(contents.as_str(), false);
        println!("Result [Digits Only]: {result}");
    }
    {
        let result = parse_digits(contents.as_str(), true);
        println!("Result [Digit Names]: {result}");
    }
}

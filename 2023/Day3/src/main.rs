use std::collections::HashMap;

struct PartNumber
{
    number: i32,
    col: i32,
    len: i32
}
impl PartNumber {
    fn new(number: i32, col: i32, len: i32) -> PartNumber {
        PartNumber { number, col, len }
    }
}

#[derive(PartialEq, Eq, Hash)]
struct Symbol {
    symbol: char,
    col: i32
}

struct ParsedLine {
    parts: Vec<PartNumber>,
    symbols: Vec<Symbol>
}

// parse a line, getting all part numbers & control symbols
fn parse_line(line: &str) -> ParsedLine {
    let mut parsed_line : ParsedLine = ParsedLine { parts: Vec::new(), symbols: Vec::new() };

    let mut value = 0;
    let mut start_col = 0;
    let mut length = 0;

    for (i, char) in line.chars().enumerate() {
        if char.is_numeric() {
            let number = char as i32 - 0x30;
            start_col = if value == 0 { i as i32 } else { start_col };
            value = value * 10 + number;
            length = length + 1;

        } else {
            if length > 0 {
                let part = PartNumber::new(value, start_col, length);
                parsed_line.parts.push(part);

                value = 0;
                start_col = 0;
                length = 0;
            }

            // symbols!
            if char != '.' && !char.is_alphabetic() {
                parsed_line.symbols.push(Symbol { symbol:char, col:i as i32 })
            } else if char != '.' {
                println!("Skipping symbol {char}");
            }
        }
    }

    // if we were reading a digit, add it now
    if length > 0 {
        let part = PartNumber::new(value, start_col, length);
        parsed_line.parts.push(part);
    }

    parsed_line
}

fn has_matching_symbol(part: &PartNumber, symbols: &Vec<Symbol>) -> bool {
    let min = if part.col == 0 { 0 } else { part.col - 1 };
    let max = part.col + part.len;

    // Assumes cols in ascending order
    for symbol in symbols {
        if symbol.col <= max {
            if symbol.col >= min {
                return true;
            }
        } else {
            return false;
        }
    }

    false
}

fn get_valid_parts(parts: &Vec<PartNumber>, cols_above: &Vec<Symbol>, cols_same: &Vec<Symbol>, cols_below: &Vec<Symbol>) -> Vec<i32> {
    parts.iter()
        .filter(|x| has_matching_symbol(&x, &cols_above) || has_matching_symbol(&x, cols_same) || has_matching_symbol(&x, cols_below))
        .map(|x| x.number)
        .collect()
}

fn part_1(input: &str) {
    let lines: Vec<ParsedLine> = input.lines().map(|x| parse_line(x)).collect();

    let mut all_valid_parts : Vec<i32> = Vec::new();
        
    for i in 0..lines.len() {
        let parts = &lines[i].parts;
        let cols_same = &lines[i].symbols;
        let cols_above = if i > 0 { &lines[i - 1].symbols } else { cols_same };
        let cols_below = if i < (lines.len()-1) { &lines[i + 1].symbols } else { cols_same };

        let mut valid_parts = get_valid_parts(parts, cols_above, cols_same, cols_below);
        all_valid_parts.append(&mut valid_parts);
    }
    
    let result : i32 = all_valid_parts.iter().sum();
    println!("Result = {result}");
}

fn find_matching_parts<'a, 'b>(symbol: &'a Symbol, parts: &'b Vec<PartNumber>) -> Vec<&'b PartNumber> {
    let mut result : Vec<&PartNumber> = Vec::new();

    for part in parts {
        let min = if part.col == 0 { 0 } else { part.col - 1 };
        let max = part.col + part.len;

        if min <= symbol.col && max >= symbol.col {
            result.push(part);
        }
    }

    result
}

fn find_matching_part_numbers<'a, 'b>(symbol: &'a Symbol, parts_above: &'b Vec<PartNumber>, parts_same: &'b Vec<PartNumber>, parts_below: &'b Vec<PartNumber>) -> Vec<&'b PartNumber> {
    let mut result : Vec<&PartNumber> = Vec::new();

    result.append(&mut find_matching_parts(symbol, parts_above));
    result.append(&mut find_matching_parts(symbol, parts_same));
    result.append(&mut find_matching_parts(symbol, parts_below));

    result
}

fn part_2(input: &str) {
    let lines: Vec<ParsedLine> = input.lines().map(|x| parse_line(x)).collect();

    let mut sum = 0;

    for i in 0..lines.len() {
        let gears : Vec<&Symbol> = lines[i].symbols.iter().filter(|x| x.symbol == '*').collect();
            
        let empty_vec: Vec<PartNumber> = vec![];
        let parts_same = &lines[i].parts;
        let parts_above = if i > 0 { &lines[i - 1].parts } else { &empty_vec };
        let parts_below = if i < (lines.len()-1) { &lines[i + 1].parts } else { &empty_vec };

        let mut matches : HashMap<&Symbol, Vec<&PartNumber>> = HashMap::new();
        for gear in gears {
            matches.insert(gear, find_matching_part_numbers(gear, parts_above, parts_same, parts_below));
        }

        sum = sum + matches.iter()
            .filter(|x| x.1.len() == 2)
            .map(|x| x.1.iter().map(|x| x.number).fold(1, |acc, val| acc * val))
            .sum::<i32>();
    }

    println!("Result: {sum}");
}

fn main() {
    let input = include_str!("input.txt");
    part_1(input);

    //let input_2 = include_str!("part2_sample.txt");
    part_2(input);
}

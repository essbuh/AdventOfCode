use std::cmp::{min, max};

type Point = (i32, i32);

const CHAR_EMPTY : char = '.';
const CHAR_FILLED : char = '#';
const CHAR_UP : char = '^';
const CHAR_DOWN : char = 'v';

fn hex_to_dec(hex: &[char]) -> u32 {
    let mut result = 0;
    for char in hex {
        result *= 16;
        
        if char <= &'9' {
            result += (*char as u32) - ('0' as u32);
        } else if char <= &'f' {
            result += (*char as u32) - ('a' as u32) + 10;
        }
    }

    result
}

struct DigInstruction
{
    dir: char,
    count: u32,
}
struct DigPlan {
    instructions: Vec<DigInstruction>,
}
impl DigPlan {
    fn run_instructions(self: &Self, map: &mut DigMap) {    
        println!("Ensuring capacity...");    
        let mut pos = map.ensure_capacity(&self.instructions);

        map.dig_hole(&pos, CHAR_FILLED);

        println!("Running instructions...");
        for instruction in &self.instructions {
            map.run_instruction(&mut pos, instruction);
        }
    }
}
struct DigMap
{    
    map: Vec<Vec<char>>,
}
impl DigMap {
    fn ensure_capacity(self: &mut Self, instructions: &Vec<DigInstruction>) -> Point {
        let mut point : Point = (0, 0);
        let mut point_min : Point = (0, 0);
        let mut point_max : Point = (0, 0);
        for instruction in instructions {
            match instruction.dir {
                'R' => { 
                    point.0 += instruction.count as i32; 
                    point_min.0 = min(point_min.0, point.0);
                    point_max.0 = max(point_max.0, point.0);
                },
                'L' => { 
                    point.0 -= instruction.count as i32; 
                    point_min.0 = min(point_min.0, point.0);
                    point_max.0 = max(point_max.0, point.0);
                },
                'U' => { 
                    point.1 -= instruction.count as i32;
                    point_min.1 = min(point_min.1, point.1);
                    point_max.1 = max(point_max.1, point.1);
                 },
                'D' => { 
                    point.1 += instruction.count as i32;
                    point_min.1 = min(point_min.1, point.1);
                    point_max.1 = max(point_max.1, point.1);
                 },
                _ => {},
            }
        }

        let bounds = (point_max.0 - point_min.0, point_max.1 - point_min.1);
        println!("Bounds: ({}, {})", bounds.0, bounds.1);
        
        println!("Filling X...");
        let mut row = Vec::new();
        row.resize_with((bounds.0 + 1) as usize, || CHAR_EMPTY);

        println!("Filling Y...");
        self.map.resize_with((bounds.1 + 1) as usize, || row.clone());

        (-point_min.0, -point_min.1)
    }

    fn dig_hole(self: &mut Self, pos: &Point, val: char) {   
        self.map[pos.1 as usize][pos.0 as usize] = val;
    }

    fn run_instruction(self: &mut Self, pos: &mut Point, instruction: &DigInstruction) {        
        match instruction.dir {
            'R' => {
                for _ in 0..instruction.count {
                    pos.0 += 1;
                    self.dig_hole(&pos, CHAR_FILLED);
                }
            },
            'L' => {
                for _ in 0..instruction.count {
                    pos.0 -= 1;
                    self.dig_hole(&pos, CHAR_FILLED);
                }
            },
            'U' => {
                self.dig_hole(&pos, CHAR_UP);
                for _ in 0..instruction.count {
                    pos.1 -= 1;
                    self.dig_hole(&pos, CHAR_UP);
                }
            },
            'D' => {
                self.dig_hole(&pos, CHAR_DOWN);
                for _ in 0..instruction.count {
                    pos.1 += 1;
                    self.dig_hole(&pos, CHAR_DOWN);
                }
            }
            _ => { panic!("Unknown instruction"); }
        }
    }

    fn fill_interior(self: &mut Self) {
        println!("Filling...");
        for line in self.map.iter_mut() {
            let mut inside = false;
            for char in line.iter_mut() {
                match char {
                    '^' => { inside = true; },
                    'v' => { inside = false; },
                    '.' => { if inside { *char = '#' } },
                    _ => {}
                }
            }
        }
    }

    fn get_fill_size(self: &Self) -> usize {
        self.map.iter()
            .map(|row| row.into_iter().filter(|&c| c != &'.').count())
            .sum()
    }

    fn print(self: &mut Self) {
        /*println!(" -------- ");
        println!(" ");

        for line in &self.map {
            let chars : Vec<char> = line.iter().map(|c| if c == &'.' { *c } else { '#' }).collect();
            let s : String = chars.iter().collect();
            println!("{s}");
        }
    
        println!(" ");*/
    }
}

fn parse_instruction(line: &str, color_is_instruction: bool) -> DigInstruction {
    let parts : Vec<&str> = line.split(' ').collect();    

    if color_is_instruction {
        let color : Vec<char> = parts[2][2..8].to_string().chars().collect();
        let dir = match color.last().unwrap() {
            '0' => 'R',
            '1' => 'D',
            '2' => 'L',
            '3' => 'U',
            _ => panic!("Invalid input")
        };
        let count = hex_to_dec(&color[0..color.len()-1]);
        DigInstruction { dir, count }
    }
    else {
        let dir = parts[0].chars().last().unwrap();
        let count = parts[1].parse::<u32>().unwrap();
        DigInstruction { dir, count }
    }
}

fn parse_input(input: &str, color_is_instruction: bool) -> DigPlan {
    let mut map = DigPlan { instructions: Vec::new() };

    for line in input.lines() {
        map.instructions.push(parse_instruction(line, color_is_instruction));
    }

    map
}

fn run_part(input: &str, color_is_instruction: bool) -> (usize, usize) {
    let plan = parse_input(input, color_is_instruction);
    let mut map = DigMap { map: Vec::new() };
    
    plan.run_instructions(&mut map);
    map.print();
    let result_prefill = map.get_fill_size();

    map.fill_interior();
    map.print();
    let result_postfill = map.get_fill_size();

    (result_prefill, result_postfill)
}

fn part_1()
{
    let input = include_str!("input.txt");
    let (result_prefill, result_postfill) = run_part(input, false);
    
    println!("Part 1: {result_prefill} Pre-Fill, {result_postfill} Post-Fill");
}

fn part_2()
{
    let input = include_str!("sample.txt");
    let (result_prefill, result_postfill) = run_part(input, true);
    
    println!("Part 2: {result_prefill} Pre-Fill, {result_postfill} Post-Fill");
}

fn main() {
    part_1();
    //part_2();
}

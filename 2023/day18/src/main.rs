use std::cmp::{min, max};

type Point = (i32, i32);

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
        let mut pos = map.ensure_capacity(&self.instructions);

        map.dig_hole(&pos, CHAR_FILLED);

        for instruction in &self.instructions {
            map.run_instruction(&mut pos, instruction);
        }
    }
}
struct DigMap
{    
    map: Vec<Vec<(i32, char)>>,
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
        self.map.resize_with((bounds.1 + 1) as usize, || Vec::new());

        (-point_min.0, -point_min.1)
    }

    fn dig_hole(self: &mut Self, pos: &Point, val: char) {
        let row = &mut self.map[pos.1 as usize];

        let mut index = 0;
        for (i, col) in row.iter_mut().enumerate() {
            if col.0 == pos.0 {
                if col.1 == CHAR_FILLED {
                    col.1 = val;
                }
                return;
            }

            if col.0 > pos.0 {
                break;
            }

            index = i + 1;
        }

        row.insert(index, (pos.0, val));
    }

    fn run_instruction(self: &mut Self, pos: &mut Point, instruction: &DigInstruction) {        
        match instruction.dir {
            'R' => {
                pos.0 += instruction.count as i32;
                self.dig_hole(&(pos.0 - 1, pos.1), CHAR_FILLED);
            },
            'L' => {
                pos.0 -= instruction.count as i32;
                self.dig_hole(&(pos.0 + instruction.count as i32 - 1, pos.1), CHAR_FILLED);
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

    fn get_fill_size(self: &Self) -> usize {
        let mut fill_size : usize = 0;

        for (_i, row) in self.map.iter().enumerate() {
            let mut last_x = -1;

            let mut row_size = 0;
            for col in row.iter() {
                match col.1 {
                    CHAR_UP => {
                        row_size += 1;
                        last_x = col.0;
                    },
                    CHAR_DOWN => {
                        let fill_count = col.0 - last_x;
                        row_size += fill_count as usize;
                        last_x = col.0;
                    },
                    CHAR_FILLED => { 
                        let fill_count = col.0 - last_x;
                        row_size += fill_count as usize;
                        last_x = col.0;
                    }
                    _ => {}
                }
            }

            fill_size += row_size;
        }

        fill_size
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

fn run_part(input: &str, color_is_instruction: bool) -> usize {
    let plan = parse_input(input, color_is_instruction);
    let mut map = DigMap { map: Vec::new() };
    
    plan.run_instructions(&mut map);
    map.print();
    let result = map.get_fill_size();

    result
}

fn part_1()
{
    let input = include_str!("input.txt");
    let result = run_part(input, false);
    
    println!("Part 1: {result}");
}

fn part_2()
{
    let input = include_str!("input.txt");
    let result = run_part(input, true);
    
    println!("Part 2: {result}");
}

fn main() {
    part_1();
    part_2();
}

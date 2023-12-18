type Point = (i32, i32);

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
        let mut pos = (0,0);

        for instruction in &self.instructions {
            map.run_instruction(&mut pos, instruction);
        }

        assert_eq!(pos, (0,0));
    }
}
struct DigMap
{    
    map: Vec<Point>,
}
impl DigMap {
    fn dig_hole(self: &mut Self, pos: &Point) {
        self.map.push(*pos);
    }

    fn run_instruction(self: &mut Self, pos: &mut Point, instruction: &DigInstruction) {        
        match instruction.dir {
            'R' => { pos.0 += instruction.count as i32; },
            'L' => { pos.0 -= instruction.count as i32; },
            'U' => { pos.1 -= instruction.count as i32; },
            'D' => { pos.1 += instruction.count as i32; }
            _ => { panic!("Unknown instruction"); }
        }
        self.dig_hole(pos);
    }

    fn get_fill_size(self: &Self) -> i64 {
        let mut area : i64 = 0;
        let mut perimeter : i32 = 0;

        let map = &self.map;

        for i in 0..map.len()  {
            let next_i = if i == map.len()-1 { 0 } else { i+1 };

            let pt_a = map[i];
            let pt_b = map[next_i];

            let det = (pt_a.0 as i64 * pt_b.1 as i64) - (pt_b.0 as i64 * pt_a.1 as i64);
            area += det;

            perimeter += (pt_b.0 - pt_a.0).abs();
            perimeter += (pt_b.1 - pt_a.1).abs();
        }

        let numerator = (area.abs() + perimeter as i64) as f64;
        let result = (numerator / 2.0).ceil() as i64 + 1;
        result
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

fn run_part(input: &str, color_is_instruction: bool) -> i64 {
    let plan = parse_input(input, color_is_instruction);
    let mut map = DigMap { map: Vec::new() };
    
    plan.run_instructions(&mut map);

    map.get_fill_size()
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

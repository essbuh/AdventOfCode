use std::collections::{HashMap, HashSet};

use once_cell::sync::Lazy;

type Direction = (i32, i32);
type Position = (i32, i32);

const DIRECTION_RIGHT : Direction = (1, 0);
const DIRECTION_LEFT : Direction = (-1, 0);
const DIRECTION_UP : Direction = (0, -1);
const DIRECTION_DOWN : Direction = (0, 1);
const CHAR_MIRROR_R : char = '/';
const CHAR_MIRROR_L : char = '\\';
const CHAR_MIRROR_V : char = '|';
const CHAR_MIRROR_H : char = '-';
const CHAR_EMPTY : char = '.';
const CHAR_ENERGIZED : char = '#';

fn add_direction(pos: &Position, dir: &Direction) -> Position {
    (pos.0 + dir.0, pos.1 + dir.1)
}

#[allow(dead_code)]
fn is_mirror(char: &char) -> bool {
    char == &CHAR_MIRROR_H || char == &CHAR_MIRROR_V
        || char == &CHAR_MIRROR_L || char == &CHAR_MIRROR_R
}

#[allow(dead_code)]
static DIRECTION_TO_CHAR : Lazy<HashMap<Direction, char>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert(DIRECTION_RIGHT, '>');
    m.insert(DIRECTION_LEFT, '<');
    m.insert(DIRECTION_UP, '^');
    m.insert(DIRECTION_DOWN, 'v');
    m
});
static MIRROR_SPLITS: Lazy<HashMap<char, HashMap<Direction, Vec<Direction>>>> = Lazy::new(|| {
    let mut m = HashMap::new();

    {
        let directions = m.entry(CHAR_MIRROR_R).or_insert(HashMap::new());
        directions.insert(DIRECTION_RIGHT, vec![DIRECTION_UP]);
        directions.insert(DIRECTION_LEFT, vec![DIRECTION_DOWN]);
        directions.insert(DIRECTION_UP, vec![DIRECTION_RIGHT]);
        directions.insert(DIRECTION_DOWN, vec![DIRECTION_LEFT]);
    }

    {
        let directions = m.entry(CHAR_MIRROR_L).or_insert(HashMap::new());
        directions.insert(DIRECTION_RIGHT, vec![DIRECTION_DOWN]);
        directions.insert(DIRECTION_LEFT, vec![DIRECTION_UP]);
        directions.insert(DIRECTION_UP, vec![DIRECTION_LEFT]);
        directions.insert(DIRECTION_DOWN, vec![DIRECTION_RIGHT]);
    }

    {
        let directions = m.entry(CHAR_MIRROR_V).or_insert(HashMap::new());
        directions.insert(DIRECTION_RIGHT, vec![DIRECTION_UP, DIRECTION_DOWN]);
        directions.insert(DIRECTION_LEFT, vec![DIRECTION_UP, DIRECTION_DOWN]);
        directions.insert(DIRECTION_UP, vec![DIRECTION_UP]);
        directions.insert(DIRECTION_DOWN, vec![DIRECTION_DOWN]);
    }

    {
        let directions = m.entry(CHAR_MIRROR_H).or_insert(HashMap::new());
        directions.insert(DIRECTION_UP, vec![DIRECTION_LEFT, DIRECTION_RIGHT]);
        directions.insert(DIRECTION_DOWN, vec![DIRECTION_LEFT, DIRECTION_RIGHT]);
        directions.insert(DIRECTION_LEFT, vec![DIRECTION_LEFT]);
        directions.insert(DIRECTION_RIGHT, vec![DIRECTION_RIGHT]);
    }

    m
});

struct GridMap {
    chars: Vec<Vec<char>>,
    mirrors: HashMap<Position, char>,
}
impl GridMap {
    #[allow(dead_code)]
    fn print(self: &Self) {
        for line in &self.chars {
            let s : String = line.iter().collect();
            println!("{s}");
        }
    }

    #[allow(dead_code)]
    fn print_route(self: &Self, energized_cells: &HashMap<Position, HashSet<Direction>>) {
        for (row, line) in self.chars.iter().enumerate() {
            let s : String = line.iter().enumerate()
                .map(|(col, char)| {
                    if is_mirror(char) {
                         return *char;
                    }

                    match energized_cells.get(&(col as i32, row as i32)) {
                        Some(cell) => {
                            assert!(!cell.is_empty());
                            if cell.len() == 1 {
                                *DIRECTION_TO_CHAR.get(cell.iter().last().unwrap()).unwrap()
                            } else {
                                char::from_digit(cell.len() as u32, 10).unwrap()
                            }
                        },
                        None => { *char }
                    }
                }).collect();
            println!("{s}");
        }
    }

    #[allow(dead_code)]
    fn print_energized(self: &Self, energized_cells: &HashMap<Position, HashSet<Direction>>) {
        for (row, line) in self.chars.iter().enumerate() {
            let s : String = line.iter().enumerate()
                .map(|(col, _)| {
                    match energized_cells.get(&(col as i32, row as i32)) {
                        Some(_) => { &CHAR_ENERGIZED },
                        None => { &CHAR_EMPTY }
                    }
                }).collect();
            println!("{s}");
        }
    }

    fn get_char(self: &Self, pos: &Position) -> Option<char> {
        if pos.1 >= 0 && (pos.1 as usize) < self.chars.len() {
            if pos.0 >= 0 && (pos.0 as usize) < self.chars[0].len() {
                return Some(self.chars[pos.1 as usize][pos.0 as usize]);
            }    
        }

        None
    }

    fn try_add_energized_cell(self: &Self, energized_cells: &mut HashMap<Position, HashSet<Direction>>, pos: &Position, dir: &Direction) -> Option<char> {
        let char = self.get_char(pos);
        if char.is_none() {
            return None;
        }

        let cell_set = energized_cells.entry(*pos).or_insert(HashSet::new());
        if cell_set.contains(dir) {
            // Already energized in this direction
            return None;
        }

        cell_set.insert(*dir);
        return char;
    }

    fn calc_energized_cells(self: &Self, start_pos: &Position, start_dir: &Direction) -> HashMap<Position, HashSet<Direction>> {
        let mut energized_cells : HashMap<Position, HashSet<Direction>> = HashMap::new();
        let mut working_set = vec![(*start_pos, *start_dir)];

        while !working_set.is_empty() {
            let current_pos = working_set.pop().unwrap();
            
            let char = self.try_add_energized_cell(&mut energized_cells, &current_pos.0, &current_pos.1);
            if char.is_none() {
                continue;
            }

            let char = char.unwrap();

            match MIRROR_SPLITS.get(&char) {
                Some(mirror_response) => {
                    match mirror_response.get(&current_pos.1) {
                        Some(responses) => {
                            for response in responses {
                                let next_pos = add_direction(&current_pos.0, response);
                                working_set.push((next_pos, *response));
                            }
                        }
                        None => { /* Swallowed */}
                    }
                },
                None => {
                    // Keep going in the same direction
                    let next_pos = add_direction(&current_pos.0, &current_pos.1);
                    working_set.push((next_pos, current_pos.1));
                }
            }
        }

        energized_cells
    }
}

fn parse_input(input: &str) -> GridMap {
    let mut map = GridMap { chars: Vec::new(), mirrors: HashMap::new() };

    for (row, line) in input.lines().enumerate() {
        let chars : Vec<char> = line.chars().collect();        
        for (col, char) in chars.iter().enumerate() {
            if MIRROR_SPLITS.contains_key(char) {
                map.mirrors.insert((col as i32, row as i32), *char);
            }
        }
        map.chars.push(chars);
    }

    map
}

fn part_1() {
    let input = include_str!("input.txt");
    let map = parse_input(&input);
    
    //map.print();

    let energized_cells = map.calc_energized_cells(&(0,0), &DIRECTION_RIGHT);
    let num_energized_cells = energized_cells.len();
    
    /*println!("");
    println!(" -------- ");
    println!("");
    
    map.print_energized();

    println!("");
    println!(" -------- ");
    println!("");*/

    println!("Part 1: {num_energized_cells}");
}

fn part_2() {
    let input = include_str!("input.txt");
    let map = parse_input(&input);
    
    //map.print();

    let best_option = vec![
        (0..map.chars[0].len()).into_iter().map(|col| {
            vec![
                map.calc_energized_cells(&(col as i32, 0), &DIRECTION_DOWN),
                map.calc_energized_cells(&(col as i32, map.chars.len() as i32 - 1), &DIRECTION_UP)
            ]
        }).flatten().max_by(|a, b| a.len().cmp(&b.len())).unwrap(),
        (0..map.chars.len()).into_iter().map(|row| {
            vec![
                map.calc_energized_cells(&(0, row as i32), &DIRECTION_RIGHT),
                map.calc_energized_cells(&(map.chars[0].len() as i32 - 1, row as i32), &DIRECTION_LEFT),
            ]        
        }).flatten().max_by(|a, b| a.len().cmp(&b.len())).unwrap()
    ];

    let best_all = best_option.into_iter()
        .max_by(|a, b| a.len().cmp(&b.len()))
        .unwrap();
    
    /*println!("");
    println!(" -------- ");
    println!("");*/
    
    //map.print_energized(&best_all);

    /*println!("");
    println!(" -------- ");
    println!("");*/

    println!("Part 2: {}", best_all.len());
}

fn main() {
    part_1();
    part_2();
}

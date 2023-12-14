use std::collections::HashMap;

type Point = (i32, i32);
type Direction = (i32, i32);

const CHAR_CUBE : char = '#';
const CHAR_ROUND : char = 'O';
const CHAR_EMPTY : char = '.';

fn add_dir_to_point(point: &Point, direction: &Direction) -> Point {
    (point.0 + direction.0, point.1 + direction.1)
}

fn scale_dir(direction: &Direction, scale: i32) -> Direction {
    (direction.0 * scale, direction.1 * scale)
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Platform {
    rows: Vec<Vec<char>>,
}
impl Platform {
    #[allow(dead_code)]
    fn print(self: &Self) {
        for row in &self.rows {
            let s : String = row.iter().collect();
            println!("{s}");
        }
    }
    fn is_valid_index(self: &Self, point: &Point) -> bool {
        point.0 >= 0 && (point.0 as usize) < self.rows[0].len()
            && point.1 >= 0 && (point.1 as usize) < self.rows.len()
    }

    fn get_char(self: &Self, point: &Point) -> char {
        assert!(self.is_valid_index(point));
        self.rows[point.1 as usize][point.0 as usize]
    }

    fn get_slide_position(self: &Self, position: &Point, slide_direction: &Direction) -> Point {
        if slide_direction.0 == 0 && slide_direction.1 == 0 {
            return *position;
        }

        let mut slide_pos = *position;
        let mut rocks_encountered = 0;
        loop {
            let next_pos = add_dir_to_point(&slide_pos, slide_direction);
            if self.is_valid_index(&next_pos) {
                match self.get_char(&next_pos) {
                    CHAR_CUBE => { break; },
                    CHAR_ROUND => { rocks_encountered += 1; },
                    _ => {},
                }

                slide_pos = next_pos;
            } else {
                break;
            }
        }

        // Offset by the number of other rocks we ran into, as they'll slide the same
        let offset = scale_dir(&slide_direction, -rocks_encountered);
        add_dir_to_point(&slide_pos, &offset)
    }

    fn get_rock_support_weight(self: &Self, pos: &Point, tilt_direction: &Direction) -> i32 {
        let char = self.get_char(&pos);
        if char != CHAR_ROUND {
            return 0;
        }

        let slide_pos = self.get_slide_position(&pos, &tilt_direction);
        (self.rows.len() - slide_pos.1 as usize) as i32
    }

    fn get_support_weight(self: &Self, tilt_direction: (i32, i32)) -> i32 {
        (0..self.rows.len()).map(|row| {
            (0..self.rows[row].len()).map(|col| {
                self.get_rock_support_weight(&(col as i32, row as i32), &tilt_direction)
            }).sum::<i32>()
        }).sum()
    }

    fn run_cycle(self: &Self) -> Platform {
        let mut result_platform = self.clone();

        // tilt north
        //println!("Tilting North");
        for row in 1..result_platform.rows.len() {
            for col in 0..result_platform.rows[row].len() {
                let pos = (col as i32, row as i32);
                if result_platform.get_char(&pos) == CHAR_ROUND {
                    let slide_pos = result_platform.get_slide_position(&pos, &(0, -1));
                    if pos != slide_pos {
                        assert_eq!(result_platform.get_char(&slide_pos), CHAR_EMPTY);
                        result_platform.rows[pos.1 as usize][pos.0 as usize] = CHAR_EMPTY;
                        result_platform.rows[slide_pos.1 as usize][slide_pos.0 as usize] = CHAR_ROUND;
                    }
                }
            }
        }
        // tilt west
        //println!("Tilting West");
        for row in 0..result_platform.rows.len() {
            for col in 1..result_platform.rows[row].len() {
                let pos = (col as i32, row as i32);
                if result_platform.get_char(&pos) == CHAR_ROUND {
                    let slide_pos = result_platform.get_slide_position(&pos, &(-1, 0));
                    if pos != slide_pos {
                        assert_eq!(result_platform.get_char(&slide_pos), CHAR_EMPTY);
                        result_platform.rows[pos.1 as usize][pos.0 as usize] = CHAR_EMPTY;
                        result_platform.rows[slide_pos.1 as usize][slide_pos.0 as usize] = CHAR_ROUND;
                    }
                }
            }
        }
        // tilt south
        //println!("Tilting South");
        for row in (0..result_platform.rows.len()-1).rev() {
            for col in 0..result_platform.rows[row].len() {
                let pos = (col as i32, row as i32);
                if result_platform.get_char(&pos) == CHAR_ROUND {
                    let slide_pos = result_platform.get_slide_position(&pos, &(0, 1));
                    if pos != slide_pos {
                        assert_eq!(result_platform.get_char(&slide_pos), CHAR_EMPTY);
                        result_platform.rows[pos.1 as usize][pos.0 as usize] = CHAR_EMPTY;
                        result_platform.rows[slide_pos.1 as usize][slide_pos.0 as usize] = CHAR_ROUND;
                    }
                }
            }
        }
        // tilt east
        //println!("Tilting East");
        for row in 0..result_platform.rows.len() {
            for col in (0..result_platform.rows[row].len()-1).rev() {
                let pos = (col as i32, row as i32);
                if result_platform.get_char(&pos) == CHAR_ROUND {
                    let slide_pos = result_platform.get_slide_position(&pos, &(1, 0));
                    if slide_pos != pos {
                        assert_eq!(result_platform.get_char(&slide_pos), CHAR_EMPTY);
                        result_platform.rows[pos.1 as usize][pos.0 as usize] = CHAR_EMPTY;
                        result_platform.rows[slide_pos.1 as usize][slide_pos.0 as usize] = CHAR_ROUND;
                    }
                }
            }
        }

        result_platform
    }
}

fn parse_input(input: &str) -> Platform {
    let mut platform = Platform { rows: Vec::new() };

    for line in input.lines() {
        platform.rows.push(line.chars().collect());
    }

    platform
}

fn part_1()
{
    let input = include_str!("input.txt");
    let platform = parse_input(&input);
    let weight = platform.get_support_weight((0, -1));

    println!("Part 1: {weight}");
}

fn part_2()
{
    let input = include_str!("input.txt");
    let mut platform = parse_input(&input);

    let mut seen_platforms : HashMap<Platform, i32> = HashMap::new();
    let mut iter_to_platform : HashMap<i32, Platform> = HashMap::new();
    
    seen_platforms.insert(platform.clone(), 0);
    iter_to_platform.insert(0, platform.clone());
    
    let num_cycles = /*3*/1000000000;
    let mut loop_start = None;

    for i in 0..num_cycles {
        //println!("--------");
        let next_platform = platform.run_cycle();
        match seen_platforms.get(&next_platform) {
            Some(iter) => {
                loop_start = Some(*iter);
                //println!("Hit a loop! current index = {i}, loop_start = {}", *iter);
                break;
            }
            None => {
                seen_platforms.insert(next_platform.clone(), i+1);
                iter_to_platform.insert(i+1, next_platform.clone());
            }
        }

        platform = next_platform.clone();
        //platform.print();
    }

    let weight : i32;
    match &loop_start {
        Some(start) => {
            let loop_end = seen_platforms.len() as i32;
            let loop_length = loop_end - start;
            //println!("Hit a loop on iteration {}, length {}", start, loop_length);
            let final_index = (num_cycles - start) % loop_length + start;
            let final_platform = iter_to_platform.get(&final_index).unwrap();
            weight = final_platform.get_support_weight((0, 0));

            //println!("Final platform: {final_index}");
            //final_platform.print();
        },
        None => {
            //println!("No loop detected.");
            weight = platform.get_support_weight((0, 0));
        }
    }
    
    println!("Part 2: {weight}");
}

fn main() {
    part_1();
    part_2();
}

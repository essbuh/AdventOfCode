use std::collections::{HashSet, HashMap};

const CHAR_ROCK : char = '#';
const CHAR_EMPTY : char = '.';
const CHAR_STEP : char = 'O';
const CHAR_START : char = 'S';

struct Direction{ x: i64, y: i64 }
const DIR_UP : Direction = Direction{ x: 0, y: -1 };
const DIR_DOWN : Direction = Direction{ x: 0, y: 1 };
const DIR_LEFT : Direction = Direction{ x: -1, y: 0 };
const DIR_RIGHT : Direction = Direction{ x: 1, y: 0 };

#[derive(PartialEq)]
enum StepResult {
    Overflow,
    Rock,
    Success
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Point{ x: i64, y: i64 }
impl Point {
    fn zero() -> Point {
        Point { x: 0, y: 0 }
    }
    fn add(self: &Self, dir: &Direction) -> Point {
        Point { x: self.x + dir.x, y: self.y + dir.y }
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
struct MemoKey {
    s: Vec<Point>
}
impl MemoKey {
    fn from_input(steps: &HashSet<Point>) -> MemoKey {
        let mut steps_vec : Vec<Point> = steps.clone().into_iter().collect();
        steps_vec.sort_by(|a, b| {
            if a.x == b.x {
                a.y.cmp(&b.y)
            } else {
                a.x.cmp(&b.x)
            }
        });

        MemoKey { s: steps_vec }
    }
    fn from_point(point: &Point) -> MemoKey {
        MemoKey { s: vec![ *point ] }
    }
}

#[derive(Debug)]
struct InfiniteGarden {
    gardens: HashMap<Point, Garden>,
    garden_dim: Point,
    default_garden: Garden,
    // Given a garden and input steps, returns a result step & all overflow steps
    step_memo: HashMap<MemoKey, (HashSet<Point>, HashSet<Point>)>,
}
impl InfiniteGarden {
    fn from_input(input: &str) -> InfiniteGarden {
        let mut garden = Garden::from_input(input);
        let mut inf_garden = InfiniteGarden { 
            gardens: HashMap::new(), 
            garden_dim: garden.dim, 
            default_garden: garden.clone(),
            step_memo: HashMap::new()
        };

        garden.entry_point = Some(MemoKey::from_point(&inf_garden.default_garden.start_point));
        garden.reached_start = true;
        inf_garden.gardens.insert(Point::zero(), garden);

        inf_garden
    }
    
    fn get_cell(self: &Self, point: &Point) -> Point {
        // If negative, add an extra garden dim as int division rounds to zero
        let mut point_x = point.x;
        if point_x < 0 { point_x -= self.garden_dim.x; }
        let mut point_y = point.y;
        if point_y < 0 { point_y -= self.garden_dim.y; }

        let cell_x = point_x / self.garden_dim.x;
        let cell_y = point_y / self.garden_dim.y;
        Point { x: cell_x, y: cell_y }
    }

    fn get_garden_at_cell(self: &mut Self, cell: &Point) -> &mut Garden {
        let garden = self.gardens.entry(*cell).or_insert(self.default_garden.clone());
        garden.cell = *cell;        
        garden
    }

    fn run_steps(self: &mut Self, step_num: i64, step_map: &mut HashMap<Point, HashSet<Point>>, equilibrium: &mut Equilibrium) {
        let mut overflow_steps = HashMap::new();
        let mut cells_to_remove = HashSet::new();

        for (cell, steps) in step_map.iter_mut() {
            assert!(!self.get_garden_at_cell(cell).equilibrium);
            let result = self.run_garden_steps(cell, &steps, equilibrium);
            match result {
                Some((garden_steps, overflow)) => {
                    *steps = garden_steps;

                    for mut step in overflow {                        
                        let mut next_cell = *cell;
                        if step.x < 0 { next_cell.x -= 1; step.x += self.garden_dim.x; }
                        else if step.x >= self.garden_dim.x { next_cell.x += 1; step.x -= self.garden_dim.x; }
                        
                        if step.y < 0 { next_cell.y -= 1; step.y += self.garden_dim.y; }
                        else if step.y >= self.garden_dim.y { next_cell.y += 1; step.y -= self.garden_dim.y; }

                        overflow_steps.entry(next_cell).or_insert(HashSet::new()).insert(step);
                    }
                },
                None => {
                    let garden = self.get_garden_at_cell(cell);
                    garden.eq_start = step_num;

                    equilibrium.gardens.insert(*cell, (step_num, garden.eq_counts.0, garden.eq_counts.1));
                    cells_to_remove.insert(*cell);
                }
            }
        }

        for (cell, points) in overflow_steps {
            if equilibrium.gardens.contains_key(&cell) || cells_to_remove.contains(&cell) {
                continue;
            }
            
            let garden = self.get_garden_at_cell(&cell);
            for point in points {
                if garden.try_step(&point) == StepResult::Success {
                    step_map.entry(cell).or_default().insert(point);
                }
            }

            if garden.entry_point.is_none() {
                //println!("Entered cell {},{}", cell.x, cell.y);
                garden.entry_point = Some(MemoKey::from_input(step_map.get(&cell).unwrap()));
            }
        }
        
        for cell in cells_to_remove {
            step_map.remove(&cell);
        }
    }
    
    fn run_garden_steps(self: &mut Self, cell: &Point, steps: &HashSet<Point>, equilibrium: &mut Equilibrium) -> Option<(HashSet<Point>, HashSet<Point>)> {
        let key = MemoKey::from_input(steps);
        if self.step_memo.contains_key(&key) {
            let next_len = self.step_memo.get(&key).unwrap().0.len();

            let garden = self.get_garden_at_cell(cell);
            if garden.step_memo.contains(&key) {
                assert!(!garden.equilibrium);
                
                //println!("Cell at {},{} reached equilibrium, {next_len} <-> {}!", cell.x, cell.y, steps.len());
                garden.equilibrium = true;            
                garden.eq_counts = (next_len, steps.len());

                if equilibrium.steps.0.len() == 0 {
                    equilibrium.steps.0.extend(self.step_memo.get(&key).unwrap().0.clone());
                    equilibrium.steps.1.extend(steps);
                }

                return None;
            }

            if garden.entry_point.is_none() {
                //println!("Entered cell {},{}", cell.x, cell.y);
                garden.entry_point = Some(key.clone());
            }

            garden.step_memo.insert(key.clone());
            return Some(self.step_memo.get(&key).unwrap().clone());
        }

        let garden = self.get_garden_at_cell(cell);
        let result = garden.run_steps(steps);
        //println!("Cell at {},{} has {} steps", cell.x, cell.y, result.0.len());
        self.step_memo.insert(key, result.clone());
        Some(result)
    }


    fn print(self: &mut Self, step_count: i64, steps: &HashMap<Point, HashSet<Point>>, equilibrium: &Equilibrium) {
        let min_x = self.gardens.iter().map(|(k, _)| k.x).min().unwrap();
        let min_y = self.gardens.iter().map(|(k, _)| k.y).min().unwrap();
        let max_x = self.gardens.iter().map(|(k, _)| k.x).max().unwrap();
        let max_y = self.gardens.iter().map(|(k, _)| k.y).max().unwrap();

        for cell_y in min_y..=max_y {
            for y in 0..self.garden_dim.y {            
                let mut s = String::new();
                
                for cell_x in min_x..=max_x {
                    let cell = Point { x: cell_x, y: cell_y };
                    let garden = self.get_garden_at_cell(&cell);
                    let garden_steps = match garden.equilibrium {
                        false => steps.get(&cell),
                        true => {
                            let step_eq = (step_count - garden.eq_start) % 2;
                            let step_count = if step_eq == 0 { garden.eq_counts.0 } else { garden.eq_counts.1 };
                            if equilibrium.steps.0.len() == step_count {
                                Some(&equilibrium.steps.0)
                            } else {
                                Some(&equilibrium.steps.1)
                            }
                        }
                    };
                    match garden_steps {
                        Some(cell_steps) => {
                            s += &garden.get_row_as_str(y, &cell_steps);
                        },
                        None => { s += &garden.get_row_as_str(y, &HashSet::new()); }
                    }                    
                    s += " ";
                }

                println!("{s}");
            }

            println!("");
        }
    }
}

struct Equilibrium {
    steps: (HashSet<Point>, HashSet<Point>),
    gardens: HashMap<Point, (i64, usize, usize)>,
}

#[derive(Debug, Clone)]
struct Garden {
    rocks: HashSet<Point>,
    dim: Point,
    cell: Point,
    start_point: Point,
    reached_start: bool,
    entry_point: Option<MemoKey>,
    equilibrium: bool,
    eq_start: i64,
    eq_counts: (usize, usize),
    step_memo: HashSet<MemoKey>,
}
impl Garden {
    fn from_input(input: &str) -> Garden {
        let mut dim: Point = Point { x: 0, y: input.lines().count() as i64 };
        let mut rocks = HashSet::new();
        let mut starting_point = Point::zero();

        for (row, line) in input.lines().enumerate() {
            dim.x = line.len() as i64;

            for (col, char) in line.chars().enumerate() {
                let point = Point { x: col as i64, y: row as i64 };

                match char {
                    CHAR_START => { starting_point = point; },
                    CHAR_ROCK => { rocks.insert(point); },
                    _ => {}
                }
            }
        }

        Garden{  
            rocks, 
            dim, 
            cell: Point::zero(), 
            start_point: starting_point,
            entry_point: None,
            reached_start: false,
            equilibrium: false, 
            eq_start: 0,
            eq_counts: (0, 0),
            step_memo: HashSet::new(),
         }
    }

    fn get_row_as_str(self: &Self, row: i64, steps: &HashSet<Point>) -> String {
        let mut s: String = String::new();
        s.reserve(self.dim.x as usize);

        for col in 0..self.dim.x {
            let point = Point { x: col, y: row };

            if steps.contains(&point) {
                s += &CHAR_STEP.to_string();
            } else if self.rocks.contains(&point) {
                s += &CHAR_ROCK.to_string();
            } else {
                s += &CHAR_EMPTY.to_string();
            }
        }

        s
    }

    fn print(self: &Self, steps: &HashSet<Point>) {
        for row in 0..self.dim.y {
            let s = self.get_row_as_str(row, steps);
            println!("{s}");
        }
    }

    fn has_rock(self: &Self, point: &Point) -> bool {
        self.rocks.contains(&point)
    }

    fn try_step(self: &Self, point: &Point) -> StepResult {
        if self.has_rock(&point) {
            return StepResult::Rock;
        }

        if point.x < 0 || point.x >= self.dim.x || point.y < 0 || point.y >= self.dim.y {
            return StepResult::Overflow;
        }

        StepResult::Success
    }

    fn to_garden_space(self: &Self, point: &Point) -> Point {
        let mut point_gs = *point;
        while point_gs.x < 0 { point_gs.x += self.dim.x; }
        while point_gs.y < 0 { point_gs.y += self.dim.y; }

        Point { x: point_gs.x % self.dim.x, y: point_gs.y % self.dim.y }
    }

    fn run_steps(self: &mut Self, current_steps: &HashSet<Point>) -> (HashSet<Point>, HashSet<Point>) {
        let mut new_steps = HashSet::new();
        let mut overflow_steps = HashSet::new();
        let directions = vec![ DIR_UP, DIR_LEFT, DIR_DOWN, DIR_RIGHT ];

        if self.entry_point.is_none() {
            //println!("Entered cell {},{}", self.cell.x, self.cell.y);
            self.entry_point = Some(MemoKey::from_input(current_steps));
        }

        for step in current_steps {
            for direction in &directions {
                let next_point = step.add(&direction);
                match self.try_step(&next_point) {
                    StepResult::Overflow => { overflow_steps.insert(next_point); },
                    StepResult::Success => { 
                        new_steps.insert(next_point); 
                        if !self.reached_start && next_point == self.start_point {
                            //println!("Garden at cell {},{} reached start!", self.cell.x, self.cell.y);
                            self.reached_start = true;
                        }
                    },
                    _ => {}
                }
            }
        }

        self.step_memo.insert(MemoKey::from_input(&current_steps));

        (new_steps, overflow_steps)
    }
}

pub fn get_result_part1(input: &str, step_count: i64, debug: bool) -> usize {
    let mut garden = Garden::from_input(input);
    garden.reached_start = true;
    
    let mut steps = HashSet::new();
    steps.insert(garden.start_point);
    
    if debug {
        println!("Starting Layout:");
        garden.print(&steps);
        println!("\n------------\n");
    }

    for i in 0..step_count {
        //println!("Processing step {} of {step_count}...", i+1);
        let (new_steps, _overflow_steps) = garden.run_steps(&steps);
        steps = new_steps;

        if debug {
            println!("After {} step(s):\n", i+1);
            garden.print(&steps);
            println!("\n------------\n");
        }
    }

    steps.len()
}

fn get_step_count(step_count: i64, step_map: &HashMap<Point, HashSet<Point>>, equilibrium: &Equilibrium) -> usize {
    let partial_steps : usize = step_map.iter().map(|(_, v)| { v.len() }).sum();
    let equilibrium_steps : usize = equilibrium.gardens.iter().map(|(_, v)| {
        let m = ((step_count-1) - v.0) % 2;
        if m == 0 { v.1 } else { v.2 }
    }).sum();

    partial_steps + equilibrium_steps
}

/**
 * Lagrange's Interpolation formula for ax^2 + bx + c with x=[0,1,2] and y=[y0,y1,y2] we have
 *   f(x) = (x^2-3x+2) * y0/2 - (x^2-2x)*y1 + (x^2-x) * y2/2
 * so the coefficients are:
 * a = y0/2 - y1 + y2/2
 * b = -3*y0/2 + 2*y1 - y2/2
 * c = y0
 */
fn simplified_lagrange(values: &(i64, i64, i64)) -> (i64, i64, i64) {
    (
      (values.0 as f64 / 2.0 - values.1 as f64 + values.2 as f64 / 2.0) as i64,
      (-3.0 * (values.0 as f64 / 2.0) + 2.0 * values.1 as f64 - values.2 as f64 / 2.0) as i64,
      values.0,
    )
  }

pub fn solve_part2(input: &str, step_count: i64, debug: bool) -> i64 {
    let input_dim = input.lines().into_iter().count() as i64;
    
    let values = (
        get_result_part2(input, input_dim/2, debug) as i64, 
        get_result_part2(input, input_dim/2 + input_dim, debug) as i64, 
        get_result_part2(input, input_dim/2 + (input_dim * 2), debug) as i64
    );

    let poly = simplified_lagrange(&values);
    let target = (step_count - input_dim/2) / input_dim;
    poly.0 * target * target + poly.1 * target + poly.2
  }

pub fn get_result_part2(input: &str, step_count: i64, debug: bool) -> usize {
    let mut garden = InfiniteGarden::from_input(input);
    
    let mut starting_steps = HashSet::new();
    starting_steps.insert(garden.default_garden.start_point);

    let mut equilibrium = Equilibrium { steps: (HashSet::new(), HashSet::new()), gardens: HashMap::new() };
    
    let mut step_map = HashMap::new();
    step_map.insert(Point::zero(), starting_steps);
    
    if debug {
        println!("Starting Layout:");
        garden.print(0, &step_map, &equilibrium);
        println!("------------\n");
    }

    for i in 0..step_count {
        //println!("Processing step {} of {step_count}...", i+1);
        garden.run_steps(i, &mut step_map, &mut equilibrium);

        if debug {
            println!("After {} step(s), {} at equilibrium:", i+1, equilibrium.gardens.len());
            garden.print(i, &step_map, &equilibrium);
            println!("------------\n");
        }
    }

    get_step_count(step_count, &step_map, &equilibrium)
}    

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_sample() {
        let result = get_result_part1(include_str!("sample.txt"), 6, false);
        assert_eq!(result, 16);
    }

    #[test]
    fn part_1_input() {
        let result = get_result_part1(include_str!("input.txt"), 64, false);
        assert_eq!(result, 3841);
    }

    #[test]
    fn part_2_sample_1() {
        let result = solve_part2(include_str!("sample.txt"), 6, true);
        assert_eq!(result, 16);
    }
        
    #[test]
    fn part_2_sample_2() {
        let result = get_result_part2(include_str!("sample.txt"), 10, true);
        assert_eq!(result, 50);
    }
        
    #[test]
    fn part_2_sample_3() {        
        let result = get_result_part2(include_str!("sample.txt"), 50, true);
        assert_eq!(result, 1594);
    }
        
    #[test]
    fn part_2_sample_4() {        
        let result = get_result_part2(include_str!("sample.txt"), 100, false);
        assert_eq!(result, 6536);
    }
        
    #[test]
    fn part_2_sample_6() {        
        let result = get_result_part2(include_str!("sample.txt"), 1000, false);
        assert_eq!(result, 668697);
    }
        
    #[test]
    fn part_2_sample_7() {        
        let result = get_result_part2(include_str!("sample.txt"), 5000, false);
        assert_eq!(result, 16733044);
    }

    #[test]
    fn part_2_input() {
        let result = solve_part2(include_str!("input.txt"), 26501365, false);
        assert_eq!(result, 636391426712747);
    }
}
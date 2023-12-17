use std::{collections::{HashMap, HashSet, BinaryHeap}, cmp::Ordering};

type Direction = (i32, i32);
type Point = (i32, i32);

const DIR_RIGHT : Direction = (1, 0);
const DIR_LEFT : Direction = (-1, 0);
const DIR_UP : Direction = (0, -1);
const DIR_DOWN : Direction = (0, 1);

fn get_distance(a: &Point, b: &Point) -> u32 {
    a.0.abs_diff(b.0) + a.1.abs_diff(b.1)
}

fn add_dir(point: &Point, dir: &Direction) -> Point {
    (point.0 + dir.0, point.1 + dir.1)
}

#[allow(dead_code)]
fn get_direction_char(from: &Point, to: &Point) -> char {
    if from.1 != to.1 {
        if from.1 < to.1 { 'v' } else { '^' }
    } else {
        if from.0 < to.0 { '>' } else { '<' }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct SearchCandidate {
    point: Point,
    dir: Direction,
    count_same_dir: i32,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct OpenSetEntry {
    candidate: SearchCandidate,
    f_score: u32
}
impl Ord for OpenSetEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        other.f_score.cmp(&self.f_score)
    }
}
impl PartialOrd for OpenSetEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct LavaMap {
    cells: Vec<Vec<u32>>,
}
impl LavaMap {
    fn is_valid(self: &Self, point: &Point) -> bool {
        point.0 >= 0 && point.1 >= 0 &&
            (point.0 as usize) < self.cells[0].len() &&
            (point.1 as usize) < self.cells.len()
    }

    fn get_value(self: &Self, point: &Point) -> u32 {
        self.cells[point.1 as usize][point.0 as usize]
    }

    fn try_push(self: &Self, point: &Point, dir: &Direction, dir_count: i32, out_candidates: &mut Vec<SearchCandidate>) {
        let next_point = add_dir(point, dir);
        if self.is_valid(&next_point) {
            out_candidates.push(SearchCandidate{ point: next_point, dir: *dir, count_same_dir: dir_count });
        }
    }
    
    fn get_search_candidates(self: &Self, point: &SearchCandidate, min_same_dir: i32, max_same_dir: i32, out_candidates: &mut Vec<SearchCandidate>) {
        out_candidates.clear();
        out_candidates.reserve(4);
    
        let mut valid_dir = true;
        if point.count_same_dir == 0 || point.count_same_dir >= min_same_dir {
            match &point.dir {
                &DIR_RIGHT => { 
                    self.try_push(&point.point, &DIR_UP, 1, out_candidates);
                    self.try_push(&point.point, &DIR_DOWN, 1, out_candidates);
                 },
                &DIR_LEFT => { 
                    self.try_push(&point.point, &DIR_UP, 1, out_candidates);
                    self.try_push(&point.point, &DIR_DOWN, 1, out_candidates);
                 },
                &DIR_UP => { 
                    self.try_push(&point.point, &DIR_LEFT, 1, out_candidates);
                    self.try_push(&point.point, &DIR_RIGHT, 1, out_candidates);
                 },
                &DIR_DOWN => { 
                    self.try_push(&point.point, &DIR_LEFT, 1, out_candidates);
                    self.try_push(&point.point, &DIR_RIGHT, 1, out_candidates);
                 },
                _ => {
                    self.try_push(&point.point, &DIR_UP, 1, out_candidates);
                    self.try_push(&point.point, &DIR_DOWN, 1, out_candidates);
                    self.try_push(&point.point, &DIR_LEFT, 1, out_candidates);
                    self.try_push(&point.point, &DIR_RIGHT, 1, out_candidates);
                    valid_dir = false;
                }
            } 
        }

        if valid_dir && point.count_same_dir + 1 <= max_same_dir {
            self.try_push(&point.point, &point.dir, point.count_same_dir + 1, out_candidates);
        }
    }
    
    fn traverse_graph(self: &Self, start: &Point, end: &Point, min_same_dir: i32, max_same_dir: i32) -> u32 {
        let mut open_set : BinaryHeap<OpenSetEntry> = BinaryHeap::new();
        let mut open_set_hash : HashSet<OpenSetEntry> = HashSet::new();
        let mut found_goal : Option<SearchCandidate> = None;
    
        let first_candidate = SearchCandidate { point: *start, dir: (0,0), count_same_dir: 0 };
        let mut g_scores : HashMap<SearchCandidate, u32> = HashMap::new();
        g_scores.insert(first_candidate.clone(), 0);
    
        let mut f_scores : HashMap<SearchCandidate, u32> = HashMap::new();
        f_scores.insert(first_candidate.clone(), get_distance(start, end));
    
        let mut came_from : HashMap<SearchCandidate, SearchCandidate> = HashMap::new();
    
        let first_open_set = OpenSetEntry { candidate: first_candidate, f_score: 0 };
        open_set.push(first_open_set);
        open_set_hash.insert(first_open_set);
    
        let mut neighbors : Vec<SearchCandidate> = Vec::new();
    
        loop {
            if open_set.is_empty() {
                break;
            }
    
            let search_point = open_set.pop().unwrap();
            open_set_hash.remove(&search_point);

            if &search_point.candidate.point == end {
                if search_point.candidate.count_same_dir < min_same_dir {
                    continue;
                }
                
                found_goal = Some(search_point.candidate);
                break;
            }

            self.get_search_candidates(&search_point.candidate, min_same_dir, max_same_dir, &mut neighbors);
            
            for neighbor in &mut neighbors {
                let tentative_gscore = g_scores[&search_point.candidate] + self.get_value(&neighbor.point);
                if tentative_gscore < *g_scores.entry(*neighbor).or_insert(u32::MAX) {
                    let f_score = tentative_gscore + get_distance(&neighbor.point, end);

                    came_from.insert(*neighbor, search_point.candidate);
                    g_scores.insert(*neighbor, tentative_gscore);

                    f_scores.insert(*neighbor, tentative_gscore + f_score);

                    let next_open_set = OpenSetEntry { candidate: *neighbor, f_score };
                    if !open_set_hash.contains(&next_open_set) {
                        open_set_hash.insert(next_open_set);
                        open_set.push(next_open_set);
                    }
                }
            }
        }
    
        assert!(found_goal.is_some());
    
        let last_candidate = found_goal.unwrap();
        let mut path : Vec<&SearchCandidate> = vec![ &last_candidate ];
        loop {
            match came_from.get(path.last().unwrap()) {
                Some(p) => { path.push(p); },
                None => { break; }
            }
        }

       // println!("Path: {:?}", path);
    
        //self.print_path(&path);
    
        let last_point = path.pop().unwrap(); // Remove the start point
        assert_eq!(&last_point.point, start);

        let total_cost : u32 = path.iter().map(|x| self.get_value(&x.point)).sum();
        total_cost
    }

    #[allow(dead_code)]
    fn print_path(self: &Self, path: &Vec<&SearchCandidate>) {
        self.cells.iter().enumerate().for_each(|(j, row)| {
            let s : String = row.iter().enumerate().map(|(i, cell)| {
                let point = (i as i32, j as i32);
                match path.iter().position(|p| p.point.0 == point.0 && p.point.1 == point.1) {
                    Some(p) => {
                        if p < (path.len() - 1) {
                            let prev_point = path[p + 1];
                            get_direction_char(&prev_point.point, &point)
                        } else {
                            char::from_u32(*cell + 0x30).unwrap()
                        }
                    },
                    None => { char::from_u32(*cell + 0x30).unwrap() }
                }
            }).collect();

            println!("{s}");
        })
    }
}

fn parse_input(input: &str) -> LavaMap {
    LavaMap {
        cells: input.lines().map(|line| {
            line.chars().into_iter().map(|c| (c as u32) - 0x30).collect()
        }).collect()
    }
}

fn part_1() {
    let input = include_str!("input.txt");
    let grid = parse_input(&input);
    let shortest_cost = grid.traverse_graph(&(0,0), &((grid.cells[0].len() as i32 - 1), grid.cells.len() as i32 - 1), 1, 3);

    println!("Part 1: {shortest_cost}");
}

fn part_2() {
    let input = include_str!("input.txt");
    let grid = parse_input(&input);
    let shortest_cost = grid.traverse_graph(&(0,0), &((grid.cells[0].len() as i32 - 1), grid.cells.len() as i32 - 1), 4, 10);

    println!("Part 2: {shortest_cost}");
}
fn main() {
    part_1();
    part_2();
}

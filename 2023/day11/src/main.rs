use std::{collections::{HashMap, BinaryHeap, HashSet}, cmp::Ordering};

const EXPANSION_CHAR : char = 'X';

fn is_empty(chars: &Vec<char>) -> bool {
    chars.iter().all(|c| c == &'.' || c == &'X')
}

#[allow(dead_code)]
fn print_map(map: &GalaxyMap) {
    for row in &map.rows {
        let string : String = row.iter().collect();
        println!("{:?}", string);
    }
}

fn is_search_move_allowed(delta_x: i32, delta_y: i32, allow_diagonal: bool) -> bool {
    if allow_diagonal {
        true // assume only 1 space between
    } else {
        delta_x == 0 || delta_y == 0 // no diagonals!
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Point {
    x: usize,
    y: usize,
}
impl Point {
    fn new(x: usize, y: usize) -> Point {
        Point { x, y }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct SearchCandidate {
    point: Point,
    dist: usize,
}
impl Ord for SearchCandidate {
    fn cmp(&self, other: &Self) -> Ordering {
        other.dist.cmp(&self.dist)
            .then_with(|| other.point.cmp(&self.point))
    }
}
impl PartialOrd for SearchCandidate {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug)]
struct GalaxyMap {
    rows: Vec<Vec<char>>,
    galaxies: Vec<Point>,
    expansion_cost: usize,
}
impl GalaxyMap {
    fn expand(self: &mut Self) {
        // expand rows
        for i in (0..self.rows.len()).rev() {
            if is_empty(&self.rows[i]) {
                self.rows[i].iter_mut().for_each(|x| *x = EXPANSION_CHAR);
            }
        }

        // expand cols
        for i in (0..self.rows[0].len()).rev() {
            let cols : Vec<char> = self.rows.iter().map(|x| x[i]).collect();
            if is_empty(&cols) {
                self.rows.iter_mut().for_each(|x| x[i] = EXPANSION_CHAR);
            }
        }
    }

    fn parse(input: &str, expansion_cost: usize) -> GalaxyMap {
        let mut map = GalaxyMap { rows: Vec::new(), galaxies: Vec::new(), expansion_cost };

        for line in input.lines() {
            map.rows.push(line.chars().collect());
        }

        map.expand();

        // find galaxies in expanded map
        for (row, line) in map.rows.iter().enumerate() {
            for (col, char) in line.iter().enumerate() {
                if char == &'#' {
                    map.galaxies.push(Point::new(col, row));
                }
            }
        }

        map
    }

    fn is_valid_point(self: &Self, point: &Point) -> bool {
        point.y < self.rows.len() && point.x < self.rows[point.y].len()
    }

    fn get_distance(self: &Self, point_a: &Point, point_b: &Point) -> usize {
        point_a.x.abs_diff(point_b.x) + point_b.y.abs_diff(point_b.y)
    }

    fn get_cell_cost(self: &Self, x: usize, y: usize) -> usize {
        let char = self.rows[y][x];
        if char == EXPANSION_CHAR { self.expansion_cost } else { 1 } 
    }
    
    fn get_search_candidates(self: &Self, point: &Point, dest: &Point, out_candidates: &mut Vec<SearchCandidate>) {
        let row_range = if point.y > 0 { -1..2 } else { 0..2 };
        for y in row_range {
            let col_range = if point.x > 0 { -1..2 } else { 0..2 };
            for x in col_range {
                if !is_search_move_allowed(x, y, false) {
                    continue;
                }

                let candidate_x = (point.x as i32) + x;
                let candidate_y = (point.y as i32) + y;
                let candidate_point = Point { x: candidate_x as usize, y: candidate_y as usize };
                if &candidate_point != point && self.is_valid_point(&candidate_point) {
                    let candidate = SearchCandidate { point: candidate_point, dist: self.get_distance(&candidate_point, dest) };
                    out_candidates.push(candidate);
                }
            }
        }
    }

    fn get_shortest_path(self: &Self, galaxy_a: usize, galaxy_b: usize) -> usize {
        let point_a = &self.galaxies[galaxy_a];
        let point_b = &self.galaxies[galaxy_b];

        let mut open_set = BinaryHeap::new();
        let mut open_set_hash = HashSet::new();
        let mut found_goal = false;

        let mut g_scores : HashMap<Point, usize> = HashMap::new();
        g_scores.insert(*point_a, 0);

        let mut f_scores : HashMap<Point, usize> = HashMap::new();
        f_scores.insert(*point_a, self.get_distance(point_a, point_b));

        let mut came_from : HashMap<Point, Point> = HashMap::new();

        open_set.push(SearchCandidate{ point: *point_a, dist: 0 });
        open_set_hash.insert(*point_a);

        loop {
            if open_set.is_empty() {
                break;
            }

            let search_point = open_set.pop().unwrap();
            if &search_point.point == point_b {
                found_goal = true;
                break;
            }
            open_set_hash.remove(&search_point.point);

            let mut neighbors : Vec<SearchCandidate> = Vec::new();
            self.get_search_candidates(&search_point.point, &point_b, &mut neighbors);

            for neighbor in neighbors {
                let tentative_gscore = g_scores[&search_point.point] + self.get_cell_cost(neighbor.point.x, neighbor.point.y);
                if tentative_gscore < *g_scores.entry(neighbor.point).or_insert(usize::MAX) {
                    // This path to neighbor is better than any previous
                    came_from.insert(neighbor.point, search_point.point);
                    g_scores.insert(neighbor.point, tentative_gscore);
                    f_scores.insert(neighbor.point, tentative_gscore + neighbor.dist);

                    if !open_set_hash.contains(&neighbor.point) {
                        open_set_hash.insert(neighbor.point.clone());
                        open_set.push(neighbor);
                    }
                }
            }
        }

        assert_eq!(found_goal, true);

        let mut path : Vec<&Point> = vec![ point_b ];
        loop {
            match came_from.get(path.last().unwrap()) {
                Some(p) => { path.push(p); },
                None => { break; }
            }
        }

        let last_point = path.pop().unwrap(); // Remove the start point
        assert_eq!(last_point, point_a);

        let total_cost = path.iter().map(|x| self.get_cell_cost(x.x, x.y)).sum();
        //println!("Shortest path between galaxies {galaxy_a} and {galaxy_b} is {total_cost}");
        total_cost
    }

    fn get_shortest_paths(self: &Self) -> usize {
        let mut total_sum = 0;
        for i in 0..self.galaxies.len() {
            for j in (i+1)..self.galaxies.len() {
                total_sum = total_sum + self.get_shortest_path(i, j);
            }
        }
        total_sum
    }
}

fn part_1() {
    let input = include_str!("input.txt");
    let map = GalaxyMap::parse(input, 2);
    //print_map(&map);

    let shortest_paths = map.get_shortest_paths();
    println!("Part 1: {shortest_paths}");
}

fn part_2() {    
    let input = include_str!("input.txt");
    let map = GalaxyMap::parse(input, 1000000);
    //print_map(&map);

    let shortest_paths = map.get_shortest_paths();
    println!("Part 2: {shortest_paths}");
}

fn main() {
    part_1();
    part_2();
}

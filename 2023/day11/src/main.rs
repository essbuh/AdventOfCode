use std::collections::HashSet;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
}
impl Point {
    fn new(x: usize, y: usize) -> Point {
        Point { x, y }
    }
}

#[derive(Debug, Clone, Copy)]
struct SearchCandidate {
    point: Point,
    dist: usize,
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
        let mut distance: usize = 0;

        let range_y = if point_a.y < point_b.y { point_a.y..(point_b.y+1) } else { point_b.y..(point_a.y+1) };
        for y in range_y {
            let range_x = if point_a.x < point_b.x { point_a.x..(point_b.x+1) } else { point_b.x..(point_a.x+1) };
            for x in range_x {
                distance = distance + self.get_cell_cost(x, y);
            }
        }

        distance
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
                    let candidate = SearchCandidate { point: candidate_point, dist: self.get_distance(&candidate_point, &dest) };
                    out_candidates.push(candidate);
                }
            }
        }
    }

    fn get_shortest_path(self: &Self, galaxy_a: usize, galaxy_b: usize) -> usize {
        let point_a = &self.galaxies[galaxy_a];
        let point_b = &self.galaxies[galaxy_b];

        let mut visited_nodes: HashSet<Point> = HashSet::new();
        let mut search_stack : Vec<Point> = Vec::new();
        let mut candidates : Vec<SearchCandidate> = Vec::new();
        let mut found_goal = false;

        candidates.push(SearchCandidate { point: *point_a, dist: self.get_distance(&point_a, &point_b) });

        loop {
            if candidates.is_empty() {
                break;
            }

            let search_point = candidates.pop().unwrap().point;
            if visited_nodes.contains(&search_point) {
                continue;
            }

            visited_nodes.insert(search_point);

            if &search_point == point_b {
                search_stack.push(search_point);
                found_goal = true;
                break;
            }            

            let mut search_candidates : Vec<SearchCandidate> = Vec::new();
            self.get_search_candidates(&search_point, &point_b, &mut search_candidates);

            if !search_candidates.is_empty() {
                search_stack.push(search_point);
                search_candidates.sort_by(|a, b| b.dist.cmp(&a.dist));
                candidates.append(&mut search_candidates);
            }
        }

        assert_eq!(found_goal, true);
        let first_item = search_stack.remove(0);
        assert_eq!(&first_item, point_a);

        let shortest_path = search_stack.iter().map(|p| self.get_cell_cost(p.x, p.y)).sum();
        //println!("Shortest path between galaxies {galaxy_a} and {galaxy_b} is {shortest_path}");
        shortest_path
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

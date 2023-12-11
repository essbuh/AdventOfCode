use std::mem::swap;

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

    fn get_manhattan_distance(self: &Self, galaxy_a: usize, galaxy_b: usize) -> usize {
        let point_a = self.galaxies[galaxy_a];
        let point_b = self.galaxies[galaxy_b];

        let mut start_x = point_a.x;
        let mut end_x = point_b.x;
        if start_x > end_x {
            swap(&mut start_x, &mut end_x);
        }

        let mut start_y = point_a.y;
        let mut end_y = point_b.y;
        if start_y > end_y {
            swap(&mut start_y, &mut end_y);
        }

        let expansion_count = (
            (start_x..end_x).filter(|x| self.rows[0][*x] == EXPANSION_CHAR).count(),
            (start_y..end_y).filter(|y| self.rows[*y][0] == EXPANSION_CHAR).count());

        let result = point_a.x.abs_diff(point_b.x)
            + point_a.y.abs_diff(point_b.y)
            + (expansion_count.0 * (self.expansion_cost-1))
            + (expansion_count.1 * (self.expansion_cost-1));

        //println!("Result of {galaxy_a} -> {galaxy_b}: {result}");
        result
    }

    fn get_sum_manhattan_distance(self: &Self) -> usize {
        let mut total_sum = 0;
        for i in 0..self.galaxies.len() {
            for j in (i+1)..self.galaxies.len() {
                total_sum = total_sum + self.get_manhattan_distance(i, j);
            }
        }
        total_sum
    }
}

fn part_1() {
    let input = include_str!("input.txt");
    let map = GalaxyMap::parse(input, 2);
    //print_map(&map);

    let shortest_paths = map.get_sum_manhattan_distance();
    println!("Part 1: {shortest_paths}");
}

fn part_2() {    
    let input = include_str!("input.txt");
    let map = GalaxyMap::parse(input, 1000000);
    //print_map(&map);

    let shortest_paths = map.get_sum_manhattan_distance();
    println!("Part 2: {shortest_paths}");
}

fn main() {
    part_1();
    part_2();
}

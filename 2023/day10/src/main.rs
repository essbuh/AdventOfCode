use std::collections::{HashMap, HashSet};

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
struct Point { row: i32, col: i32 }
impl Point {
    fn new() -> Point {
        Point { row: 0, col: 0 }
    }
}

#[derive(Debug)]
struct PipeMap {
    connections: HashMap<Point, Vec<Point>>,
    start: Point
}
impl PipeMap {
    fn get_connection(self: &mut Self, point: &Point) -> &mut Vec<Point> {
        self.connections.entry(*point).or_insert(Vec::new())
    }

    fn connect_points(self: &mut Self, point: &Point, left: &Point, right: &Point) {
        let connection = self.get_connection(point);
        connection.push(*left);
        connection.push(*right);
    }

    fn set_start_connections(self: &mut Self, connections: &mut Vec<Point>) {
        let start_conn = self.connections.entry(self.start).or_insert(Vec::new());
        start_conn.append(connections);
    }

    fn find_loop(self: &Self, start: &Point) -> i32 {
        let mut visited_points : HashSet<&Point> = HashSet::new();

        let mut point = start;
        let mut last_point = point;

        let mut loop_point : Option<&Point> = None;
        loop {
            if visited_points.contains(point) {
                loop_point = Some(point);
                break;
            }

            visited_points.insert(point);

            match self.connections.get(point) {
                Some(connections) => {
                    match connections.iter().find(|p| p != &last_point) {
                        Some(p) => { last_point = point; point = p; },
                        None => { break; }
                    }
                }
                None => { break; }
            }
        }

        //println!("Visited points: {:?}", visited_points);

        match loop_point {
            Some(p) => { assert_eq!(p, start); /*println!("Found loop after {} points", visited_points.len());*/ visited_points.len() as i32 },
            None => { -1 },
        }
    }
}

fn parse_input(input: &str) -> PipeMap {
    let mut map = PipeMap { connections: HashMap::new(), start: Point::new() };

    for (i, line) in input.lines().enumerate() {
        for (j, char) in line.chars().enumerate() {
            let row = i as i32;
            let col = j as i32;

            let point = Point { row, col };
            
            let mut connections: Option<(Point, Point)> = None;

            match char {
                '|' => { connections = Some((Point { row: row-1, col }, Point { row: row+1, col })); }
                '-' => { connections = Some((Point { row, col: col-1 }, Point { row, col: col+1 })); },
                'L' => { connections = Some((Point { row, col: col+1 }, Point { row: row-1, col })); },
                'J' => { connections = Some((Point { row: row-1, col }, Point { row, col:col-1 } )); },
                'F' => { connections = Some((Point { row: row+1, col }, Point { row, col:col+1 } )); },
                '7' => { connections = Some((Point { row, col: col-1 }, Point { row: row+1, col } )); },
                'S' => { map.start = point },
                _ => { /* skip */}
            }

            match &connections {
                Some(p) => { map.connect_points(&point, &p.0, &p.1); }
                None => {} 
            }
        }
    }

    // Connect the starting point
    let mut start_connections = map.connections.iter()
            .filter(|(_, conn)| conn.contains(&map.start))
            .map(|p| *p.0 )
            .collect();
    map.set_start_connections(&mut start_connections);

    map
}

fn part_1(input: &str) {
    let map = parse_input(input);
    //println!("Map: {:#?}", map);
    let loop_count = map.find_loop(&map.start);
    assert!(loop_count > 0);
    let max_dist = ((loop_count as f32) / 2.0).ceil() as i32;
    println!("Part 1: {max_dist}");
}

fn main() {
    let input = include_str!("input.txt");
    part_1(&input);
}

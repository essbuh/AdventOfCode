use std::collections::{HashMap, HashSet};

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
struct Point { row: i32, col: i32 }
impl Point {
    fn new() -> Point {
        Point { row: 0, col: 0 }
    }

    fn from_yx(row: i32, col: i32) -> Point {
        Point { row, col }
    }

    fn add(self: &Self, dir: &Direction) -> Point {
        Point::from_yx(self.row + dir.0, self.col + dir.1)
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

    fn find_loop(self: &Self) -> Vec<Point> {
        let mut visited_points : Vec<Point> = Vec::new();

        let mut point = &self.start;
        let mut last_point = point;

        let mut loop_point : Option<&Point> = None;
        loop {
            if visited_points.contains(point) {
                loop_point = Some(point);
                break;
            }

            visited_points.push(*point);

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
            Some(p) => { assert_eq!(p, &self.start); },
            None => {},
        }

        //println!("Found loop after {} points", visited_points.len());
        visited_points
    }
}

#[allow(dead_code)]
fn print_pipe(pipe: &FloodPipe) -> String {
    let mut str = String::new();
    str += "L(";
    str += &pipe.left.pos.row.to_string();
    str += ",";
    str += &pipe.left.pos.col.to_string();
    str += " ";
    str += &pipe.left.symbol.to_string();
    str += "), R(";
    str += &pipe.right.pos.row.to_string();
    str += ",";
    str += &pipe.right.pos.col.to_string();
    str += " ";
    str += &pipe.right.symbol.to_string();
    str += ") dir(";
    str += &pipe.direction.0.to_string();
    str += ",";
    str += &pipe.direction.1.to_string();
    str += ")";
    str
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

fn part_1() {
    let input = include_str!("input.txt");
    let map = parse_input(input);
    //println!("Map: {:#?}", map);
    let loop_count = map.find_loop().len();
    assert!(loop_count > 0);
    let max_dist = ((loop_count as f32) / 2.0).ceil() as i32;
    println!("Part 1: {max_dist}");
}

/////////////////////////////

type Direction = (i32, i32);
//const DIR_NONE : Direction = (0, 0);
const DIR_LEFT : Direction = (0, -1);
const DIR_RIGHT : Direction = (0, 1);
const DIR_UP : Direction = (-1, 0);
const DIR_DOWN : Direction = (1, 0);

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
struct FloodPoint {
    symbol: char,
    pos: Point,
}

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
struct FloodPipe {
    left: FloodPoint,
    right: FloodPoint,
    direction: Direction
}

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
enum FloodStackEntry {
    Single(FloodPoint),
    Pipe(FloodPipe),
}
impl FloodStackEntry {
    fn from_single(map: &FloodMap, point: &Point) -> FloodStackEntry {
        let char = map.get(point).unwrap();
        FloodStackEntry::Single(FloodPoint { symbol: char, pos: *point })
    }

    fn from_pipe(map: &FloodMap, left: &Point, right: &Point, dir: &Direction) -> FloodStackEntry {
        let char_l = map.get(left).unwrap();
        let char_r = map.get(right).unwrap();
        FloodStackEntry::Pipe(FloodPipe {
            left: FloodPoint { symbol: char_l, pos: *left },
            right: FloodPoint { symbol: char_r, pos: *right },
            direction: *dir})
    }
}

fn is_pipe_open_up(left: char, right: char) -> bool {
    !((left == '.' || left == 'O') && (right == '.' || right == 'O')) &&
    left != 'F' && left != 'L' && left != '-' &&
        right != 'J' && right != '7' && right != '-'
}

fn is_pipe_open_down(left: char, right: char) -> bool {
    !((left == '.' || left == 'O') && (right == '.' || right == 'O')) &&
    left != 'J' && left != '7' && left != '-' &&
        right != 'F' && right != 'L' && right != '-'
}

fn is_pipe_open_left(left: char, right: char) -> bool {
    !((left == '.' || left == 'O') && (right == '.' || right == 'O')) &&
    left != 'J' && left != 'L' && left != '|' &&
        right != 'F' && right != '7' && right != '|'
}

fn is_pipe_open_right(left: char, right: char) -> bool {
    !((left == '.' || left == 'O') && (right == '.' || right == 'O')) &&
    left != 'F' && left != '7' && left != '|' &&
        right != 'J' && right != 'L' && right != '|'
}

fn get_connecting_pipes(map: &FloodMap, pipe: &FloodPipe, flood_stack: &mut Vec<FloodStackEntry>) {
    let next_left = pipe.left.pos.add(&pipe.direction);
    let next_char_l = map.get(&next_left);

    let next_right = pipe.right.pos.add(&pipe.direction);
    let next_char_r = map.get(&next_right);

    let mut exited_pipe = false;
    if next_char_l.is_some() && next_char_l.unwrap() == '.' {
        //println!("Got output to ({},{}) from pipe {}", next_left.row,next_left.col, print_pipe(&pipe));
        flood_stack.push(FloodStackEntry::from_single(map, &next_left));
        exited_pipe = true;
    }
    if next_char_r.is_some() && next_char_r.unwrap() == '.' {
        //println!("Got output to ({},{}) from pipe {}", next_right.row,next_right.col, print_pipe(&pipe));
        flood_stack.push(FloodStackEntry::from_single(map, &next_right));
        exited_pipe = true;
    }

    if exited_pipe {
        return;
    }
    
    if pipe.direction.0 != 0 {
        if pipe.direction.0 < 0 {
            // going up
            // Check in same direction
            if next_char_l.is_some() && next_char_r.is_some() {
                if is_pipe_open_up(next_char_l.unwrap(), next_char_r.unwrap()) {
                    let next_pipe = FloodStackEntry::from_pipe(map, &next_left, &next_right, &pipe.direction);
                    //match next_pipe { FloodStackEntry::Pipe(x) => { println!("Going to pipe {} from pipe {}", print_pipe(&x), print_pipe(&pipe)); }, _ => {}}
                    flood_stack.push(next_pipe);
                }
            }
            if next_char_r.is_some() {
                if is_pipe_open_right(next_char_r.unwrap(), pipe.right.symbol) {
                    let next_pipe = FloodStackEntry::from_pipe(map, &next_right, &pipe.right.pos, &DIR_RIGHT);
                    //match next_pipe { FloodStackEntry::Pipe(x) => { println!("Going to pipe {} from pipe {}", print_pipe(&x), print_pipe(&pipe)); }, _ => {}}
                    flood_stack.push(next_pipe);
                }
            }
            if next_char_l.is_some() {
                if is_pipe_open_left(pipe.left.symbol, next_char_l.unwrap()) {
                    let next_pipe = FloodStackEntry::from_pipe(map, &pipe.left.pos, &next_left, &DIR_LEFT);
                    //match next_pipe { FloodStackEntry::Pipe(x) => { println!("Going to pipe {} from pipe {}", print_pipe(&x), print_pipe(&pipe)); }, _ => {}}
                    flood_stack.push(next_pipe);
                }
            }
        } else {
            // going down
            // Check in same direction
            if next_char_l.is_some() && next_char_r.is_some() {
                if is_pipe_open_down(next_char_l.unwrap(), next_char_r.unwrap()) {
                    let next_pipe = FloodStackEntry::from_pipe(map, &next_left, &next_right, &pipe.direction);
                    //match next_pipe { FloodStackEntry::Pipe(x) => { println!("Going to pipe {} from pipe {}", print_pipe(&x), print_pipe(&pipe)); }, _ => {}}
                    flood_stack.push(next_pipe);
                }
            }
            if next_char_r.is_some() {
                if is_pipe_open_left(next_char_r.unwrap(), pipe.right.symbol) {
                    let next_pipe = FloodStackEntry::from_pipe(map, &next_right, &pipe.right.pos, &DIR_LEFT);
                    //match next_pipe { FloodStackEntry::Pipe(x) => { println!("Going to pipe {} from pipe {}", print_pipe(&x), print_pipe(&pipe)); }, _ => {}}
                    flood_stack.push(next_pipe);
                }
            }
            if next_char_l.is_some() {
                if is_pipe_open_right(pipe.left.symbol, next_char_l.unwrap()) {
                    let next_pipe = FloodStackEntry::from_pipe(map, &pipe.left.pos, &next_left, &DIR_RIGHT);
                    //match next_pipe { FloodStackEntry::Pipe(x) => { println!("Going to pipe {} from pipe {}", print_pipe(&x), print_pipe(&pipe)); }, _ => {}}
                    flood_stack.push(next_pipe);
                }
            }
        }
    } else {        
        // horizontal pipe
        if pipe.direction.1 < 0 {
            // going left
            // Check in same direction
            if next_char_l.is_some() && next_char_r.is_some() {
                if is_pipe_open_left(next_char_l.unwrap(), next_char_r.unwrap()) {
                    let next_pipe = FloodStackEntry::from_pipe(map, &next_left, &next_right, &pipe.direction);
                    //match next_pipe { FloodStackEntry::Pipe(x) => { println!("Going to pipe {} from pipe {}", print_pipe(&x), print_pipe(&pipe)); }, _ => {}}
                    flood_stack.push(next_pipe);
                }
            }
            if next_char_r.is_some() {
                if is_pipe_open_up(next_char_r.unwrap(), pipe.right.symbol) {
                    let next_pipe = FloodStackEntry::from_pipe(map, &next_right, &pipe.right.pos, &DIR_UP);
                    //match next_pipe { FloodStackEntry::Pipe(x) => { println!("Going to pipe {} from pipe {}", print_pipe(&x), print_pipe(&pipe)); }, _ => {}}
                    flood_stack.push(next_pipe);
                }
            }
            if next_char_l.is_some() {
                if is_pipe_open_down(pipe.left.symbol, next_char_l.unwrap()) {
                    let next_pipe = FloodStackEntry::from_pipe(map, &pipe.left.pos, &next_left, &DIR_DOWN);
                    //match next_pipe { FloodStackEntry::Pipe(x) => { println!("Going to pipe {} from pipe {}", print_pipe(&x), print_pipe(&pipe)); }, _ => {}}
                    flood_stack.push(next_pipe);
                }
            }
        } else {
            // Check in same direction
            if next_char_l.is_some() && next_char_r.is_some() {
                if is_pipe_open_right(next_char_l.unwrap(), next_char_r.unwrap()) {
                    let next_pipe = FloodStackEntry::from_pipe(map, &next_left, &next_right, &pipe.direction);
                    //match next_pipe { FloodStackEntry::Pipe(x) => { println!("Going to pipe {} from pipe {}", print_pipe(&x), print_pipe(&pipe)); }, _ => {}}
                    flood_stack.push(next_pipe);
                }
            }
            // going right
            if next_char_r.is_some() {
                if is_pipe_open_down(next_char_r.unwrap(), pipe.right.symbol) {
                    let next_pipe = FloodStackEntry::from_pipe(map, &next_right, &pipe.right.pos, &DIR_DOWN);
                    //match next_pipe { FloodStackEntry::Pipe(x) => { println!("Going to pipe {} from pipe {}", print_pipe(&x), print_pipe(&pipe)); }, _ => {}}
                    flood_stack.push(next_pipe);
                }
            }
            if next_char_l.is_some() {
                if is_pipe_open_up(pipe.left.symbol, next_char_l.unwrap()) {
                    let next_pipe = FloodStackEntry::from_pipe(map, &pipe.left.pos, &next_left, &DIR_UP);
                    //match next_pipe { FloodStackEntry::Pipe(x) => { println!("Going to pipe {} from pipe {}", print_pipe(&x), print_pipe(&pipe)); }, _ => {}}
                    flood_stack.push(next_pipe);
                }
            }
        }
    }
}

fn collect_pipes(map: &FloodMap, point: &Point, dir: &Direction, flood_stack: &mut Vec<FloodStackEntry>) {
    if dir.0 != 0 {
        // Try left & right in direction
        let point_left = Point::from_yx(point.row, point.col - 1);
        let point_right = Point::from_yx(point.row, point.col + 1);

        let char = map.get(&point).unwrap();
        
        let char_left = map.get(&point_left);
        if char_left.is_some() {
            if dir.0 < 0 {
                if is_pipe_open_up(char_left.unwrap(), char) {
                    let next_pipe = FloodStackEntry::from_pipe(map, &point_left, &point, dir);
                    //match next_pipe { FloodStackEntry::Pipe(x) => { println!("Going to pipe {} from point ({},{})", print_pipe(&x), point.row,point.col); }, _ => {}}
                    flood_stack.push(next_pipe);
                }
            } else {
                if is_pipe_open_down(char, char_left.unwrap()) {
                    let next_pipe = FloodStackEntry::from_pipe(map, &point, &point_left, dir);
                    //match next_pipe { FloodStackEntry::Pipe(x) => { println!("Going to pipe {} from point ({},{})", print_pipe(&x), point.row,point.col); }, _ => {}}
                    flood_stack.push(next_pipe);
                }
            }            
        }
        
        let char_right = map.get(&point_right);
        if char_right.is_some() {
            if dir.0 < 0 {
                if is_pipe_open_up(char, char_right.unwrap()) {
                    let next_pipe = FloodStackEntry::from_pipe(map, &point, &point_right, dir);
                    //match next_pipe { FloodStackEntry::Pipe(x) => { println!("Going to pipe {} from point ({},{})", print_pipe(&x), point.row,point.col); }, _ => {}}
                    flood_stack.push(next_pipe);
                }
            } else {
                if is_pipe_open_down(char_right.unwrap(), char) {
                    let next_pipe = FloodStackEntry::from_pipe(map, &point_right, &point, dir);
                    //match next_pipe { FloodStackEntry::Pipe(x) => { println!("Going to pipe {} from point ({},{})", print_pipe(&x), point.row,point.col); }, _ => {}}
                    flood_stack.push(next_pipe);
                }
            }
        }
    } else {
        // Try up & down in direction
        let point_up = Point::from_yx(point.row - 1, point.col);
        let point_down = Point::from_yx(point.row + 1, point.col);

        let char = map.get(&point).unwrap();
        
        let char_down = map.get(&point_down);
        if char_down.is_some() {
            if dir.1 < 0 {
                // left
                if is_pipe_open_left(char_down.unwrap(), char) {
                    let next_pipe = FloodStackEntry::from_pipe(map, &point_down, &point, dir);
                    //match next_pipe { FloodStackEntry::Pipe(x) => { println!("Going to pipe {} from point ({},{})", print_pipe(&x), point.row,point.col); }, _ => {}}
                    flood_stack.push(next_pipe);
                }
            } else {
                // right
                if is_pipe_open_right(char, char_down.unwrap()) {
                    let next_pipe = FloodStackEntry::from_pipe(map, &point, &point_down, dir);
                    //match next_pipe { FloodStackEntry::Pipe(x) => { println!("Going to pipe {} from point ({},{})", print_pipe(&x), point.row,point.col); }, _ => {}}
                    flood_stack.push(next_pipe);
                }
            }
        }
        
        let char_up = map.get(&point_up);
        if char_up.is_some() {
            if dir.1 < 0 {
                // left
                if is_pipe_open_left(char, char_up.unwrap()) {
                    let next_pipe = FloodStackEntry::from_pipe(map, &point, &point_up, dir);
                    //match next_pipe { FloodStackEntry::Pipe(x) => { println!("Going to pipe {} from point ({},{})", print_pipe(&x), point.row,point.col); }, _ => {}}
                    flood_stack.push(next_pipe);
                }
            } else {
                // right
                if is_pipe_open_right(char_up.unwrap(), char) {
                    let next_pipe = FloodStackEntry::from_pipe(map, &point_up, &point, dir);
                    //match next_pipe { FloodStackEntry::Pipe(x) => { println!("Going to pipe {} from point ({},{})", print_pipe(&x), point.row,point.col); }, _ => {}}
                    flood_stack.push(next_pipe);
                }
            }
        }
    }
}

#[derive(Debug)]
struct FloodMap {
    chars: Vec<Vec<char>>,
}
impl FloodMap {
    fn contains(self: &Self, pos: &Point) -> bool {
        pos.row >= 0 && pos.row < self.chars.len() as i32 && pos.col >= 0 && pos.col < self.chars[pos.row as usize].len() as i32
    }

    fn get(self: &Self, pos: &Point) -> Option<char> {
        if self.contains(pos) {
            Some(self.chars[pos.row as usize][pos.col as usize])
        } else {
            None
        }
    }

    fn set_char(self: &mut Self, pos: &Point, char: char) {
        assert!(self.contains(pos));
        
        let my_char = &mut self.chars[pos.row as usize][pos.col as usize];
        *my_char = char;

        //println!("--------");
        //for line in &self.chars { println!("{:?}", line); }
    }

    fn initialize(self: &mut Self, input: &str) {
        let pipe_map = parse_input(input);
        let pipe_loop = pipe_map.find_loop();

        assert!(!pipe_loop.is_empty());
        // Replace starting point with what we know it to be
        let start_out = pipe_loop[1];
        let start_in = pipe_loop.last().unwrap();
        let start_char : char;
        
        if start_out.col != pipe_map.start.col {
            // Went horizontal
            let went_right = start_out.col > pipe_map.start.col;
            if start_in.row > pipe_map.start.row {
                // Came up
                start_char = if went_right { 'F' } else { '7' };
            } else {
                // Came down
                start_char = if went_right { 'L' } else { 'J' };
            }
        } else {
            // Went vertical
            let went_up = start_out.row < pipe_map.start.row;
            if start_in.col < pipe_map.start.col {
                // Came left
                start_char = if went_up { 'J' } else { '7' };
            } else {
                // Came right
                start_char = if went_up { 'L' } else { 'F' };
            }
        }

        for line in input.lines() {
            self.chars.push(line.chars().collect());
        }

        // Replace start char with detected pipe type
        self.chars[pipe_map.start.row as usize][pipe_map.start.col as usize] = start_char;

        // Mark anything not in the pipe loop as an unknown point
        let map_height = self.chars.len();
        let map_width = self.chars[0].len();
        for row in 0..map_height {
            for col in 0..map_width {
                if !pipe_loop.contains(&Point{ row: row as i32, col: col as i32 }) {
                    self.chars[row][col] = '.';
                }
            }
        }
    }

    fn border_flood(self: &mut Self) {
        assert!(!self.chars.is_empty());

        let mut flood_stack : Vec<FloodStackEntry> = Vec::new();
        let map_width = self.chars[0].len() as i32;
        let map_height = self.chars.len() as i32;
        for i in 0..map_height {
            flood_stack.push(FloodStackEntry::from_single(self, &Point::from_yx(i, 0)));
            flood_stack.push(FloodStackEntry::from_single(self, &Point::from_yx(i, map_width - 1)));
        }
        for i in 0..map_width {
            flood_stack.push(FloodStackEntry::from_single(self, &Point::from_yx(0, 1)));
            flood_stack.push(FloodStackEntry::from_single(self, &Point::from_yx(map_height - 1, i)));
        }

        let mut visited_cells: HashSet<FloodStackEntry> = HashSet::new();
        let directions : Vec<Direction> = vec![ DIR_UP, DIR_DOWN, DIR_LEFT, DIR_RIGHT ];

        loop {
            match flood_stack.pop() {
                Some(next_pos) => {
                    if visited_cells.contains(&next_pos) {
                        continue;
                    }

                    //println!("Visiting: {:?}", &next_pos);
                    visited_cells.insert(next_pos.clone());

                    match next_pos {
                        FloodStackEntry::Single(item) => {
                            if self.contains(&item.pos) {
                                if item.symbol == '.' {
                                    self.set_char(&item.pos, 'O');
                                    //println!("Got to ({},{})", item.pos.row,item.pos.col);

                                    for direction in &directions {
                                        let next_point = item.pos.add(direction);
                                        match self.get(&next_point) {
                                            Some(p) => {
                                                if p == '.' {
                                                    flood_stack.push(FloodStackEntry::from_single(&self, &next_point))
                                                } else {
                                                    collect_pipes(&self, &next_point, &direction, &mut flood_stack);
                                                }
                                            },
                                            None => {}
                                        }
                                    }
                                }
                            }
                        },
                        FloodStackEntry::Pipe(pipe) => {
                            if pipe.left.symbol == '.' {
                                flood_stack.push(FloodStackEntry::from_single(&self, &pipe.left.pos))
                            } else if pipe.right.symbol == '.' {
                                flood_stack.push(FloodStackEntry::from_single(&self, &pipe.right.pos))
                            } else {
                                //println!("Checking pipe {}", print_pipe(&pipe));
                                get_connecting_pipes(&self, &pipe, &mut flood_stack);
                            }
                        }
                    }
                },
                None => { break; }
            }
        }
    }
}

fn part_2() {
    let input = include_str!("input.txt");
    let mut map = FloodMap { chars: Vec::new() };
    map.initialize(input);
    
    map.border_flood();

    let num_inside : usize = map.chars
        .iter()
        .map(|line| line.into_iter().filter(|x| *x == &'.').count())
        .sum();
    
    //for line in &map.chars { println!("{:?}", line); }
    
    println!("Num Inside: {num_inside}");
}

fn main() {
    part_1();
    part_2();
}

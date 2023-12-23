use std::collections::{HashMap, HashSet, VecDeque};

pub type Direction = (i32, i32);
const DIR_UP: Direction = (0, -1);
const DIR_DOWN: Direction = (0, 1);
const DIR_LEFT: Direction = (-1, 0);
const DIR_RIGHT: Direction = (1, 0);

pub type Path = (HashSet<Point>, Point, usize);

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub struct Point {
    x: i32,
    y: i32
}
impl Point {
    pub fn from_xy(x: i32, y: i32) -> Self { Point { x, y } }
    
    pub fn add_dir(&self, dir: &Direction) -> Self {
        Point { x: self.x + dir.0, y: self.y + dir.1 }
    }

    pub fn dir_to(&self, other: &Point) -> Direction {
        ( other.x - self.x, other.y - self.y )
    }
}

#[derive(Debug)]
pub struct Maze
{
    pub chars: Vec<Vec<char>>,
    dim: (usize, usize),
    ignore_slopes: bool,
    pub entry: Point,
    exit: Point,
    pub connections: HashMap<Point, HashSet<Point>>,
    pub direct_connections: HashMap<Point, Vec<(Point, usize, Vec<Point>)>>,
}
impl Maze {
    pub fn from_input(input: &str, ignore_slopes: bool) -> Self {
        let all_dirs = vec![ DIR_UP, DIR_DOWN, DIR_LEFT, DIR_RIGHT ];
        let all_chars : Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
        let height = all_chars.len();
        let width = all_chars[0].len();

        let mut maze = Maze {
            chars: all_chars,
            dim: (width, height),
            ignore_slopes,
            entry: Point { x: 0, y: 0 },
            exit: Point { x: 0, y: 0 },
            connections: HashMap::new(),
            direct_connections: HashMap::new(),
        };

        {
            let add_dir = |maze: &mut Maze, point: &Point, dir: &Direction| {
                let next_point = point.add_dir(&dir);
                if maze.is_passable(&next_point, &dir) {
                    maze.add_connection(&point, &next_point);
                }
            };
            let add_all_dirs = |maze: &mut Maze, point: &Point| {
                for dir in &all_dirs {
                    add_dir(maze, point, dir);
                }
            };
            
            for row in 0..height {
                for col in 1..width-1 { // No need to check edges due to forest border
                    let point = Point { x: col as i32, y: row as i32 };
    
                    match maze.chars[row][col] {
                        '.' => {
                            if row == 0 { maze.entry.x = col as i32 }
                            else if row == height - 1 { maze.exit = point.clone(); }
    
                            add_all_dirs(&mut maze, &point);
                        },
                        '^' => {
                            if maze.ignore_slopes {
                                add_all_dirs(&mut maze, &point);
                            } else {
                                add_dir(&mut maze, &point, &DIR_UP);
                            }
                        },
                        'v' => {
                            if maze.ignore_slopes {
                                add_all_dirs(&mut maze, &point);
                            } else {
                                add_dir(&mut maze, &point, &DIR_DOWN);
                            }
                        },
                        '>' => {
                            if maze.ignore_slopes {
                                add_all_dirs(&mut maze, &point);
                            } else {
                                add_dir(&mut maze, &point, &DIR_RIGHT);
                            }
                        },
                        '<' => {
                            if maze.ignore_slopes {
                                add_all_dirs(&mut maze, &point);
                            } else {
                                add_dir(&mut maze, &point, &DIR_LEFT);
                            }
                        },
                        _ => {}
                    }
                }
            }
        }

        maze.collapse_connections(maze.entry.clone());

        maze
    }
    
    fn collapse_connections(&mut self, start_point: Point) {        
        let mut visited_points = HashSet::new();
        let mut point_queue = VecDeque::new();

        point_queue.push_back(start_point);

        while let Some(p) = point_queue.pop_front() {
            if visited_points.contains(&p) {
                continue;
            }

            visited_points.insert(p);

            if let Some(connections) = self.connections.get(&p).cloned() {
                for connection in connections {
                    let dir = p.dir_to(&connection);
                    let segment = self.get_path_segment(&p, &dir);
                    if !segment.is_empty() {
                        assert_eq!(&segment[0], &p);
    
                        let segment_end = *segment.last().unwrap();
                        
                        self.direct_connections
                            .entry(p).or_default()
                            .push((segment_end, segment.len() - 1, segment));
    
                        point_queue.push_back(segment_end);
                    }
                }
            }
        }
    }

    fn is_passable(&self, point: &Point, traveled_dir: &Direction) -> bool {
        if point.x < 0 || point.x >= self.dim.0 as i32
            || point.y < 0 || point.y >= self.dim.1 as i32
        {
            return false;
        }
    
        match self.chars[point.y as usize][point.x as usize] {
            '.' => { true },
            '>' => { self.ignore_slopes || traveled_dir == &DIR_RIGHT },
            '<' => { self.ignore_slopes || traveled_dir == &DIR_LEFT },
            '^' => { self.ignore_slopes || traveled_dir == &DIR_UP },
            'v' => { self.ignore_slopes || traveled_dir == &DIR_DOWN },
            _ => { false }
        }
    
    }

    fn add_connection(&mut self, from: &Point, to: &Point) {
        self.connections.entry(*from).or_default().insert(*to);
    }

    fn get_path_segment(&self, start_point: &Point, dir: &Direction) -> Vec<Point> {
        let mut point = start_point.add_dir(dir); 
        let mut last_point = *start_point;       
        
        let mut segment = vec![ last_point, point ];

        while let Some(connections) = self.connections.get(&point) {
            let connections : HashSet<&Point> = HashSet::from_iter(connections.iter().filter(|&p| p != &last_point));
            if connections.len() == 1 {
                let next_point = *connections.iter().next().unwrap();
                if !segment.contains(next_point) {
                    segment.push(*next_point);
                    last_point = point;
                    point = *next_point;
                    continue;
                }   
            } else if connections.is_empty() && &point != &self.exit {
                // Hit a dead end, this is not a valid segment
                return Vec::new();
            }

            break;
        }

        segment
    }

    pub fn get_paths(&self, debug: bool) -> Vec<Path> {
        let mut final_paths = Vec::new();

        let mut path_queue : VecDeque<Path> = VecDeque::new();
        path_queue.push_front((HashSet::from([self.entry]), self.entry, 0));

        while let Some(path) = path_queue.pop_front() {
            if debug { println!("Testing path: {path:?}"); }
            let last_point = &path.1;
            if last_point == &self.exit {
                // Reached the end!
                final_paths.push(path);
                continue;
            }

            if let Some(connections) = self.direct_connections.get(&last_point) {
                let connections = connections.iter()
                    .filter(|&c| !path.0.contains(&c.0));

                if self.ignore_slopes {
                    if let Some(exit_path) = connections.clone()
                        .find(|&c| &c.0 == &self.exit)
                    {
                        let mut path = path.clone();
                        path.0.insert(exit_path.0);
                        path.1 = exit_path.0;
                        path.2 += exit_path.1;

                        path_queue.push_front(path);
                        continue;
                    }
                }
                
                for connection in connections {
                    let mut path = path.clone();
                    path.0.insert(connection.0);
                    path.1 = connection.0;
                    path.2 += connection.1;
                    path_queue.push_front(path);
                }
            }
        }

        final_paths
    }
}
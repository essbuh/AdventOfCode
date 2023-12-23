#![allow(dead_code)]

use std::{mem::swap, cmp::{min,max}, iter, collections::{HashSet, HashMap}};
type PointType = usize;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: PointType,
    pub y: PointType,
    pub z: PointType,
}
impl Point {
    pub fn new() -> Point { Point { x:0, y:0, z:0 } }

    pub fn from_xyz(x: PointType, y: PointType, z: PointType) -> Point { Point { x, y, z } }

    pub fn from_input(input: &str) -> Point {
        let values : Vec<&str> = input.split(',').collect();
        Point {
            x: values[0].parse().unwrap(),
            y: values[1].parse().unwrap(),
            z: values[2].parse().unwrap(),
        }
    }

    fn supports(self: &Self, other: &Point) -> bool {
        self.z == other.z - 1
            && (self.x == other.x || self.y == other.y)
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Brick {
    pub left: Point,
    pub right: Point,
    pub label: char,
}
impl Brick {
    pub fn from_input(input: &str, label: char) -> Brick {
        let points : Vec<&str> = input.split('~').collect();
        let mut left = Point::from_input(&points[0]);
        let mut right = Point::from_input(&points[1]);

        if right.z < left.z || right.y < left.y || right.x < left.x {
            swap(&mut left, &mut right);
        }

        Brick { left, right, label }
    }

    fn is_supported_by(self: &Self, other: &Brick) -> bool {
        other.right.z == self.left.z - 1 
            && self.right.x >= other.left.x && other.right.x >= self.left.x
            && self.right.y >= other.left.y && other.right.y >= self.left.y
    }
}

#[derive(Debug)]
pub struct BrickTower {
    pub bricks: Vec<Brick>,
    pub bounds: (Point, Point),
    pub bricks_by_z: Vec<Vec<usize>>,
    supports: HashMap<usize, Vec<usize>>,
    supported_by: HashMap<usize, Vec<usize>>,
}
impl BrickTower {
    pub fn from_input(input: &str) -> BrickTower {
        let mut bricks = Vec::new();

        let brick_chars : Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789".chars().collect();
        
        for (i, line) in input.lines().enumerate() {
            let label = brick_chars[i % brick_chars.len()];
            bricks.push(Brick::from_input(line, label));
        }

        let mut tower = BrickTower { 
            bricks, 
            bounds: (Point::new(), Point::new()), 
            bricks_by_z: Vec::new(),
            supported_by: HashMap::new(),
            supports: HashMap::new(),
         };
        tower.update_cache();
        tower
    }

    fn calc_bounds(self: &mut Self) {
        let mut bounds_min = Point::from_xyz(PointType::MAX, PointType::MAX, PointType::MAX);
        let mut bounds_max = Point::from_xyz(0, 0, 0);

        for brick in &self.bricks {
            bounds_min.x = min(bounds_min.x, brick.left.x);
            bounds_min.y = min(bounds_min.y, brick.left.y);
            bounds_min.z = min(bounds_min.z, brick.left.z);

            bounds_max.x = max(bounds_max.x, brick.right.x);
            bounds_max.y = max(bounds_max.y, brick.right.y);
            bounds_max.z = max(bounds_max.z, brick.right.z);
        }

        self.bounds = (bounds_min, bounds_max);
    }

    fn cache_z(self: &mut Self) {
        self.bricks_by_z = iter::repeat(Vec::new()).take(self.bounds.1.z + 1).collect();

        for z in 0..=self.bounds.1.z {
            let bricks = &mut self.bricks_by_z[z];
            
            for (i, brick) in self.bricks.iter().enumerate() {
                if brick.left.z <= z &&  brick.right.z >= z {
                    bricks.push(i);
                }
            }
        }
    }

    fn update_cache(self: &mut Self) {
        self.bricks.sort_by(|a, b| {
            a.left.z.cmp(&b.left.z)
        });

        self.calc_bounds();
        self.cache_z();
    }

    pub fn drop_bricks(self: &mut Self) {
        // Minimum z, given X & Y
        let max_dim = max(self.bounds.1.x, self.bounds.1.y);
        let mut max_brick_z : Vec<Vec<PointType>> = (0..=max_dim).map(|_| {
            iter::repeat(0).take(max_dim + 1).collect()
        }).collect();

        for brick in self.bricks.iter_mut() {
            let mut ceil_z = 0;
            for x in brick.left.x..=brick.right.x {
                for y in brick.left.y..=brick.right.y {
                    ceil_z = max(ceil_z, max_brick_z[x][y] + 1);
                }
            }

            if ceil_z < brick.left.z {                        
                let drop_z = brick.left.z - ceil_z;
                brick.left.z -= drop_z;
                brick.right.z -= drop_z;
            }

            for x in brick.left.x..=brick.right.x {
                for y in brick.left.y..=brick.right.y {
                    max_brick_z[x][y] = max(max_brick_z[x][y], brick.right.z);
                }
            }
        }

        self.update_cache();
        self.update_supports();

        assert_eq!((1..self.bricks_by_z.len()).filter(|x| self.bricks_by_z[*x].is_empty()).count(), 0);
    }

    fn update_supports(self: &mut Self) {
        self.supported_by.clear();
        self.supports.clear();

        for i in 0..self.bricks.len() {
            let brick = &self.bricks[i];

            for j in i+1..self.bricks.len() {
                let brick_b = &self.bricks[j];

                if brick_b.is_supported_by(brick) {
                    self.supported_by.entry(i).or_default().push(j);
                    self.supports.entry(j).or_default().push(i);
                }
            }
        }
    }

    pub fn get_supports(self: &Self, brick_idx: usize) -> Vec<usize> {
        self.supports.get(&brick_idx).unwrap_or(&Vec::new()).clone()
    }

    pub fn get_supported_by(self: &Self, brick_idx: usize) -> Vec<usize> {
        self.supported_by.get(&brick_idx).unwrap_or(&Vec::new()).clone()
    }

    fn is_removable(self: &Self, idx: usize) -> bool {
        match self.supported_by.get(&idx) {
            Some(supported_bricks) => {
                supported_bricks.iter().all(|b| { 
                    self.supports.get(b)
                        .expect("Should have been supported by this brick?")
                        .len() > 1
                })
            }
            None => { true }
        }
    }

    pub fn num_removable_bricks(self: &Self) -> usize {
        (0..self.bricks.len())
            .filter(|i| self.is_removable(*i))
            .count()
    }

    pub fn num_falling_if_disintegrated(self: &Self, idx: usize) -> usize {
        let mut falling_bricks: HashSet<usize> = HashSet::from([idx]);

        let mut supports = HashSet::new();
        let mut next_supports = HashSet::from_iter(self.supported_by.get(&idx).cloned().unwrap_or_default().into_iter());
        
        while !next_supports.is_empty() {
            swap(&mut supports, &mut next_supports);
            for support_idx in supports.drain() {
                match self.supports.get(&support_idx) {
                    Some(supports) => {
                        if supports.iter().all(|i| falling_bricks.contains(i)) {
                            // everything is falling, so this will fall too
                            falling_bricks.insert(support_idx);

                            let next = self.supported_by.get(&support_idx).cloned().unwrap_or_default();
                            next_supports.extend(next.into_iter());
                        }
                    },
                    None => {}
                }
            }
        }

        // Minus one as we included ourselves
        falling_bricks.len() - 1
    }
}
#![allow(dead_code)]

use std::{mem::swap, cmp::{min,max}, iter, collections::{HashSet, HashMap}};
type PointType = usize;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
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

    fn overlaps_x(self: &Self, other: &Point) -> bool {
        self.x >= other.x && other.x >= self.x
    }

    fn overlaps_y(self: &Self, other: &Point) -> bool {
        self.y >= other.y && other.y >= self.y
    }

    fn overlaps(self: &Self, other: &Point) -> bool {
        self.overlaps_x(other) && self.overlaps_y(other)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Brick {
    pub min: Point,
    pub max: Point,
    pub label: char,
}
impl Brick {
    pub fn from_input(input: &str, label: char) -> Brick {
        let points : Vec<&str> = input.split('~').collect();
        let mut min = Point::from_input(&points[0]);
        let mut max = Point::from_input(&points[1]);

        if max.z < min.z || max.y < min.y || max.x < min.x {
            swap(&mut min, &mut max);
        }

        assert!(min.x==max.x || min.y==max.y); // No horizontal diagonals
        assert!(min.z==max.z || (min.x==max.x && min.y==max.y)); // No vertical diagonals

        Brick { min, max, label }
    }

    fn overlaps_x(self: &Self, other: &Brick) -> bool {
        self.max.x >= other.min.x && other.max.x >= self.min.x
    }

    fn overlaps_y(self: &Self, other: &Brick) -> bool {
        self.max.y >= other.min.y && other.max.y >= self.min.y
    }

    fn overlaps_xy(self: &Self, other: &Brick) -> bool {
        self.overlaps_x(other) && self.overlaps_y(other)
    }

    fn is_supported_by(self: &Self, other: &Brick) -> bool {
        if other.max.z != self.min.z - 1 {
            // Not directly under me
            return false;
        }

        self.overlaps_xy(other)
    }
}

#[derive(Debug)]
pub struct BrickTower {
    pub bricks: Vec<Brick>,
    pub bounds: (Point, Point),
    supports: HashMap<usize, HashSet<usize>>,
    rests_on: HashMap<usize, HashSet<usize>>,
}
impl BrickTower {
    pub fn from_input(input: &str) -> BrickTower {
        let mut bricks = Vec::new();
        let brick_chars : Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789".chars().collect();
        
        for (i, line) in input.lines().enumerate() {
            let label = brick_chars[i % brick_chars.len()];
            bricks.push(Brick::from_input(line, label));
        }

        bricks.sort_by(|a, b| a.min.z.cmp(&b.min.z));

        let mut tower = BrickTower { 
            bricks, 
            bounds: (Point::new(), Point::new()), 
            supports: HashMap::new(),
            rests_on: HashMap::new(),
         };

        tower.calc_bounds();
        tower
    }

    fn calc_bounds(self: &mut Self) {
        let mut bounds_min = Point::from_xyz(PointType::MAX, PointType::MAX, PointType::MAX);
        let mut bounds_max = Point::from_xyz(0, 0, 0);

        for brick in &self.bricks {
            bounds_min.x = min(bounds_min.x, brick.min.x);
            bounds_min.y = min(bounds_min.y, brick.min.y);
            bounds_min.z = min(bounds_min.z, brick.min.z);

            bounds_max.x = max(bounds_max.x, brick.max.x);
            bounds_max.y = max(bounds_max.y, brick.max.y);
            bounds_max.z = max(bounds_max.z, brick.max.z);
        }

        self.bounds = (bounds_min, bounds_max);
    }

    fn is_supported_by(self: &Self, brick_a: usize, brick_b: usize) -> bool {
        self.bricks[brick_a].is_supported_by(&self.bricks[brick_b])
    }

    pub fn drop_bricks(self: &mut Self) {
        for i in 0..self.bricks.len() {
            let brick = self.bricks[i].clone();

            let mut dest_z = 1;
            let mut may_rest_on = Vec::new();
            for (idx, settled_brick) in self.bricks[0..i].iter().enumerate() {
                if brick.overlaps_xy(settled_brick) {
                    may_rest_on.push(idx);
                    dest_z = dest_z.max(settled_brick.max.z + 1);
                }
            }

            for (idx, settled_brick) in may_rest_on.into_iter().map(|idx| (idx, &self.bricks[idx])) {
                if settled_brick.max.z + 1 == dest_z {
                    self.rests_on.entry(i).or_default().insert(idx);
                    self.supports.entry(idx).or_default().insert(i);
                }
            }

            let brick = &mut self.bricks[i];
            let z_diff = brick.max.z - brick.min.z;
            brick.min.z = dest_z;
            brick.max.z = dest_z + z_diff;
        }
       
        self.calc_bounds();
    }

    pub fn count_fallen_if_disintegrated(self: &Self, brick_idx: usize) -> usize {
        let mut falling = HashSet::from([brick_idx]);

        let mut check = HashSet::new();
        let mut check_next = self.supports.get(&brick_idx).cloned().unwrap_or_default();

        while !check_next.is_empty() {
            swap(&mut check, &mut check_next);
            for check in check.drain() {
                if self.rests_on[&check]
                    .iter()
                    .all(|brick| falling.contains(brick))
                {
                    falling.insert(check);
                    check_next.extend(
                        self.supports
                            .get(&check)
                            .iter()
                            .flat_map(|elem| elem.iter())
                            .copied()
                    );
                }
            }
        }

        // Subtract one as we were including ourselves
        falling.len() - 1
    }

    pub fn can_disintegrate(self: &Self, brick_idx: usize) -> bool {
        match self.supports.get(&brick_idx) {
            Some(supported) => {
                supported.iter().all(|supported| {
                    self.rests_on
                        .get(supported)
                        .expect("Supported by brick_idx so should rest on at least that brick")
                        .len() > 1                    
                })
            },
            None => { true }
        }
    }

    pub fn num_removable_bricks(self: &Self, include_losing_support: bool) -> usize {
        if include_losing_support {
            (0..self.bricks.len())
                .map(|x| self.count_fallen_if_disintegrated(x))
                .sum()
        } else {
            (0..self.bricks.len())
                .filter(|x| self.can_disintegrate(*x)).count()
        }
    }

    pub fn get_bricks_by_z(self: &Self, z: usize) -> Vec<usize> {
        (0..self.bricks.len())
            .filter(|x| self.bricks[*x].min.z <= z && self.bricks[*x].max.z >= z)
            .collect()
    }
}
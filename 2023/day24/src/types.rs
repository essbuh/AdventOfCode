#[derive(Debug, Copy, Clone)]
pub struct Vector {
    pub x: i64,
    pub y: i64,
    pub z: i64
}
impl Vector {
    fn from_input(input: &str) -> Self {
        let parts : Vec<&str> = input.split(",").collect();
        Vector {
            x: parts[0].trim().parse::<i64>().unwrap(),
            y: parts[1].trim().parse::<i64>().unwrap(),
            z: parts[2].trim().parse::<i64>().unwrap()
        }
    }
    
    fn equals(&self, other: &Self, ignore_z: bool) -> bool {
        self.x == other.x
            && self.y == other.y
            && (ignore_z || self.z == other.z)
    }
}

fn is_in_bounds(pos: &(f64, f64, f64), bounds: (i64, i64, i64)) -> bool {
    pos.0 as i64 >= bounds.0 && pos.0 as i64 <= bounds.1
        && pos.1 as i64 >= bounds.0 && pos.1 as i64 <= bounds.1
        && (bounds.2 == i64::MAX || (pos.2 as i64 >= bounds.0 && pos.2 as i64 <= bounds.1))
}

#[derive(Debug, Copy, Clone)]
pub struct Hailstone {
    pub pos: Vector,
    pub vel: Vector
}
impl Hailstone {
    fn from_input(input: &str) -> Self {
        let parts : Vec<&str> = input.split(" @ ").collect();
        Hailstone {
            pos: Vector::from_input(parts[0]),
            vel: Vector::from_input(parts[1])
        }
    }

    fn get_pos_at_time(&self, t: f64) -> (f64, f64, f64) {
        (
            self.pos.x as f64 + self.vel.x as f64 * t,
            self.pos.y as f64 + self.vel.y as f64 * t,
            self.pos.z as f64 + self.vel.z as f64 * t
        )
    }

    fn get_intersect_times(&self, other: &Self, ignore_z: bool) -> (f64, f64) {
        if self.pos.equals(&other.pos, ignore_z) {
            return (0.0, 0.0);
        }

        let dx = other.pos.x - self.pos.x;
        let dy = other.pos.y - self.pos.y;
        let det = other.vel.x * self.vel.y - other.vel.y * self.vel.x;
        if det != 0 { 
            let u = (dy * other.vel.x - dx * other.vel.y) as f64 / det as f64;
            let v = (dy * self.vel.x - dx * self.vel.y) as f64 / det as f64;
            return (u, v);
        }
        
        return (f64::MAX, f64::MAX);
    }
}

#[derive(Debug)]
pub struct Hailstorm {
    hailstones: Vec<Hailstone>,
}
impl Hailstorm {
    pub fn from_input(input: &str) -> Self {
        let mut hailstones = Vec::new();
        for line in input.lines() {
            hailstones.push(Hailstone::from_input(line));
        }
        Hailstorm { hailstones }
    }

    pub fn count_intersections(&self, bounds: (i64, i64, i64), debug: bool) -> usize {
        let mut num_intersects = 0;
        let is_2d = bounds.2 == i64::MAX;

        for i in 0..self.hailstones.len() {
            for j in i+1..self.hailstones.len() {
                let hailstone_a = &self.hailstones[i];
                let hailstone_b = &self.hailstones[j];
                if debug {
                    println!("Hailstone A: {:.0}, {:.0}, {:.0} @ {:.0}, {:.0}, {:.0}",
                        hailstone_a.pos.x, hailstone_a.pos.y, hailstone_a.pos.z,
                        hailstone_a.vel.x, hailstone_a.vel.y, hailstone_a.vel.z);
                    println!("Hailstone B: {:.0}, {:.0}, {:.0} @ {:.0}, {:.0}, {:.0}",
                        hailstone_b.pos.x, hailstone_b.pos.y, hailstone_b.pos.z,
                        hailstone_b.vel.x, hailstone_b.vel.y, hailstone_b.vel.z);
                }

                let intersect_times = hailstone_a.get_intersect_times(&hailstone_b, is_2d);
                if intersect_times.0 >= 0.0 && intersect_times.1 >= 0.0 {
                    let intersect_pos = hailstone_a.get_pos_at_time(intersect_times.0);
                    if intersect_times.0 == f64::MAX {
                        if debug { println!("Hailstones' paths are parallel; they never intersect."); }
                    }
                    else if is_in_bounds(&intersect_pos, bounds) {
                        num_intersects += 1;
                        if debug { 
                            println!("Hailstones' paths will cross *inside* the test area (at x={:.3}, y={:.3}{})",
                                intersect_pos.0, intersect_pos.1,
                                if is_2d { "".to_string() } else { format!(", z={:.2}", intersect_pos.2) });
                        }
                    } else if debug {
                        println!("Hailstones' paths will cross outside the test area (at x={:.3}, y={:.3}{})",
                            intersect_pos.0, intersect_pos.1,
                            if is_2d { "".to_string() } else { format!(", z={:.2}", intersect_pos.2) });
                    }
                } else if debug {
                    if intersect_times.0 < 0.0 && intersect_times.1 < 0.0 {
                        println!("Hailstones' paths crossed in the past for both hailstones.");
                    } else {
                        println!("Hailstones' paths crossed in the past for hailstone {}.",
                            if intersect_times.0 < 0.0 { "A" } else { "B" })
                    }
                }
            }
        }

        num_intersects
    }

    pub fn all_intersects(&self, velmod: (i64, i64)) -> Option<(f64, i64)> {
        let mut result = None;
        let mut last_z = i64::MAX;

        let mut hailstone_a = self.hailstones[0].clone();
        hailstone_a.vel.x -= velmod.0;
        hailstone_a.vel.y -= velmod.1;

        for i in 1..self.hailstones.len() {
            let mut all_intersects = true;
            
            let mut hailstone_b = self.hailstones[i].clone();
            hailstone_b.vel.x -= velmod.0;
            hailstone_b.vel.y -= velmod.1;

            let intersect_times = hailstone_a.get_intersect_times(&hailstone_b, true);
            if intersect_times.0 == f64::MAX || (intersect_times.0 < 0.0 && intersect_times.1 < 0.0) {
                all_intersects = false;
            } else {
                let intersect_z = hailstone_a.get_pos_at_time(intersect_times.0).2 as i64;
                if last_z != i64::MAX && intersect_z != last_z {
                    all_intersects = false;
                } else {
                    last_z = intersect_z;
                    result = Some((&self.hailstones[i]
                        , intersect_times.0
                        , intersect_times.1));
                }
            }

            if !all_intersects {
                result = None;
                break;
            }
        }

        match result {
            Some(r) => { 
                let hailstone_a = &self.hailstones[0];
                let hailstone_b = r.0;
                
                let z_a = hailstone_a.get_pos_at_time(r.1).2 as i64;
                let z_b = hailstone_b.get_pos_at_time(r.2).2 as i64;

                let rock_velo = ((z_b - z_a) as f64 / (r.2 - r.1)) as i64;
                Some((
                    r.1, 
                    rock_velo
                )) 
            }
            None => { None }
        }
    }

    pub fn find_common_rock(&self) -> Hailstone {
        let mut size = 1;
        let mut rock = Hailstone {
            pos: Vector { x: 0, y: 0, z: 0 },
            vel: Vector { x: 0, y: 0, z: 0 }
        };
        let mut last_result : Option<(f64, (i64, i64, i64))> = None;

        loop {
            let mut found_result = false;
            for x in -size..=size {
                for y in -size..=size {
                    if x == 0 && y == 0 {
                        continue;
                    }

                    if let Some(result) = self.all_intersects((x, y)) {
                        last_result = Some((result.0, (x,y, result.1)));
                        found_result = true;
                        break;
                    }
                }

                if found_result {
                    break;
                }
            }

            if found_result {
                break;
            } else {
                size += 1;
            }
        }

        assert!(last_result.is_some());
        let (t, rock_velo) = last_result.unwrap();
        
        let intersect_pos = self.hailstones[0].get_pos_at_time(t);
        
        rock.pos = Vector {
            x: (intersect_pos.0 - (rock_velo.0 as f64 * t)) as i64,
            y: (intersect_pos.1 - (rock_velo.1 as f64 * t)) as i64,
            z: (intersect_pos.2 - (rock_velo.2 as f64 * t)) as i64,
        };

        rock.vel = Vector {
            x: rock_velo.0,
            y: rock_velo.1,
            z: rock_velo.2,
        };

        rock
    }
}
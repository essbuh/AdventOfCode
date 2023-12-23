#![allow(dead_code)]

use crate::types::*;

impl Maze {
    fn hydrate_path(&self, path: &Path) -> Vec<Point> {
        let mut result = Vec::new();
        let points = &path.0;

        for i in 0..points.len()-1 {
            let curr_point = &points[i];
            let next_point = &points[i+1];
            
            let connection = self.direct_connections.get(curr_point).unwrap()
                .iter()
                .filter(|&c| {
                    &c.0 == next_point
                    //&& c.1 == next_point.1
                })
                .max_by(|a, b| a.1.cmp(&b.1))
                .expect("bad path");

            result.pop(); // Segments have start & end
            result.extend(&connection.2);
        }

        if result.len() != path.2 + 1 { println!("Missing some points"); }
        result
    }

    pub fn print(&self) {
        for row in &self.chars {
            println!("{}", row.iter().collect::<String>());
        }
    }

    pub fn print_path(&self, path: &Path) {
        let real_path = self.hydrate_path(path);

        for (j, row) in self.chars.iter().enumerate() {
            let mut s = String::new();
            for (i, char) in row.iter().enumerate() {
                let point = Point::from_xy(i as i32, j as i32);
                if &point == &self.entry {
                    s += "S";
                } else {
                    let path_steps = real_path.iter().filter(|&p| p == &point).count();
                    if path_steps > 0 {
                        s += &(path_steps - 1).to_string();
                    } else {
                        s += &char.to_string();
                    }
                }
            }

            println!("{}", s);
        }
    }
}
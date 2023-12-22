use crate::types::BrickTower;

use std::{iter, cmp::max};

fn print_vert_label(tower: &BrickTower, is_xz: bool) -> (String, String) {
    let max_horiz = if is_xz { tower.bounds.1.x } else { tower.bounds.1.y };
    let mut label = String::new();

    let mut divisor = 1;
    while divisor <= max_horiz {
        let mut s = String::new();
        for i in 0..=max_horiz {
            let val = i / divisor;
            if val == 0 && divisor > 1 { 
                s += " "; 
            } else {
                s += &val.to_string();
            }
        }

        label = if label.is_empty() { s } else { s + "\n" + &label };
        divisor = divisor * 10;
    }
    
    let half_point = max_horiz / 2;
    (iter::repeat(" ").take(half_point).collect::<String>() 
        + if is_xz { "x" } else { "y" },
        label)
}

fn print_vert_labels(tower: &BrickTower) {
    let label_x = print_vert_label(tower, true);
    let label_y = print_vert_label(tower, false);
    
    let max_dim = max(tower.bounds.1.x, tower.bounds.1.y);
    let max_x_chars = max_dim + 1
        + 1 // Padding
        + tower.bounds.1.z.to_string().len()
        + 1; // Padding

    println!("{}{}{}{}{}{}{}",
        label_x.0,
        iter::repeat(" ").take(max_x_chars - label_x.0.len()).collect::<String>(),
        label_y.0,
        iter::repeat(" ").take(max_x_chars - label_y.0.len()).collect::<String>(),
        label_x.0,
        iter::repeat(" ").take(max_x_chars - label_x.0.len()).collect::<String>(),
        label_y.0);

    println!("{}{}{}{}{}{}{}",
        label_x.1,
        iter::repeat(" ").take(max_x_chars - label_x.1.len()).collect::<String>(),
        label_y.1,
        iter::repeat(" ").take(max_x_chars - label_y.1.len()).collect::<String>(),
        label_x.1,
        iter::repeat(" ").take(max_x_chars - label_x.1.len()).collect::<String>(),
        label_y.1);
}

pub fn print_tower(tower: &BrickTower) {
    print_vert_labels(tower);

    let max_dim = max(tower.bounds.1.x, tower.bounds.1.y);
    let max_z_str = tower.bounds.1.z.to_string();
    
    let mid_z = (tower.bounds.1.z + 1) / 2;
    for z in (1..=tower.bounds.1.z).rev() {
        let z_as_str = z.to_string();
        let mut bricks = tower.get_bricks_by_z(z);

        // +X -Y
        let mut s = String::new();
        for x in 0..=max_dim {
            let mut label : Option<char> = None;
            bricks.sort_by(|a, b| tower.bricks[*b].min.y.cmp(&tower.bricks[*a].min.y));

            for brick_idx in &bricks {
                let brick = &tower.bricks[*brick_idx];
                if brick.min.x <= x && brick.max.x >= x {
                    label = Some(brick.label);
                    break;
                }
            }
            match label {
                Some(v) => { s += &v.to_string(); },
                None => { s += "."; }
            }
        }

        let mut z_str = format!("{} {}{} ", 
            s, 
            &z_as_str, 
            &iter::repeat(" ").take(max_z_str.len() - z_as_str.len()).collect::<String>());
        
        // -Y -X
        let mut s = String::new();
        for y in (0..=max_dim).rev() {
            let mut label : Option<char> = None;
            bricks.sort_by(|a, b| tower.bricks[*b].max.x.cmp(&tower.bricks[*a].max.x));

            for brick_idx in &bricks {
                let brick = &tower.bricks[*brick_idx];
                if brick.min.y <= y && brick.max.y >= y {
                    label = Some(brick.label);
                    break;
                }
            }
            match label {
                Some(v) => { s += &v.to_string(); },
                None => { s += "."; }
            }
        }

        z_str = format!("{}{} {}{} ", 
            z_str, s, 
            &z_as_str, &iter::repeat(" ").take(max_z_str.len() - z_as_str.len()).collect::<String>());
        
        // -X +Y
        let mut s = String::new();
        for x in (0..=max_dim).rev() {
            let mut label : Option<char> = None;
            bricks.sort_by(|a, b| tower.bricks[*a].max.y.cmp(&tower.bricks[*b].max.y));

            for brick_idx in &bricks {
                let brick = &tower.bricks[*brick_idx];
                if brick.min.x <= x && brick.max.x >= x {
                    label = Some(brick.label);
                    break;
                }
            }
            match label {
                Some(v) => { s += &v.to_string(); },
                None => { s += "."; }
            }
        }

        z_str = format!("{}{} {}{} ", 
            z_str, s, 
            &z_as_str, &iter::repeat(" ").take(max_z_str.len() - z_as_str.len()).collect::<String>());
        
        // +Y +X
        let mut s = String::new();
        for y in 0..=max_dim {
            let mut label : Option<char> = None;
            bricks.sort_by(|a, b| tower.bricks[*a].min.x.cmp(&tower.bricks[*b].min.x));

            for brick_idx in &bricks {
                let brick = &tower.bricks[*brick_idx];
                if brick.min.y <= y && brick.max.y >= y {
                    label = Some(brick.label);
                    break;
                }
            }
            match label {
                Some(v) => { s += &v.to_string(); },
                None => { s += "."; }
            }
        }

        z_str = format!("{}{} {}{} ", 
            z_str, s, 
            &z_as_str, &iter::repeat(" ").take(max_z_str.len() - z_as_str.len()).collect::<String>());

        println!("{z_str} {}", if z == mid_z { "z" } else { "" });
    }

    println!("{} 0{} {} 0{} {} 0{} {} 0", 
        iter::repeat("-").take(max_dim + 1).collect::<String>(),
        iter::repeat(" ").take(max_z_str.len() - 1).collect::<String>(),
        iter::repeat("-").take(max_dim + 1).collect::<String>(),
        iter::repeat(" ").take(max_z_str.len() - 1).collect::<String>(),
        iter::repeat("-").take(max_dim + 1).collect::<String>(),
        iter::repeat(" ").take(max_z_str.len() - 1).collect::<String>(),
        iter::repeat("-").take(max_dim + 1).collect::<String>());
}
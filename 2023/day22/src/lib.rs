mod types;
mod debug;

use types::BrickTower;
use debug::print_tower;

pub fn part_1(input: &str, debug: bool) -> usize {
    let mut tower = BrickTower::from_input(input);
    if debug {
        println!("Starting Layout:\n");
        print_tower(&tower);
    }

    tower.drop_bricks();

    if debug {
        println!("After Dropping:\n");
        print_tower(&tower);
    
        for (idx, brick) in tower.bricks.iter().enumerate() {
            println!("Brick {},{},{}~{},{},{} supported by {}, supports {}",
                brick.left.x,brick.left.y,brick.left.z,
                brick.right.x,brick.right.y,brick.right.z,
                tower.get_supports(idx)
                    .iter().map(|x| {
                        let b = &tower.bricks[*x];
                        format!("{},{},{}~{},{},{}", 
                            b.left.x,b.left.y,b.left.z,
                            b.right.x,b.right.y,b.right.z)
                    }).collect::<Vec<String>>()
                    .join("|"),
                tower.get_supported_by(idx)
                    .iter().map(|x| {
                        let b = &tower.bricks[*x];
                        format!("{},{},{}~{},{},{}", 
                            b.left.x,b.left.y,b.left.z,
                            b.right.x,b.right.y,b.right.z)
                    }).collect::<Vec<String>>()
                    .join("|")
            );
        }
    }

    tower.num_removable_bricks()
}

pub fn part_2(input: &str, debug: bool) -> usize {
    let mut tower = BrickTower::from_input(input);
    if debug {
        println!("Starting Layout:\n");
        print_tower(&tower);
    }

    tower.drop_bricks();

    if debug {
        println!("After Dropping:\n");
        print_tower(&tower);
    
        for (idx, brick) in tower.bricks.iter().enumerate() {
            println!("Brick {},{},{}~{},{},{} supported by {}, supports {}",
                brick.left.x,brick.left.y,brick.left.z,
                brick.right.x,brick.right.y,brick.right.z,
                tower.get_supports(idx)
                    .iter().map(|x| {
                        let b = &tower.bricks[*x];
                        format!("{},{},{}~{},{},{}", 
                            b.left.x,b.left.y,b.left.z,
                            b.right.x,b.right.y,b.right.z)
                    }).collect::<Vec<String>>()
                    .join("|"),
                tower.get_supported_by(idx)
                    .iter().map(|x| {
                        let b = &tower.bricks[*x];
                        format!("{},{},{}~{},{},{}", 
                            b.left.x,b.left.y,b.left.z,
                            b.right.x,b.right.y,b.right.z)
                    }).collect::<Vec<String>>()
                    .join("|")
            );
        }
    }

    (0..tower.bricks.len())
        .map(|i| tower.num_falling_if_disintegrated(i))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_sample() {
        let result = part_1(include_str!("sample.txt"), false);
        println!("Part 1 (Sample): {result}");
        assert_eq!(result, 5);
    }

    #[test]
    fn part_1_sample_2() {
        let result = part_1(include_str!("sample_2.txt"), false);
        println!("Part 1 (Sample 2): {result}");
        assert_eq!(result, 10);
    }

    #[test]
    fn part_1_input() {
        let result = part_1(include_str!("input.txt"), false);
        println!("Part 1 (Real): {result}");
        assert_eq!(result, 418);
    }

    #[test]
    fn part_2_sample_1() {
        let result = part_2(include_str!("sample.txt"), false);
        println!("Part 2 (Sample): {result}");
        assert_eq!(result, 7);
    }

    #[test]
    fn part_2_input() {
        let result = part_2(include_str!("input.txt"), false);
        println!("Part 2 (Real): {result}");
        assert_eq!(result, 70702);
    }
}
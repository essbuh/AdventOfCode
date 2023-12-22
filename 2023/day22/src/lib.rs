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
    }

    tower.num_removable_bricks(false)
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
    }

    tower.num_removable_bricks(true)
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
    fn part_1_sample_3() {
        let result = part_1(include_str!("sample_3.txt"), true);
        println!("Part 1 (Sample 3): {result}");
        assert_eq!(result, 2);
    }

    #[test]
    fn part_1_sample_4() {
        let result = part_1(include_str!("sample_4.txt"), true);
        println!("Part 1 (Sample 4): {result}");
        assert_eq!(result, 1);
    }

    #[test]
    fn part_1_input() {
        let result = part_1(include_str!("input.txt"), true);
        println!("Part 1 (Real): {result}");
        assert_eq!(result, 525);
    }
    
    #[test]
    fn part_1_input2() {
        let result = part_1(include_str!("input_2.txt"), true);
        println!("Part 1 (Real 2): {result}");
        assert_eq!(result, 471);
    }

    #[test]
    fn part_2_sample() {
        let result = part_2(include_str!("sample.txt"), true);
        println!("Part 2 (Sample): {result}");
        assert_eq!(result, 7);
    }

    #[test]
    fn part_2_input() {
        let result = part_2(include_str!("input.txt"), true);
        println!("Part 2 (Real): {result}");
        assert_eq!(result, 70702);
    }
    
    #[test]
    fn part_2_input2() {
        let result = part_2(include_str!("input_2.txt"), true);
        println!("Part 2 (Real 2): {result}");
        assert_eq!(result, 68525);
    }
}
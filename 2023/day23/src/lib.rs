mod types;
mod debug;

fn get_longest_path_len(input: &str, ignore_slopes: bool, debug: bool) -> usize {
    let maze = types::Maze::from_input(&input, ignore_slopes);
    
    //if debug {
        //maze.print();
    //}

    let longest_path = maze.get_longest_path(debug);

    if debug {
        maze.print_path(&longest_path);
    }

    longest_path.2
}

pub fn part_1(input: &str, debug: bool) -> usize {
    get_longest_path_len(input, false, debug)
}

pub fn part_2(input: &str, debug: bool) -> usize {
    get_longest_path_len(input, true, debug)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample() {
        let result = part_1(include_str!("sample.txt"), false);
        assert_eq!(result, 94);
    }

    #[test]
    fn part1_sample2() {
        let result = part_1(include_str!("sample_2.txt"), false);
        assert_eq!(result, 14);
    }

    #[test]
    fn part1_sample3() {
        let result = part_1(include_str!("sample_3.txt"), false);
        assert_eq!(result, 3);
    }

    #[test]
    fn part1_sample4() {
        let result = part_1(include_str!("sample_4.txt"), false);
        assert_eq!(result, 52);
    }

    #[test]
    fn part1_input() {
        let result = part_1(include_str!("input.txt"), false);
        assert_eq!(result, 2250);
    }

    #[test]
    fn part2_sample() {
        let result = part_2(include_str!("sample.txt"), false);
        assert_eq!(result, 154);
    }

    #[test]
    fn part2_input() {
        let result = part_2(include_str!("input.txt"), false);
        assert_eq!(result, 6470);
    }
}

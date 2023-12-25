fn get_elf_capacities(input: &str) -> Vec<usize> {
    let mut result = Vec::new();

    let mut sum = 0;
    for line in input.lines() {
        if line.is_empty() {
            result.push(sum);
            sum = 0;
        } else {
            sum += line.parse::<usize>().unwrap();
        }
    }

    result.push(sum);

    result
}

fn part_1(input: &str) -> usize {
    get_elf_capacities(input).into_iter().max().unwrap()
}

fn part_2(input: &str) -> usize {
    let mut capacities = get_elf_capacities(input);
    capacities.sort_by(|a, b| b.cmp(&a));
    
    capacities.into_iter()
        .take(3)
        .sum()
}

fn main() {
    let result = part_1(include_str!("input.txt"));
    println!("Part 1: {result}");

    let result = part_2(include_str!("input.txt"));
    println!("Part 2: {result}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample() {
        let result = part_1(include_str!("sample.txt"));
        assert_eq!(result, 24000);
    }

    #[test]
    fn part1_input() {
        let result = part_1(include_str!("input.txt"));
        assert_eq!(result, 66616);
    }

    #[test]
    fn part2_sample() {
        let result = part_2(include_str!("sample.txt"));
        assert_eq!(result, 45000);
    }
    
    #[test]
    fn part2_input() {
        let result = part_2(include_str!("input.txt"));
        assert_eq!(result, 199172);
    }
}
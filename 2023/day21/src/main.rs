mod day21;

fn part_1() {
    let input = include_str!("input.txt");
    let result = day21::get_result_part1(input, 64, false);
    println!("Part 1: {result}");
}

fn part_2() {
    let input = include_str!("input.txt");
    let result = day21::solve_part2(input, 26501365, false);
    println!("Part 2: {result}");
}

fn main() {
    part_1();
    part_2();
}

mod day20;

fn part_1() {
    let input = include_str!("input.txt");
    let result = day20::get_result_part1(input, 1000, false);    
    println!("Part 1: {result}");
}

fn part_2() {
    let input = include_str!("input.txt");
    let result = day20::get_result_part2(input, false);
    println!("Part 2: {result}");
}

fn main() {
    part_1();
    part_2();
}

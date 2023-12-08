use std::collections::HashMap;
use num::integer::lcm;

#[derive(Debug)]
struct NodeMap<'a> {
    directions: Vec<char>,
    nodes: HashMap<&'a str, (&'a str, &'a str)>,
}

fn parse_node_map(input: &str) -> NodeMap {
    let mut lines = input.lines();

    let mut node_map = NodeMap { directions: Vec::new(), nodes: HashMap::new() };
    node_map.directions = lines.next().unwrap().trim().chars().collect();
    
    // skip empty line
    lines.next();

    node_map.nodes = lines
        .map(|x| {
            let mut parts = x.split(" = (");
            let key = parts.next().unwrap();
            let mut vals = parts.next().unwrap().split(", ");
            let left = vals.next().unwrap();
            let right = vals.next().unwrap().trim_end_matches(")");

            (key, (left, right))
        })
        .collect();

    node_map
}

fn count_steps(node_map: &NodeMap, start: &str, dest: &str, dest_is_ending: bool) -> i64 {
    let mut step_count = 0;

    let mut current_node = start;
    let mut next_dir_index = 0;

    loop {
        let hit_ending = match dest_is_ending {
            true => current_node.ends_with(dest),
            false => current_node == dest
        };
        if hit_ending {
            break;
        }

        let direction = node_map.directions[next_dir_index];
        let node = node_map.nodes[current_node];
        current_node = if direction == 'L' { node.0 } else { node.1 };

        next_dir_index = (next_dir_index + 1) % node_map.directions.len();
        step_count = step_count + 1;
    }

    step_count
}

fn get_lcm(nums: &[i64]) -> i64 {
    assert!(!nums.is_empty());
    if nums.len() == 1 {
        return nums[0];
    }
    if nums.len() == 2 {
        return lcm(nums[0], nums[1]);
    }

    let a = nums[0];
    let b = get_lcm(&nums[1..]);
    return lcm(a, b);
}

fn count_steps_ending(node_map: &NodeMap, start: &str, dest: &str) -> i64 {
    let node_steps : Vec<i64> = node_map.nodes
        .keys().into_iter()
        .filter(|k| k.ends_with(start))
        .map(|n| count_steps(node_map, n, dest, true))
        .collect();

    get_lcm(&node_steps[0..])
}

fn main() {
    let input = include_str!("input.txt");
    let node_map = parse_node_map(input);
    let steps = count_steps(&node_map, "AAA", "ZZZ", false);
    println!("Num Steps (Simple): {steps}");
    
    let steps = count_steps_ending(&node_map, "A", "Z");
    println!("Num Steps (Ghost): {steps}");
}

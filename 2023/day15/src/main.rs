use std::collections::HashMap;

type MirrorMap<'a> = HashMap<i32, Vec<(&'a str, i32)>>;
const OP_ASSIGN : char = '=';
const OP_REMOVE : char = '-';

fn get_hash(input: &str) -> i32 {
    input.chars().into_iter()
        .map(|c| (c as i32))
        .fold(0, |acc, val| {
            ((acc + val) * 17) % 256
        })
}

fn get_sequence_hash(input: &str) -> i32 {
    input.split(',').into_iter()
        .map(|x| get_hash(&x))
        .sum()
}

fn part_1() {
    let input = include_str!("input.txt");
    let hash_value = get_sequence_hash(&input);

    println!("Part 1: {hash_value}");
}

fn get_part2_solution(input: &str) -> MirrorMap {
    let mut mirror_map : MirrorMap = MirrorMap::new();

    for step in input.split(',').into_iter() {
        let mut step_chars = step.chars();
        let instruction_idx = step_chars.position(|x| x == OP_ASSIGN || x == OP_REMOVE).unwrap();
        let label = &step[0..instruction_idx];
        let hash = get_hash(&label);
        let mirror_box = mirror_map.entry(hash).or_insert(Vec::new());

        let instruction = step.chars().nth(instruction_idx).unwrap();
        let existing_index = mirror_box.iter().position(|x| x.0 == label);

        match instruction {
            OP_ASSIGN => {
                // .nth() is consuming, so all that's left is value
                let value = step_chars.collect::<String>().parse::<i32>().unwrap();

                match existing_index {
                    Some(index) => { 
                        //println!("[Box {hash}] Replacing label {label} index {index} with value {value}");
                        mirror_box[index] = (label, value); 
                    },
                    None => { 
                        //println!("[Box {hash}] Adding label {label} value {value}");
                        mirror_box.push((label, value)); 
                    }
                }
            },
            OP_REMOVE => {
                match existing_index {
                    Some(index) => {                        
                        //println!("[Box {hash}] Removing label {label} from index {index}");
                        mirror_box.remove(index);
                    },
                    None => {}
                }
            },
            _ => {},
        }

        //println!("{:?}", mirror_map);
    }

    mirror_map
}

fn get_mirror_map_power(mirror_map: &MirrorMap) -> i32 {
    mirror_map.iter()
        .map(|(k, v)| {
            v.iter().enumerate()
                .map(|(i, entry)| (*k + 1) * (i as i32 + 1) * entry.1)
        }).flatten().sum()
}

fn part_2() {
    let input = include_str!("input.txt");
    let mirror_map = get_part2_solution(&input);
    let power = get_mirror_map_power(&mirror_map);
    println!("Part 2: {power}");
}

fn main() {
    part_1();
    part_2();
}

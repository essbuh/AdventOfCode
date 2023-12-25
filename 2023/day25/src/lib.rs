#![allow(dead_code)]

mod types;

use types::Diagram;

fn part_1(input: &str, debug: bool) -> usize {
    let mut diagram = Diagram::from_input(input);
    if debug { println!("{diagram:#?}"); }
    let wires_to_cut = diagram.get_standalone_wires(debug);
    diagram.remove_connections(&wires_to_cut);
    let result = diagram.get_group_sizes(debug);
    assert_eq!(result.len(), 2);
    result[0] * result[1]
}

fn part_1_direct(input: &str, wires_to_cut: &Vec<(&str, &str)>, debug: bool) -> usize {
    let mut diagram = Diagram::from_input(input);
    if debug { println!("{diagram:#?}"); }

    diagram.remove_connections(&wires_to_cut);

    let result = diagram.get_group_sizes(debug);
    assert_eq!(result.len(), 2);    
    result[0] * result[1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample() {
        let result = part_1(include_str!("sample.txt"), false);
        assert_eq!(result, 54);
    }

    #[test]
    fn part1_input() {
        let result = part_1_direct(include_str!("input.txt"),
            &vec![ ("htb","bbg"), ("pcc", "htj"), ("pjj", "dlk") ],
            false);
        assert_eq!(result, 538560);
    }
}

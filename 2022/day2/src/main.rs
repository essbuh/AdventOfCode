fn get_winning_play(other: char) -> char {
    match other {
        'A' => { 'Y' },
        'B' => { 'Z' },
         _  => { 'X' },
    }
}

fn get_losing_play(other: char) -> char {
    match other {
        'A' => { 'Z' },
        'B' => { 'X' },
         _  => { 'Y' },
    }
}

fn get_draw_play(other: char) -> char {
    char::from_u32(other as u32 + ('X' as u32 - 'A' as u32)).unwrap()
}

fn get_hand_score(hand: char) -> usize {
    match hand {
        'X' => { 1 },
        'Y' => { 2 },
         _  => { 3 }
    }
}

fn is_b_winner(a: char, b: char) -> bool {
    b == get_winning_play(a)
}

fn is_draw(a: char, b: char) -> bool {
    (b as i32 - a as i32) == ('X' as i32 - 'A' as i32)
}

fn get_score(a: char, b: char) -> usize {
    let round_score = 
        if is_draw(a, b) { 3 } 
        else if is_b_winner(a, b) { 6 } 
        else { 0 };

    let hand_score = get_hand_score(b);

    round_score + hand_score
}

fn get_score_pt2(a: char, b: char) -> usize {
    let hand = match b {
        'X' => { get_losing_play(a) }, // lose
        'Y' => { get_draw_play(a) }, // draw
         _  => { get_winning_play(a) }  // win
    };

    get_score(a, hand)
}

fn get_rounds(input: &str) -> Vec<(char, char)> {
    input.lines()
        .map(|l| l.split_once(" ").unwrap())
        .map(|l| (l.0.chars().next().unwrap(), l.1.chars().next().unwrap()))
        .collect()
}

fn part_1(input: &str) -> usize {
    get_rounds(input).into_iter()
        .map(|r| get_score(r.0, r.1))
        .sum()
}

fn part_2(input: &str) -> usize {
    get_rounds(input).into_iter()
        .map(|r| get_score_pt2(r.0, r.1))
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
        assert_eq!(result, 15);
    }

    #[test]
    fn part1_input() {
        let result = part_1(include_str!("input.txt"));
        assert_eq!(result, 10624);
    }

    #[test]
    fn part2_sample() {
        let result = part_2(include_str!("sample.txt"));
        assert_eq!(result, 12);
    }
    
    #[test]
    fn part2_input() {
        let result = part_2(include_str!("input.txt"));
        assert_eq!(result, 14060);
    }
}
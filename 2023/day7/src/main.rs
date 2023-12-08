use std::collections::HashMap;
use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfKind,
    FullHouse,
    FourOfKind,
    FiveOfKind,
}
impl HandType {
    fn from_cards(cards: &str, j_is_wildcard: bool) -> HandType {
        let mut card_counts : HashMap<char, i32> = HashMap::new();
        for char in cards.chars() {
            let count = card_counts.entry(char).or_insert(0);
            *count = *count + 1;
        }

        if j_is_wildcard {
            // Replace Js with the most-common card
            let highest_non_j = card_counts.iter()
                .filter(|(k, _)| *k != &'J')
                .max_by(|a, b| a.1.cmp(b.1))
                .map(|(k, _)| k);

            match highest_non_j {
                Some(card) => {
                    let j_count = *card_counts.get(&'J').unwrap_or(&0);
                    card_counts.entry(*card).and_modify(|v| *v += j_count);

                    card_counts.remove(&'J');
                },
                None => {}
            }
        }

        let highest_card_count = card_counts.iter()
            .map(|(_, value)| value)
            .max().unwrap();

        match card_counts.len() {
            1 => { assert_eq!(highest_card_count, &5); HandType::FiveOfKind },
            2 => if highest_card_count == &4 { HandType::FourOfKind } else { HandType::FullHouse }
            3 => if highest_card_count == &3 { HandType::ThreeOfKind } else { HandType::TwoPair }
            4 => HandType::OnePair,
            _ => HandType::HighCard
        }
    }
}

#[derive(Debug)]
struct Hand {
    bid: i32,
    card_strengths: Vec<i32>,
    hand_type: HandType,
}
impl Hand {
    fn new(line: &str, j_is_wildcard: bool) -> Hand {
        let line_parts : Vec<&str> = line.split(' ').collect();
        let cards = line_parts[0];
        Hand {
            bid: line_parts[1].parse::<i32>().unwrap(),
            card_strengths: get_card_strengths(cards, j_is_wildcard),
            hand_type: HandType::from_cards(cards, j_is_wildcard),
        }
    }
}

fn get_card_strengths(cards: &str, j_is_wildcard: bool) -> Vec<i32> {
    const CARD_STRENGTHS : &'static [char] = &['2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A'];
    const CARD_STRENGTHS_J_WC : &'static [char] = &['J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A'];

    let strengths = if j_is_wildcard { CARD_STRENGTHS_J_WC } else { CARD_STRENGTHS };

    cards.chars()
        .map(|card| strengths.iter().position(|x| x == &card).unwrap() as i32)
        .collect()
}

// Returns true if 'a' should be sorted before 'b'
fn compare_hands(a: &Hand, b: &Hand) -> Ordering {    
    match a.hand_type.cmp(&b.hand_type) {
        Ordering::Greater => Ordering::Greater,
        Ordering::Less => Ordering::Less,
        Ordering::Equal => {
            let first_difference = a.card_strengths.iter().enumerate()
                .map(|(i, strength)| strength.cmp(&b.card_strengths[i]))
                .find(|x| x != &Ordering::Equal);

            match first_difference {
                Some(x) => x,
                None => Ordering::Equal
            }
        }
    }
}

fn parse_hands(input: &str, j_is_wildcard: bool) -> Vec<Hand> {
    input.lines()
        .map(|line| Hand::new(line, j_is_wildcard))
        .collect()
}

fn find_winnings(input: &str, j_is_wildcard: bool) {
    let mut hands = parse_hands(input, j_is_wildcard);
    hands.sort_by(compare_hands);

    let sum: i32 = hands.iter().enumerate()
        .map(|(i, hand)| hand.bid * (i as i32 + 1))
        .sum();
    println!("Total Winnings (J_WC={j_is_wildcard}): {sum}");
}

fn main() {
    let input = include_str!("input.txt");    
    find_winnings(input, false);
    find_winnings(input, true);
}

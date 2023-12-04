use std::collections::{HashSet, HashMap};

struct LotteryTicket
{
    card_num: i32,
    winning_num: HashSet<i32>,
    my_num: HashSet<i32>
}
impl LotteryTicket {
    fn new() -> LotteryTicket {
        LotteryTicket { card_num: 0, winning_num: HashSet::new(), my_num: HashSet::new() }
    }

    fn get_score(self: &Self) -> i32 {
        self.my_num.iter()
            .filter(|x| self.winning_num.contains(x))
            .fold(0, |acc, _val| if acc == 0 { 1 } else { acc * 2 })
    }

    fn num_matches(self: &Self) -> usize {
        self.my_num.iter()
            .filter(|x| self.winning_num.contains(x))
            .count()
    }
}

fn parse_numbers(line: &str) -> HashSet<i32> {
    line.trim().split(' ')
        .filter(|x| !x.is_empty())
        .map(|x| x.parse::<i32>().expect("Line is not in proper format"))
        .collect()
}

fn parse_card_id(line: &str) -> i32 {
    let card_id : i32 = line.split_whitespace()
        .map(|x| x.parse::<i32>())
        .filter(|x| x.is_ok())
        .next().expect("Did not find a valid card ID")
        .expect("Did not find a valid card ID");

    card_id
}

fn parse_line(line: &str) -> LotteryTicket {
    let mut ticket = LotteryTicket::new();

    let (card_id, card_numbers) = line.split_once(':')
        .expect("Line was not in proper format: {line}");
    ticket.card_num = parse_card_id(card_id);

    let (winning_nums, my_nums) = card_numbers.split_once('|')
        .expect("Line was not in proper format: {line}");

    ticket.winning_num = parse_numbers(winning_nums);
    ticket.my_num = parse_numbers(my_nums);

    ticket
}

fn problem_1(input: &str) {
    let score : i32 = input.lines()
        .map(|line| parse_line(line))
        .map(|ticket| ticket.get_score())
        .sum();

    println!("Score: {score}");
}

fn problem_2(input: &str) {
    let tickets : Vec<LotteryTicket> = input.lines()
        .map(|line| parse_line(line))
        .collect();

    let mut ticket_counts : HashMap<i32, i32> = HashMap::new();
    ticket_counts.insert(tickets[0].card_num, 1);

    for i in 0..tickets.len() {
        
        let ticket = &tickets[i];
        let ticket_count : i32 = *ticket_counts.entry(ticket.card_num).or_insert(1);

        // Update ticket counts for the number of winning tickets
        let winning_count = ticket.num_matches();
        let start_index = i+1;
        for j in start_index..(start_index+winning_count) {
            if j < tickets.len() {
                let next_card = tickets[j].card_num;
                let next_count = ticket_counts.entry(next_card).or_insert(1);
                *next_count += ticket_count;
            }
        }
    }

    let total_count : i32 = ticket_counts.iter()
        .fold(0, |acc, val| acc + val.1);

    println!("Ticket Count: {total_count}");
}

fn main() {
    let input_1 = include_str!("input.txt");
    problem_1(input_1);

    let input_2 = include_str!("input.txt");
    problem_2(input_2);
}

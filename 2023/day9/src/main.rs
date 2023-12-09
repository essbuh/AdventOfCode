#[derive(Debug)]
struct InputSequence {
    numbers: Vec<Vec<i64>>,
}
impl InputSequence {
    fn get_row(&self, i: usize) -> &Vec<i64> { &self.numbers[i] }
    fn get_row_mut(&mut self, i: usize) -> &mut Vec<i64> { &mut self.numbers[i] }

    fn extrapolate(&mut self) {
        for i in (1..self.numbers.len()).rev() {
            let row_a = self.get_row(i);
            let row_b = self.get_row(i - 1);
            let extrapolated_val = row_a.last().unwrap() + row_b.last().unwrap();
            
            self.get_row_mut(i - 1).push(extrapolated_val);
        }
    }

    fn extrapolate_backwards(&mut self) {
        for i in (1..self.numbers.len()).rev() {
            let row_a = self.get_row(i);
            let row_b = self.get_row(i - 1);
            let extrapolated_val = row_b.first().unwrap() - row_a.first().unwrap();
            
            self.get_row_mut(i - 1).insert(0, extrapolated_val);
        }
    }
}

fn get_next_sequence(sequence: &Vec<i64>) -> Vec<i64> {
    let mut next_sequence = Vec::new();

    for i in 0..(sequence.len()-1) {
        let a = sequence[i];
        let b = sequence[i + 1];
        let difference = b - a;
        next_sequence.push(difference);
    }

    next_sequence
}

fn parse_sequence(input: &str) -> InputSequence {
    let sequence: Vec<i64> = input.split(" ").map(|x| x.parse::<i64>().unwrap()).collect();

    let mut numbers = Vec::new();
    numbers.push(sequence);

    loop {
        let sequence = get_next_sequence(numbers.last().unwrap());
        let is_all_zeroes = sequence.iter().all(|x| x == &0);
        numbers.push(sequence);

        if is_all_zeroes {
            break;
        }
    }

    InputSequence { numbers }
}

fn part_1(input: &str) {
    let mut sequences : Vec<InputSequence> = 
        input.lines()
            .map(|line| parse_sequence(line))
            .collect();

    sequences.iter_mut().for_each(|s| s.extrapolate() );
    
    let extrapolated_vals : i64 = sequences.iter()
        .map(|s| s.numbers[0].last().unwrap())
        .sum();

    println!("Part 1: {extrapolated_vals}");
}

fn part_2(input: &str) {
    let mut sequences : Vec<InputSequence> = 
        input.lines()
            .map(|line| parse_sequence(line))
            .collect();

    sequences.iter_mut().for_each(|s| s.extrapolate_backwards() );
    
    let extrapolated_vals : i64 = sequences.iter()
        .map(|s| s.numbers[0].first().unwrap())
        .sum();

    println!("Part 2: {extrapolated_vals}");
}

fn main() {
    let input = include_str!("input.txt");
    part_1(input);
    part_2(input);
}

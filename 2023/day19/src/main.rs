use std::collections::HashMap;

#[derive(PartialEq, Debug, Clone)]
enum WorkflowResult {
    Workflow(String),
    Reject,
    Accept
}

#[derive(Debug)]
struct WorkflowCondition {    
    part_type: char,
    comparison: char,
    rating: i32,
}
impl WorkflowCondition {
    fn matches(self: &Self, part: &Part) -> bool {
        let value = part.ratings[&self.part_type];
        match self.comparison {
            '>' => { value > self.rating },
            '<' => { value < self.rating },
            _ => { panic!("Unknown comparision"); }
        }
    }
}

#[derive(Debug)]
struct WorkflowStep {
    condition: Option<WorkflowCondition>,
    result: WorkflowResult,
}
impl WorkflowStep {
    fn new(input: &str) -> WorkflowStep {
        let parts : Vec<&str> = input.split(':').collect();
        let mut condition = None;

        if parts.len() > 1 {
            let part_data = parts[0];
            condition = Some(WorkflowCondition {
                part_type: part_data.chars().nth(0).unwrap(),
                comparison: part_data.chars().nth(1).unwrap(),
                rating: part_data[2..].parse::<i32>().unwrap()
            });
        }

        let result = match parts.last().unwrap() {
            &"R" => { WorkflowResult::Reject }
            &"A" => { WorkflowResult::Accept },
            s => { WorkflowResult::Workflow(s.to_string()) }
        };

        WorkflowStep{ condition, result }
    }

    fn get_result(self: &Self, part: &Part) -> Option<WorkflowResult> {
        match &self.condition {
            Some(cond) => { if cond.matches(part) { Some(self.result.clone()) } else { None } },
            None => { Some(self.result.clone()) }
        }
    }
}

#[derive(Debug)]
struct Workflow {
    steps: HashMap<String, Vec<WorkflowStep>>,
}
impl Workflow {
    fn add_step(self: &mut Self, input: &str) {
        let start_idx = input.chars().into_iter().position(|c| c == '{').unwrap();
        let step_name = input[0..start_idx].to_string();
        
        let steps = input[start_idx+1..input.len()-1].split(',')
            .map(|s| WorkflowStep::new(s))
            .collect();

        self.steps.insert(step_name, steps);
    }

    fn run<'a>(self: &Self, parts: &'a Vec<Part>) -> Vec<&'a Part> {
        parts.iter()
            .filter(|p| self.get_result(*p) == WorkflowResult::Accept)
            .collect()
    }

    fn get_result(self: &Self, part: &Part) -> WorkflowResult {        
        let mut workflow_name = String::from("in");
        loop {
            let mut found_result = false;
            for step in &self.steps[&workflow_name] {
                match step.get_result(part) {
                    Some(result) => {
                        match result {
                            WorkflowResult::Workflow(w) => {
                                workflow_name = w;
                                found_result = true;
                                break;
                             }
                            other => { return other; }
                        }
                    },
                    None => {}
                }
            }

            if !found_result {
                return WorkflowResult::Reject;
            }
        }
    }

    fn get_child_combinations(self: &Self, result: &WorkflowResult, child_counts: &HashMap<char, (i64, i64)>) -> i64 {
        match result {
            WorkflowResult::Workflow(name) => {
                self.get_combinations(&self.steps[name], &child_counts)
            },
            WorkflowResult::Accept => { 
                child_counts.into_iter()
                    .map(|(_, v)| if v.1 >= v.0 { v.1 - v.0 + 1 } else { 0 })
                    .reduce(|a, b| a * b)
                    .unwrap()
            },
            WorkflowResult::Reject => { 0 }
        }
    }

    fn get_combinations(self: &Self, steps: &Vec<WorkflowStep>, part_limits: &HashMap<char, (i64, i64)>) -> i64 {        
        loop {
            let mut my_counts = part_limits.clone();
            let mut combinations: i64 = 0;

            for step in steps {
                match &step.condition {
                    Some(cond) => {
                        let mut child_counts = my_counts.clone();

                        if cond.comparison == '<' {
                            let mut new_result = my_counts[&cond.part_type];
                            new_result.1 = cond.rating as i64 - 1;
                            assert!(new_result.1 >= 0);
                            child_counts.insert(cond.part_type, new_result);
    
                            let mut new_result = my_counts[&cond.part_type];
                            new_result.0 = cond.rating as i64;
                            assert!(new_result.0 >= 0);
                            my_counts.insert(cond.part_type, new_result);    
                        } else {
                            let mut new_result = my_counts[&cond.part_type];
                            new_result.0 = cond.rating as i64 + 1;
                            assert!(new_result.1 >= 0);
                            child_counts.insert(cond.part_type, new_result);
    
                            let mut new_result = my_counts[&cond.part_type];
                            new_result.1 = cond.rating as i64;
                            assert!(new_result.0 >= 0);
                            my_counts.insert(cond.part_type, new_result);   
                        }

                        combinations += self.get_child_combinations(&step.result, &child_counts);
                    },
                    None => {
                        return combinations + self.get_child_combinations(&step.result, &my_counts);
                    }
                }
            }
        }
    }

    fn get_total_combinations(self: &Self, max_values: &HashMap<char, (i64, i64)>) -> i64 {
        let start_workflow = &self.steps["in"];
        self.get_combinations(&start_workflow, &max_values)
    }
}

#[derive(Debug)]
struct Part {
    ratings: HashMap<char, i32>,
}
impl Part {
    fn new(line: &str) -> Part {
        let qualifiers = line[1..line.len()-1].split(',');
        Part { 
            ratings : qualifiers.into_iter()
                        .map(|q| {
                            let bits : Vec<&str> = q.split('=').collect();
                            let qual_type = bits[0].chars().last().unwrap();
                            let qual_value = bits[1].parse::<i32>().unwrap();
                            (qual_type, qual_value)
                        }).collect()
        }
    }

    fn get_total_rating(self: &Self) -> i32 {
        self.ratings.iter()
            .map(|(_, v)| v)
            .sum()
    }
}

fn parse_input(input: &str) -> (Workflow, Vec<Part>) {
    let mut workflow = Workflow { steps: HashMap::new() };
    let mut parts = Vec::new();

    let mut reading_parts = false;
    for line in input.lines() {
        if line.is_empty() {
            reading_parts = true;
            continue;
        }

        if reading_parts {
            parts.push(Part::new(line));
        } else {
            workflow.add_step(line);
        }   
    }

    (workflow, parts)
}

fn part_1() {
    let input = include_str!("input.txt");
    let (workflow, parts) = parse_input(input);

    let accepted_parts = workflow.run(&parts);

    let result : i32 = accepted_parts.into_iter()
        .map(|p| p.get_total_rating())
        .sum();
    
    println!("Part 1: {result}");
}

fn part_2() {
    let input = include_str!("input.txt");
    let (workflow, _) = parse_input(input);

    let initial_count = (1, 4000);
    let mut part_limits : HashMap<char, (i64, i64)> = HashMap::new();
    part_limits.insert('x', initial_count);
    part_limits.insert('m', initial_count);
    part_limits.insert('a', initial_count);
    part_limits.insert('s', initial_count);


    let result = workflow.get_total_combinations(&part_limits);

    println!("Part 2: {result}");
}

fn main() {
    part_1();
    part_2();
}

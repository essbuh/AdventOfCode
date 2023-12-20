use std::{collections::{HashMap, VecDeque, HashSet}, fmt::Debug};
use num::integer::lcm;

#[derive(Debug)]
struct PulseQueue {
    queue: VecDeque<(String, String, PulseType)>,
    debug: bool
} 
impl PulseQueue {
    fn new(debug: bool) -> Self { PulseQueue { queue: VecDeque::new(), debug } }

    fn push(self: &mut Self, item: (String, String, PulseType)) {
        if self.debug { println!("-Queueing {} -{}-> {}", item.0, if item.2 == PulseType::High { "high" } else { "low" }, item.1); }
        self.queue.push_back(item)
    }

    fn pop(self: &mut Self) -> Option<(String, String, PulseType)> {
        self.queue.pop_front()
    }
}

fn process_queue(queue: &mut PulseQueue, map: &mut ComponentMap, desired_pulse: Option<(&str, &str, PulseType)>) -> (i64, i64, bool) {
    let mut num_high_pulses : i64 = 0;
    let mut num_low_pulses : i64 = 0;
    let mut received_desired = false;

    loop {
        match queue.pop() {
            Some((from, to, pulse_type)) => {
                match pulse_type {
                    PulseType::High => { num_high_pulses += 1; },
                    PulseType::Low => { num_low_pulses += 1; },
                }

                if desired_pulse.is_some() {
                    let pulse = desired_pulse.unwrap();
                    if from == pulse.0 && to == pulse.1 && pulse_type == pulse.2 {
                        received_desired = true;
                    }
                }

                if queue.debug { println!("+Processing {} -{}-> {}", from, if pulse_type == PulseType::High { "high" } else { "low" }, to); }
        
                match map.try_get_module(&to) {
                    Some(module) => { 
                        module.receive_pulse(&from, &pulse_type, queue); 
                        if queue.debug { println!("{to} now {module:?}"); }
                    },
                    None => { /* Consumed */}
                }
            },
            None => { break; }
        }
    }

    if queue.debug { println!("Queue complete!"); }
    (num_low_pulses, num_high_pulses, received_desired)
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum PulseType {
    High,
    Low,
}

trait PowerModule : Debug {
    fn connect_inputs(self: &mut Self, _inputs: &Vec<String>) {}
    fn as_str(self: &Self) -> String { String::new() }

    fn receive_pulse(self: &mut Self, _from: &String, _pulse_type: &PulseType, _output_queue: &mut PulseQueue) { panic!("Unimplemented"); }
    fn get_outputs(self: &Self) -> &Vec<String> { panic!("Unimplemented"); }
    fn reset(self: &mut Self) {}
}

#[derive(Debug)]
struct Broadcast {
    name: String,
    outputs: Vec<String>,
}
impl Broadcast {
    fn new(name: &str, outputs: &Vec<String>) -> Self {
        Broadcast { name: name.to_string(), outputs: outputs.clone() }
    }
}
impl PowerModule for Broadcast {
    fn get_outputs(self: &Self) -> &Vec<String> { &self.outputs }

    fn receive_pulse(self: &mut Self, _from: &String, pulse_type: &PulseType, output_queue: &mut PulseQueue) {
        for output in &self.outputs {
            output_queue.push((self.name.clone(), output.clone(), *pulse_type));
        }
    }
}

#[derive(Debug)]
struct FlipFlop {
    name: String,
    is_on: bool,
    outputs: Vec<String>,
}
impl FlipFlop {
    fn new(name: String, outputs: &Vec<String>) -> Self {
        FlipFlop { name, is_on: false, outputs: outputs.clone() }
    }
}
impl PowerModule for FlipFlop {
    fn get_outputs(self: &Self) -> &Vec<String> { &self.outputs }

    fn as_str(self: &Self) -> String { 
        if self.is_on { String::from("on") } else { String::from("off") }
    }
    
    fn receive_pulse(self: &mut Self, _from: &String, pulse_type: &PulseType, output_queue: &mut PulseQueue) {
        match pulse_type {
            PulseType::Low => {
                self.is_on = !self.is_on;

                let out_type = if self.is_on { PulseType::High } else { PulseType::Low };

                for output in &self.outputs {
                    output_queue.push((self.name.clone(), output.clone(), out_type));
                }
            },
            _ => {}
        }
    }

    fn reset(self: &mut Self) {
        self.is_on = false;
    }
}

#[derive(Debug)]
struct Conjunction {
    name: String,
    input_states: HashMap<String, PulseType>,
    outputs: Vec<String>,
}
impl Conjunction {
    fn new(name: String, outputs: &Vec<String>) -> Conjunction {
        Conjunction { name, input_states: HashMap::new(), outputs: outputs.clone() }
    }
}
impl PowerModule for Conjunction {
    fn get_outputs(self: &Self) -> &Vec<String> { &self.outputs }

    fn as_str(self: &Self) -> String { 
        let mut s = String::from("-");
        for (k, v) in &self.input_states {
            s += k;
            s += "|";
            s += match v {
                PulseType::High => "hi",
                PulseType::Low => "lo"
             };
            s += ",";
        }
        s
    }

    fn connect_inputs(self: &mut Self, inputs: &Vec<String>) {
        for input in inputs {
            self.input_states.insert(input.clone(), PulseType::Low);
        }
    }

    fn receive_pulse(self: &mut Self, from: &String, pulse_type: &PulseType, output_queue: &mut PulseQueue) {
        self.input_states.insert(from.clone(), *pulse_type);

        let pulse_type = match self.input_states.iter().all(|(_, v)| v == &PulseType::High) {
            true => PulseType::Low,
            false => PulseType::High
        };

        for output in &self.outputs {
            output_queue.push((self.name.clone(), output.clone(), pulse_type));
        }
    }

    fn reset(self: &mut Self) {
        for (_, v) in &mut self.input_states {
            *v = PulseType::Low;
        }
    }
}

#[derive(Debug)]
struct ComponentMap {
    modules: HashMap<String, Box<dyn PowerModule>>,
    buckets: Vec<HashSet<String>>,
    inputs : HashMap<String, Vec<String>>
}
impl ComponentMap {
    fn to_string(self: &Self) -> String {
        self.modules.iter()
            .map(|(k, v)| {
                k.clone() + "_" + &v.as_ref().as_str() + ";"
            })
            .collect()
    }

    fn from_input(input: &str) -> Self {
        let mut map = ComponentMap { modules: HashMap::new(), buckets: Vec::new(), inputs: HashMap::new() };
        let mut inputs: HashMap<String, Vec<String>> = HashMap::new();

        for line in input.lines() {
            let parts : Vec<&str> = line.split(" -> ").collect();
            let outputs : Vec<String> = parts[1].split(", ").map(|s| s.to_string()).collect();
            let mut part_name = parts[0].split_at(1).1;

            match parts[0].chars().nth(0).unwrap() {
                '%' => {
                    map.modules.insert(part_name.to_string(), Box::new(FlipFlop::new(part_name.to_string(), &outputs)));
                },
                '&' => {
                    map.modules.insert(part_name.to_string(), Box::new(Conjunction::new(part_name.to_string(), &outputs)));
                },
                _ => {
                    assert_eq!(parts[0], "broadcaster");
                    part_name = parts[0];
                    map.modules.insert(parts[0].to_string(), Box::new(Broadcast::new(parts[0], &outputs)));
                }
            }

            for output in &outputs {
                inputs.entry(output.clone()).or_insert(Vec::new()).push(part_name.to_string());
            }
        }

        for (k, v) in &inputs {
            match map.try_get_module(k) {
                Some(module) => { module.connect_inputs(v); },
                None => {}
            }
        }

        map.inputs = inputs;

        for output in map.modules.get("broadcaster").unwrap().get_outputs() {
            let mut new_set = HashSet::new();
            new_set.insert(output.clone());

            map.buckets.push(new_set);
        }

        let length = map.buckets.len();
        for i in 0..length {
            loop {
                let mut added_something = false;

                for node in map.buckets[i].clone() {
                    if node == "xn" { continue; }

                    match map.modules.get(&node) {
                        Some(module) => {
                            for output in module.get_outputs() {
                                if map.buckets[i].insert(output.clone()) {
                                    added_something = true;
                                }
                            }              
                        },
                        None => {}
                    }
                }

                if !added_something {
                    break;
                }
            }
        }

        map
    }

    fn try_get_module(self: &mut Self, module_name: &str) -> Option<&mut Box<dyn PowerModule>> {
        self.modules.get_mut(module_name)
    }

    fn reset(self: &mut Self) {
        for (_, v) in &mut self.modules {
            v.reset();
        }
    }
}

pub fn get_result_part1(input: &str, button_presses: usize, debug: bool) -> i64 {
    let mut map = ComponentMap::from_input(input);
    let mut queue = PulseQueue::new(debug);

    let mut remembered_states : Vec<String> = Vec::new();
    remembered_states.push(map.to_string());
    
    if debug { println!("{map:#?}"); }
    
    let mut results:  Vec<(i64, i64)> = Vec::new();

    let mut press_count = 0;
    let mut loop_start = usize::MAX;
    for i in 0..button_presses {
        press_count += 1;
        
        queue.push((String::from("button"), String::from("broadcaster"), PulseType::Low));        
        let result = process_queue(&mut queue, &mut map,  None);
        results.push((result.0, result.1));

        let map_str = map.to_string();
        match remembered_states.iter().position(|s| s == &map_str) {
            Some(index) => {
                println!("Found loop after {} button presses", i + 1);
                loop_start = index;
                break;
            },
            None => {
                remembered_states.push(map_str);
            }
        }
    }

    let mut loop_count = 1;
    let mut loop_remainder = 1;
    if loop_start < usize::MAX {
        let loop_end = results.len();
        let loop_size = loop_end - loop_start;
        
        loop_count = (button_presses - loop_start) / loop_size;
        loop_remainder = (button_presses - loop_start) % loop_size;
    }

    let (total_low, total_hi) = results.iter().enumerate()
                                    .map(|(i, x)| {
                                        let iteration_count = 
                                            if i < loop_start { 1 }
                                            else if i < loop_remainder { loop_count + 1 }
                                            else { loop_count }
                                            as i64;

                                        (x.0 * iteration_count, x.1 * iteration_count)
                                    })
                                    .reduce(|a, b| (a.0 + b.0, a.1 + b.1))
                                    .unwrap();

    if debug { println!("After {press_count} presses: Num Low: {total_low}, Num High: {total_hi}") }

    total_low * total_hi
}

pub fn get_result_part2(input: &str, debug: bool) -> i64 {
    let mut map = ComponentMap::from_input(input);
    let mut queue = PulseQueue::new(debug);

    if debug { println!("{map:#?}"); }

    let desired_inputs = map.inputs.get("xn").unwrap().clone();
    let mut loop_counts : Vec<i64> = Vec::new();

    for input in &desired_inputs {
        map.reset();

        let mut press_count = 0;
        loop {
            press_count += 1;
    
            queue.push((String::from("button"), String::from("broadcaster"), PulseType::Low));        
            let result = process_queue(&mut queue, &mut map, Some((input, "xn", PulseType::High)));
            if result.2 {
                println!("rx recieved HIGH from {input} after {press_count} presses!");
                loop_counts.push(press_count);
                break;
            }
    
            if debug {
                println!(" ");
                println!("------");
                println!("State after {press_count} presses: {map:#?}");
                println!("------");
            }
    
            let map_str = map.to_string();
            if debug {
                println!(" -> Storing state {map_str}");
                println!(" ");
            }
        }
    }

    loop_counts.into_iter()
        .reduce(|a, b| lcm(a, b))
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_sample_1() {
        let result = get_result_part1(include_str!("sample_1.txt"), 1000, false);
        assert_eq!(result, 32000000);
    }

    #[test]
    fn part_1_sample_2() {
        let result = get_result_part1(include_str!("sample_2.txt"), 1000, false);
        assert_eq!(result, 11687500);
    }
}
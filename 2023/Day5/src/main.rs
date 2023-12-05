use std::{collections::HashMap, str::Lines};
use core::cmp::min;

#[derive(Debug)]
struct NumberMapping {
    source_start: i64,
    dest_start: i64,
    range_size: i64
}
impl NumberMapping {
    fn map_value(&self, value: i64) -> Option<i64> {
        if value >= self.source_start && value <= (self.source_start + self.range_size) {
            let mapped_value = (value - self.source_start) + self.dest_start;
            return Some(mapped_value);
        }

        None
    }
}

#[derive(Debug)]
struct NumberMap {
    mappings: Vec<NumberMapping>
}
impl NumberMap {
    fn new() -> NumberMap {
        NumberMap { mappings : Vec::new() }
    }

    fn map_value(&self, value: i64) -> i64 {
        for mapping in &self.mappings {
            match mapping.map_value(value) {
                Some(x) => { return x; },
                None => ()
            }
        }

        value
    }
}

#[derive(Debug)]
struct AlmanacEntry {
    target_entry: String,
    map: NumberMap,
}

#[derive(Debug)]
struct SeedRange {
    min: i64,
    len: i64,
}

#[derive(Debug)]
enum SeedType {
    Seed(i64),
    Range(SeedRange),
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<SeedType>,
    maps: HashMap<String, AlmanacEntry>
}
impl Almanac {
    fn get_seed_locations(self: &Self, seed_start: i64, seed_num: i64) -> Vec<i64> {
        let mut map_entry = self.maps.get("seed").expect("Almanac has no seed maps");
        println!("Mapping values from SEED map...");

        let mut values : Vec<i64> = (seed_start..(seed_start + seed_num)).map(|x| x).collect();
        loop {
            // Map all values to next level of map
            values = values.iter().map(|x| map_entry.map.map_value(*x)).collect();

            // Increment map level
            match self.maps.get(&map_entry.target_entry) {
                Some(entry) => { map_entry = entry; println!("Next map type: {}...", &map_entry.target_entry); },
                None => { break; }
            }
        }

        values
    }

    fn get_lowest_seed_location(self: &Self, seed: &SeedType) -> i64 {
        let mut locations : Vec<i64>;

        println!("Checking lowest seed: {:?}", seed);

        match seed {
            SeedType::Range(range) => { locations = self.get_seed_locations(range.min, range.len); },
            SeedType::Seed(value) => { locations = self.get_seed_locations(*value, 1); }
        }

        locations.sort();
        locations[0]
    }

    fn get_lowest_location(self: &Self) -> i64 {
        let mut lowest_loc = i64::MAX;

        for seed in &self.seeds {
            let location = self.get_lowest_seed_location(seed);
            lowest_loc = min(lowest_loc, location);
        }

        lowest_loc
    }
}

fn parse_seeds(input: &mut Lines, almanac: &mut Almanac, use_seed_ranges: bool) {
    let seeds_line : &str = input.next().expect("Incorrect input format");
    assert_eq!(&seeds_line[0..7], "seeds: ");    
    let seeds : Vec<i64> = seeds_line[7..].split(' ').map(|x| x.parse::<i64>().expect("Invalid input")).collect();

    if use_seed_ranges {
        // Must be a pair of seeds
        assert_eq!(seeds.len() % 2, 0);
        for i in (0..seeds.len()).step_by(2) {
            let range = SeedRange { min: seeds[i], len: seeds[i+1] };
            almanac.seeds.push(SeedType::Range(range));
        }
    } else {
        almanac.seeds = seeds.iter().map(|x| SeedType::Seed(*x)).collect();
    }

    let next_line = input.next().expect("Incorrect input format");
    assert!(next_line.is_empty());
}

fn parse_range(input: &str) -> NumberMapping {
    let numbers : Vec<i64> = input.split(' ').map(|x| x.parse::<i64>().expect("Incorrect input format")).collect();
    assert_eq!(numbers.len(), 3);
    
    NumberMapping {
        dest_start: numbers[0],
        source_start: numbers[1],
        range_size: numbers[2]
    }
}

fn parse_map_ranges(input: &mut Lines) -> NumberMap {
    let mut map = NumberMap::new();

    loop {
        match input.next() {
            Some(line) => {
                if !line.is_empty() {
                    map.mappings.push(parse_range(line));
                } else {
                    break;                    
                }
            }
            None => { break; }
        }
    }

    map
}

fn parse_map(header: &str, input: &mut Lines, almanac: &mut Almanac) {
    let (map_type, map_text) = header.split_once(' ').expect("Incorrect input format");
    assert_eq!(map_text, "map:");
    let map_types : Vec<&str> = map_type.split('-').collect();
    assert_eq!(map_types.len(), 3);

    let from_type = map_types[0];
    let to_type = map_types[2];

    let entry = AlmanacEntry {
        target_entry: to_type.to_string(),
        map: parse_map_ranges(input)
    };
    
    almanac.maps.insert(from_type.to_string(), entry);
}

fn try_parse_map(input: &mut Lines, almanac: &mut Almanac) -> bool {

    let map_header = input.next();
    match map_header {
        Some(header) => { parse_map(&header, input, almanac); return true; },
        None => { return false; }
    }
}

fn parse_almanac(input: &mut Lines, use_seed_ranges: bool) -> Almanac {
    let mut almanac = Almanac { seeds: Vec::new(), maps: HashMap::new() };

    parse_seeds(input, &mut almanac, use_seed_ranges);

    // parse all available maps we can find
    let mut result = true;
    while result {
        result = try_parse_map(input, &mut almanac);
    }

    almanac
}

fn main() {
    let input = include_str!("input.txt");
    
    {
        let use_seed_range = false;
        let mut lines = input.lines();    
        let almanac = parse_almanac(&mut lines, use_seed_range);
        println!("Finished parsing almanac.");
    
        //println!("Almanac: {:#?}", almanac);  
    
        let lowest_loc = almanac.get_lowest_location();
        println!("Lowest Location (range={use_seed_range}): {lowest_loc}");   
    }

    {
        let use_seed_range = true;
        let mut lines = input.lines();    
        let almanac = parse_almanac(&mut lines, use_seed_range);
        println!("Finished parsing almanac.");
    
        //println!("Almanac: {:#?}", almanac);  
    
        let lowest_loc = almanac.get_lowest_location();
        println!("Lowest Location (range={use_seed_range}): {lowest_loc}");   
    }
}

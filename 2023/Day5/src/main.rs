use std::{str::Lines, time::SystemTime};

#[derive(Debug, Copy, Clone)]
struct Range {
    start: i64,
    end: i64,
}
impl Range {
    fn contains_range(&self, range: &Range) -> bool {
        self.start <= range.start
            && self.end >= range.end
    }
}

#[derive(Debug)]
struct MapConversion {
    source: Range,
    dest: Range,
}
impl MapConversion {
    fn convert_range(&self, range: &Range) -> Range {
        if self.source.contains_range(range) {
            let start = (range.start - self.source.start) + self.dest.start;
            let end = (range.end - range.start) + start;
            return Range { start, end };
        }
        
        return *range;
    }
}

#[derive(Debug)]
struct AlmanacMap {
    conversions: Vec<MapConversion>,
}
impl AlmanacMap {
    fn split_ranges(&self, range: &Range) -> Vec<Range> {
        let mut queue : Vec<Range> = Vec::new();
        queue.push(*range);

        let mut result : Vec<Range> = Vec::new();

        loop {
            match queue.pop() {
                Some(r) => {
                    let mut found_match = false;
                    for conversion in &self.conversions {
                        let start = r.start;
                        let end = r.end;

                        if start < conversion.source.start {
                            if end < conversion.source.start {
                                // fully before this entry
                                result.push(Range { start, end });
                            } else {
                                result.push(Range { start, end: conversion.source.start - 1 });
                                
                                // process next time
                                queue.push(Range { start: conversion.source.start, end });
                            }                            

                            found_match = true;
                            break;
                        } else if start <= conversion.source.end {
                            if end <= conversion.source.end {
                                // fully inside this entry
                                result.push(Range { start, end });
                            } else {
                                result.push(Range { start, end: conversion.source.end });

                                // process next time
                                queue.push(Range { start: conversion.source.end + 1, end });
                            }
                            found_match = true;
                            break;
                        }
                    }

                    if !found_match {
                        // No conversion found, just map the range directly
                        result.push(*range);
                    }
                 }
                None => { break; }
            }
        }

        result
    }

    fn convert_range(&self, range: &Range) -> Range {
        self.conversions.iter()
            .find(|conv| conv.source.contains_range(range))
            .map(|conv| conv.convert_range(range))
            .unwrap_or(*range)
    }

    fn map_values(&self, value: &Vec<Range>) -> Vec<Range> {
        value.iter()
            .map(|range| self.split_ranges(&range))
            .flatten()
            .map(|range| self.convert_range(&range))
            .collect()
    }
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<Range>,
    maps: Vec<AlmanacMap>
}
impl Almanac {
    fn get_lowest_seed_location(self: &Self) -> i64 {
        let mut locations = self.seeds.to_vec();
        
        for map in &self.maps {
            locations = map.map_values(&locations);
        }

        locations.sort_by(|a, b| a.start.cmp(&b.start));

        assert!(!locations.is_empty());

        locations[0].start
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
            let range = Range { start: seeds[i], end: (seeds[i] + seeds[i+1] - 1) };
            almanac.seeds.push(range);
        }
    } else {
        almanac.seeds = seeds.iter().map(|x| Range { start: *x, end: *x } ).collect();
    }

    //almanac.seeds.sort_by(|a, b| a.start.cmp(&b.start));
    
    let next_line = input.next().expect("Incorrect input format");
    assert!(next_line.is_empty());
}

fn parse_map_line(input: &str) -> MapConversion {
    let numbers : Vec<i64> = input.split(' ').map(|x| x.parse::<i64>().expect("Incorrect input format")).collect();
    assert_eq!(numbers.len(), 3);
    
    MapConversion {
        source: Range { start: numbers[1], end: numbers[1] + numbers[2] - 1 },
        dest: Range { start: numbers[0], end: numbers[0] + numbers[2] - 1 }
    }
}

fn parse_map_ranges(input: &mut Lines) -> Vec<MapConversion> {
    let mut map = Vec::new();
    map.reserve(14);

    loop {
        match input.next() {
            Some(line) => {
                if !line.is_empty() {
                    map.push(parse_map_line(line));
                } else {
                    break;                    
                }
            }
            None => { break; }
        }
    }

    map.sort_by(|a, b| a.source.start.cmp(&b.source.start));

    map
}

fn parse_map(input: &mut Lines, almanac: &mut Almanac) {
    let map = AlmanacMap { conversions: parse_map_ranges(input) };
    almanac.maps.push(map);
}

fn try_parse_map(input: &mut Lines, almanac: &mut Almanac) -> bool {
    let now = SystemTime::now();
    let map_header = input.next();
    match map_header {
        Some(_) => { parse_map(input, almanac); println!("Map parse took {} ms ", (now.elapsed().unwrap().as_micros() as f32 / 1000.0)); return true; },
        None => { return false; }
    }
}

fn parse_almanac(input: &mut Lines, use_seed_ranges: bool) -> Almanac {
    let mut almanac = Almanac { seeds: Vec::new(), maps: Vec::new() };

    let now = SystemTime::now();
    parse_seeds(input, &mut almanac, use_seed_ranges);
    println!("Seed parse took {} ms ", (now.elapsed().unwrap().as_micros() as f32 / 1000.0));
    
    // parse all available maps we can find
    let mut result = true;
    while result {
        result = try_parse_map(input, &mut almanac);
    }

    almanac
}

fn run_test(input: &str, use_seed_range: bool) {
    let mut lines = input.lines();    

    let now = SystemTime::now();        
    let almanac = parse_almanac(&mut lines, use_seed_range);
    println!("Finished parsing almanac in {} ms", (now.elapsed().unwrap().as_micros() as f32 / 1000.0));

    let now = SystemTime::now();
    let lowest_loc = almanac.get_lowest_seed_location();
    println!("Lowest Location (range={use_seed_range}): {lowest_loc} | time spent: {} ms", (now.elapsed().unwrap().as_micros() as f32 / 1000.0));   
}

fn main() {
    let input = include_str!("input.txt");
    
    run_test(input, false);
    run_test(input, true);
}

use std::{/*fs,*/ collections::{/*HashSet,*/ HashMap}};

const CHAR_BROKEN: char = '#';
const CHAR_WORKING: char = '.';
const CHAR_UNKNOWN: char = '?';

type SpringRow = (Vec<char>, Vec<i64>);
type SpringMap = Vec<SpringRow>;

fn parse_input(input: &str, duplicate_count: i64) -> SpringMap {
    let mut map : SpringMap = input.lines().into_iter().map(|line| {
        let (left, right) = line.split_once(' ').unwrap();
        let counts = right.split(',').map(|x| x.parse::<i64>().unwrap()).collect();
        (left.chars().collect(), counts)
    }).collect();

    for i in 0..map.len() {
        let (dupe_chars, dupe_groups) = map[i].clone();
        let (chars, groups) = &mut map[i];
        for _ in 0..duplicate_count {
            chars.push('?');
            chars.append(&mut dupe_chars.clone());

            groups.append(&mut dupe_groups.clone());
        }
    }

    map
}

#[allow(dead_code)]
fn is_valid_permutation(perm: &str, row: &SpringRow) -> bool {
    if perm.len() != row.0.len() {
        let s : String = row.0.iter().collect();
        panic!("Permutation {perm} invalid: size mismatch {}", s);
    }

    for (i, char) in perm.chars().enumerate() {
        if char == CHAR_BROKEN {
            if &row.0[i] == &CHAR_WORKING {
                panic!("Permutation {perm} invalid: bad value at col {i}");
            }
        } else if char == CHAR_WORKING {
            if &row.0[i] == &CHAR_BROKEN {
                panic!("Permutation {perm} invalid: bad value at col {i}, was . should be #");
            }
        }
    }

    let group_lengths : Vec<i64> = perm.split(CHAR_WORKING)
        .filter(|x| !x.is_empty())
        .map(|x| x.len() as i64)
        .collect();
    if group_lengths.len() != row.1.len() {
        panic!("Permutation {perm} invalid: mismatch group lengths");
    }
    for i in 0..group_lengths.len() {
        if group_lengths[i] != row.1[i] {
            panic!("Permutation {perm} invalid: mismatch group length");
        }
    }

    //println!("Permutation {perm} VALID");
    true
}

fn get_permutations(spring_row: &SpringRow/*, prefix: &str, out_perms: &mut Vec<String>*/, cache: &mut HashMap<SpringRow, i64>) -> i64 {
    if spring_row.1.is_empty() {
        return 0;
    }

    match cache.get(spring_row) {
        Some(x) => { return *x; }
        None => {},
    }

    let group_size = spring_row.1[0] as usize;

    // We need to leave at this many left at the end of the row
    let mut padding_num: i64 = spring_row.1[1..].into_iter().sum();
    padding_num += spring_row.1.len() as i64 - 1;

    if padding_num >= spring_row.0.len() as i64 {
        return 0;
    }

    let end_index = spring_row.0.len() - padding_num as usize;
    let working_set = &spring_row.0[0..end_index];
    
    let mut num_permutations = 0;

    let mut found_start = false;
    for i in 0..(working_set.len() - group_size + 1) {
        if found_start {
            // We have to start from this point, so no more permutations possible
            break;
        }

        let mut current_len = 0;
        for j in 0..group_size {
            match working_set[i+j] {
                CHAR_BROKEN => { current_len += 1; if current_len == 1 { found_start = true;} },
                CHAR_UNKNOWN => { current_len += 1; },
                CHAR_WORKING => { break; }
                _ => { panic!("Found unknown char"); }
            }
        }

        if current_len == group_size {
            if (i + group_size) < spring_row.0.len() && spring_row.0[i + group_size] == CHAR_BROKEN {
                // This group would bleed outside of the working set
                continue;
            }

            let next_slice_start = i+group_size+1;
            if spring_row.1.len() > 1 {
                assert!(next_slice_start < spring_row.0.len());

                /*let mut next_prefix = prefix.to_string();
                for _ in 0..i {
                    next_prefix += ".";
                }
                for _ in 0..group_size {
                    next_prefix += "#";
                }

                next_prefix += "."; // must have a space*/

                let sub_permutations = get_permutations(&(spring_row.0[next_slice_start..].to_vec(), spring_row.1[1..].to_vec()), cache/*, &next_prefix, out_perms*/);
                if sub_permutations > 0 {
                    num_permutations = num_permutations + sub_permutations;
                }
            } else {
                // if this is the last group and there's still more known broken chars, this doesn't work!
                if next_slice_start >= spring_row.0.len() || !spring_row.0[next_slice_start..].contains(&CHAR_BROKEN) {
                    // this is the last one!
                    num_permutations = num_permutations + 1;

                    /*let mut next_prefix = prefix.to_string();
                    for _ in 0..i {
                        next_prefix += ".";
                    }
                    for _ in 0..group_size {
                        next_prefix += "#";
                    }
                    if next_slice_start <= spring_row.0.len() {
                        next_prefix += "."; // must have a space
                    }
                    for _ in next_slice_start..spring_row.0.len() {
                        next_prefix += ".";
                    }
                    out_perms.push(next_prefix);*/
                }
            }
        }
    }
    
    cache.insert(spring_row.clone(), num_permutations);

    num_permutations
}

fn part_1() {
    let input = include_str!("input.txt");
    let spring_map = parse_input(input, 0);

    let mut sum = 0;
    let mut cache = HashMap::new();
    //let mut all_perms :Vec<String> = Vec::new();
    let num_rows = spring_map.len();
    for (i, row) in spring_map.iter().enumerate() {
        //let mut perms = Vec::new();
        let perm = get_permutations(&row/*, "", &mut perms*/, &mut cache);
        //let perm_s : String = row.0.iter().collect();
        println!("[{}/{num_rows}]: found {perm} permutations!", (i+1));
        sum = sum + perm;

        //let mut perm_set : HashSet<String> = HashSet::new();
        //for perm in &perms {
            //perm_set.insert(perm.clone());
            // sanity check this permutation matches
            //if !is_valid_permutation(&perm, &row) {}
        //}

        //if perm_set.len() != perms.len() {
            //panic!("Duplicate permutations found!");            
        //}

        //all_perms.push(perms.join("\n"));
    }

//    fs::write("output.txt", all_perms.join("\n\n")).expect("msg");

    println!("Part 1: {sum}");
}

fn part_2() {
    let input = include_str!("input.txt");
    let spring_map = parse_input(input, 4);

    let mut sum = 0;
    let mut cache = HashMap::new();
    //let mut all_perms :Vec<String> = Vec::new();
    let num_rows = spring_map.len();
    for (i, row) in spring_map.iter().enumerate() {
        //let mut perms = Vec::new();
        let perm = get_permutations(&row/*, "", &mut perms*/, &mut cache);
        //let perm_s : String = row.0.iter().collect();
        println!("[{}/{num_rows}]: found {perm} permutations!", (i+1));
        sum = sum + perm;

        //let mut perm_set : HashSet<String> = HashSet::new();
        //for perm in &perms {
            //perm_set.insert(perm.clone());
            // sanity check this permutation matches
            //if !is_valid_permutation(&perm, &row) {}
        //}

        //if perm_set.len() != perms.len() {
            //panic!("Duplicate permutations found!");            
        //}

        //all_perms.push(perms.join("\n"));
    }

    println!("Part 2: {sum}");
}

fn main() {
    part_1();
    part_2();
}

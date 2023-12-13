use std::collections::HashSet;

const REFLECTION_VERT : bool = true;
const REFLECTION_HORIZ : bool = false;

type ReflectionPoint = (bool, usize, usize);

#[derive(Debug)]
struct Pattern {
    rows: Vec<Vec<char>>,
}
impl Pattern {
    fn get_value(self: &Self, row: usize, col: usize, smudges: &HashSet<(usize, usize)>) -> char {
        match smudges.get(&(row, col)) {
            Some(x) => {
                match self.rows[x.0][x.1] {
                    '#' => { '.' },
                    '.' => { '#' },
                    _ => { panic!("Invalid char"); }
                }
            },
            None => { self.rows[row][col] }
        }
    }

    fn is_reflection_vert(self: &Self, col_a: usize, col_b : usize, existing_smudges: &HashSet<(usize, usize)>, allowed_smudges: usize, needed_smudges: &mut HashSet<(usize, usize)>) -> bool {
        let mut is_reflection = true;        
        let mut smudges = HashSet::new();

        for row in 0..self.rows.len() {
            if self.get_value(row, col_a, &existing_smudges) != self.get_value(row, col_b, &existing_smudges) {
                if smudges.len() >= allowed_smudges {
                    is_reflection = false;
                    break;
                } else {
                    smudges.insert((row, col_b));
                }
            }
        }

        if is_reflection {
            needed_smudges.extend(smudges);
        }

        is_reflection
    }

    fn is_reflection_horiz(self: &Self, row_a: usize, row_b : usize, existing_smudges: &HashSet<(usize, usize)>, allowed_smudges: usize, needed_smudges: &mut HashSet<(usize, usize)>) -> bool {
        let mut is_reflection = true;
        let mut smudges = HashSet::new();
        
        for col in 0..self.rows[0].len() {
            if self.get_value(row_a, col, &existing_smudges) != self.get_value(row_b, col, &existing_smudges) {
                if smudges.len() >= allowed_smudges {
                    is_reflection = false;
                    break;
                } else {
                    smudges.insert((row_b, col));
                }
            }
        }

        if is_reflection {
            needed_smudges.extend(smudges);
        }

        is_reflection
    }

    fn get_reflection_size_vert(self: &Self, point: &ReflectionPoint, existing_smudges: &HashSet<(usize, usize)>, allowed_smudges: usize, required_smudges: &mut HashSet<(usize, usize)>) -> usize {
        let mut reflection_size = 1;
        let mut smudges_used = HashSet::new();
        
        for i in 1..(point.1+1) {
            let col_a = point.1 - i;
            let col_b = point.2 + i;
            if col_b >= self.rows[0].len() {
                // hit the end!
                break;
            }

            if !self.is_reflection_vert(col_a, col_b, existing_smudges, allowed_smudges - smudges_used.len(), &mut smudges_used) {
                return 0;
            } else {
                reflection_size += 1;
            }
        }

        required_smudges.extend(smudges_used);
        reflection_size
    }

    fn get_reflection_size_horiz(self:  &Self, point: &ReflectionPoint, existing_smudges: &HashSet<(usize, usize)>, allowed_smudges: usize, required_smudges: &mut HashSet<(usize, usize)>) -> usize {
        let mut reflection_size = 1;
        let mut smudges_used = HashSet::new();

        for i in 1..(point.1+1) {
            let row_a = point.1 - i;
            let row_b = point.2 + i;
            if row_b >= self.rows.len() {
                // hit the end!
                break;
            }

            if !self.is_reflection_horiz(row_a, row_b, existing_smudges, allowed_smudges - smudges_used.len(), &mut smudges_used) {
                return 0;
            } else {
                reflection_size += 1;
            }
        }

        required_smudges.extend(&smudges_used);
        reflection_size
    }

    #[allow(dead_code)]
    fn print(self: &Self) {
        println!("        ");
        for row in &self.rows {
            let s : String = row.iter().collect();
            println!("{s}");
        }
    }

    fn find_reflection(self: &Self, allowed_smudges: usize) -> (ReflectionPoint, usize) {
        let mut max_reflect_horiz_len = 0;
        let mut max_reflect_horiz: (bool, usize, usize) = (REFLECTION_HORIZ, 0, 0);
        let mut all_smudges = HashSet::new();

        let dim_y = self.rows.len();
        let dim_x = self.rows[0].len();
        //self.print();
        for row in 0..dim_y-1 {
            let mut is_mirror = true;
            let mut smudges = HashSet::new();

            for col in 0..dim_x {
                if self.get_value(row, col, &all_smudges) != self.get_value(row+1, col, &all_smudges) {
                    if smudges.len() >= allowed_smudges {
                        is_mirror = false;
                        break;
                    } else {
                        smudges.insert((row+1, col));
                    }
                }
            }

            if is_mirror {
                let reflection_point = (REFLECTION_HORIZ, row, row+1);
                
                let mut used_smudges = all_smudges.clone();
                used_smudges.extend(&smudges.clone());

                let reflection_size = self.get_reflection_size_horiz(&reflection_point, &used_smudges, allowed_smudges - all_smudges.len(), &mut smudges);
                if reflection_size > 0 && reflection_point.0 >= max_reflect_horiz.0 && all_smudges.is_empty() {
                    max_reflect_horiz_len = reflection_size;
                    max_reflect_horiz = reflection_point;
                    all_smudges = smudges;
                }
            }
        }

        let mut max_reflect_vert_len = 0;
        let mut max_reflect_vert = (REFLECTION_VERT, 0, 0);
        if all_smudges.is_empty() {
            for col in 0..dim_x-1 {
                let mut is_mirror = true;
                let mut smudges = HashSet::new();
                for row in 0..dim_y {
                    if self.get_value(row, col, &all_smudges) != self.get_value(row, col+1, &all_smudges) {
                        if smudges.len() + all_smudges.len() >= allowed_smudges {
                            is_mirror = false;
                            break;
                        } else {
                            smudges.insert((row, col+1));
                        }
                    }
                }
    
                if is_mirror {
                    let reflection_point = (REFLECTION_VERT, col, col+1);
                    
                    let mut used_smudges = all_smudges.clone();
                    used_smudges.extend(&smudges.clone());
    
                    let reflection_size = self.get_reflection_size_vert(&reflection_point, &used_smudges, allowed_smudges - all_smudges.len(), &mut smudges);
    
                    if reflection_size > 0 && reflection_point.0 >= max_reflect_vert.0 && all_smudges.is_empty() {
                        max_reflect_vert_len = reflection_size;
                        max_reflect_vert = reflection_point;
                        all_smudges = smudges;
    
                        if all_smudges.len() > 0 {
                            // if we found something with a vertical smudge, invalidate the horizontal one
                            max_reflect_horiz_len = 0;
                        }
                    }
                }
            }
        }

        /*if max_reflect_horiz_len > 0 {
            println!("Pattern found horizontal mirror starting row {}, length {}, cost {}, smudges: {:?}", max_reflect_horiz.1, max_reflect_horiz_len, (max_reflect_horiz.1 + 1) * 100, all_smudges);
        }
        if max_reflect_vert_len > 0 {
            println!("Pattern found vertical mirror starting col {} length {}, cost {}, smudges: {:?}", max_reflect_vert.1, max_reflect_vert_len, max_reflect_vert.1 + 1, all_smudges);
        }*/

        assert!(allowed_smudges == 0 || all_smudges.len() != 0);
        assert_ne!(max_reflect_horiz_len != 0, max_reflect_vert_len != 0);

        if max_reflect_horiz_len > 0 {
            (max_reflect_horiz, max_reflect_horiz_len)
        } else {
            (max_reflect_vert, max_reflect_vert_len)
        }
    }
}

#[derive(Debug)]
struct MirrorMap {
    patterns: Vec<Pattern>
}
impl MirrorMap {
    fn find_reflections(self: &Self, allowed_smudges: usize) -> Vec<(ReflectionPoint, usize)> {
        self.patterns.iter()
            .map(|p| p.find_reflection(allowed_smudges))
            .collect()
    }
}

fn parse_input(input: &str) -> MirrorMap {
    let mut map = MirrorMap { patterns: Vec::new() };
    map.patterns.push(Pattern { rows: Vec::new() });

    let mut pattern_index = 0;
    for line in input.lines() {
        if line.is_empty() {
            pattern_index += 1;
            map.patterns.push(Pattern { rows: Vec::new() });
        } else {
            map.patterns[pattern_index].rows.push(line.chars().collect());
        }
    }

    map
}

fn get_result(input: &str, allowed_smudges: usize) -> usize {
    let map = parse_input(&input);
    //println!("{:#?}", map);
    let reflection_points = map.find_reflections(allowed_smudges);

    reflection_points.iter()
        .map(|r| if r.0.0 == REFLECTION_VERT { r.0.1 + 1 } else { 100 * (r.0.1 + 1) } )
        .sum()
}

fn part_1() {
    let input = include_str!("input.txt");
    let sum = get_result(input, 0);
    println!("Part 1: {sum}");
}

fn part_2() {
    let input = include_str!("input.txt");
    let sum = get_result(input, 1);
    println!("Part 2: {sum}");
}

fn main() {
    part_1();
    part_2();
}

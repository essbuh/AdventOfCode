mod types;

use types::Hailstorm;

#[allow(dead_code)]
fn part_1(input: &str, bounds: (i64, i64), debug: bool) -> usize {
    let snowstorm = Hailstorm::from_input(input);
    snowstorm.count_intersections((bounds.0, bounds.1, i64::MAX), debug)
}

#[allow(dead_code)]
fn part_2(input: &str, debug: bool) -> i64 {
    let hailstorm = Hailstorm::from_input(input);
    let rock = hailstorm.find_common_rock();
    if debug {
        println!("Found rock: {} {} {} @ {} {} {}",
            rock.pos.x, rock.pos.y, rock.pos.z,
            rock.vel.x, rock.vel.y, rock.vel.z);
    }
    rock.pos.x + rock.pos.y + rock.pos.z
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample() {
        let result = part_1(include_str!("sample.txt"), (7, 27), true);
        println!("Part 1 (Sample): {result}");
        assert_eq!(result, 2);
    }

    #[test]
    fn part1_input() {
        let result = part_1(include_str!("input.txt"), (200000000000000, 400000000000000), false);
        println!("Part 1 (Real): {result}");
        assert_eq!(result, 20847);
    }

    #[test]
    fn part2_sample() {
        let result = part_2(include_str!("sample.txt"), true);
        println!("Part 2 (Sample): {result}");
        assert_eq!(result, 47);
    }

    #[test]
    fn part2_input() {
        let result = part_2(include_str!("input.txt"), false);
        println!("Part 2 (Real): {result}");
        assert_eq!(result, 908621716620524);
    }
}

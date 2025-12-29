use crate::common::Range;
use std::{collections::HashSet, hash::RandomState, str::Lines};

struct Model {
    ranges: Vec<Range>,
}

fn parse_input(input: Lines) -> Model {
    let mut ranges = Vec::new();

    for line in input {
        if line.len() == 0 {
            break;
        }

        let (start, end) = line.split_once("-").unwrap();
        ranges.push(Range {
            start: start.parse().unwrap(),
            end: end.parse().unwrap(),
        });
    }

    ranges.sort();

    Model { ranges }
}

fn calculate_result(model: Model) -> usize {
    let mut i = 0;
    let mut ranges = model.ranges;

    while i < ranges.len() {
        let curr_range = ranges[i];
        assert!(curr_range.start <= curr_range.end);
        println!("Currently examining {curr_range:?}");

        ranges.retain_mut(|other_range| {
            assert!(other_range.start <= other_range.end);

            // If the start is less than our start, it is before us
            // in the vec, so it is guaranteed that there is no overlapping
            if other_range.start < curr_range.start {
                // println!("  keep {other_range:?} unchanged (before)");
                assert!(other_range.end < curr_range.start);
                return true;
            }

            // Keep the current range
            // TODO could we be double counting here?
            if curr_range == *other_range {
                return true;
            }

            // Keep any following, non-overlapping ranges
            if other_range.start > curr_range.end {
                // println!("  keep {other_range:?} unchanged (after)");
                return true;
            }

            if other_range.end <= curr_range.end {
                // Remove it if entirely contained in this range
                println!("  removing {other_range:?}");
                return false;
            } else {
                // Partially overlapping with our range: remove the overlapping region
                let new_val = curr_range.end + 1;
                println!("  changing {other_range:?} to {new_val}");
                other_range.start = curr_range.end + 1;
                return true;
            }
        });
        i += 1;
        ranges.sort();
    }

    let unique_ranges: HashSet<&Range, RandomState> = HashSet::from_iter(ranges.iter());

    let mut total = 0;
    for range in unique_ranges {
        assert!(range.start <= range.end);
        total += range.end - range.start + 1;
    }
    total
}

pub fn run(input: Lines) -> usize {
    let model = parse_input(input);
    return calculate_result(model);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        let data = include_str!("../tests/day5/sample.txt").lines();
        assert_eq!(run(data), 14);
    }

    #[test]
    #[ignore]
    fn challenge() {
        let data = include_str!("../tests/day5/challenge.txt").lines();
        assert_eq!(run(data), 352509891817881);
    }
}

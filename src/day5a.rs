use crate::common::Range;
use std::str::Lines;

struct Model {
    ranges: Vec<Range>,
    ids: Vec<usize>,
}

fn parse_input(input: Lines) -> Model {
    let mut parsing_ranges = true;
    let mut ranges = Vec::new();
    let mut ids = Vec::new();

    for line in input {
        if parsing_ranges {
            if line.len() == 0 {
                parsing_ranges = false;
                continue;
            }
            let (start, end) = line.split_once("-").unwrap();
            ranges.push(Range {
                start: start.parse().unwrap(),
                end: end.parse().unwrap(),
            });
        } else {
            ids.push(line.parse().unwrap());
        }
    }

    ranges.sort();

    Model { ranges, ids }
}

fn calculate_result(model: Model) -> usize {
    let mut fresh = 0;
    for id in model.ids {
        for range in model.ranges.iter() {
            if id >= range.start && id <= range.end {
                fresh += 1;
                break;
            }
        }
    }
    fresh
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
        assert_eq!(run(data), 3);
    }

    #[test]
    #[ignore]
    fn challenge() {
        let data = include_str!("../tests/day5/challenge.txt").lines();
        assert_eq!(run(data), 674);
    }
}

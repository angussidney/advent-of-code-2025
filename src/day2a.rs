use std::str::Lines;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Range {
    start: usize,
    end: usize,
}

struct Model {
    ranges: Vec<Range>,
}

fn parse_input(input: Lines) -> Model {
    let mut ranges = Vec::new();
    for line in input {
        for range in line.split(",") {
            let Some((start_str, end_str)) = range.split_once("-") else {
                panic!("Invalid input format");
            };
            let start = start_str.parse::<usize>().unwrap();
            let end = end_str.parse::<usize>().unwrap();
            ranges.push(Range { start, end });
        }
    }

    ranges.sort();

    Model { ranges }
}

fn digits(id: usize) -> u32 {
    id.ilog10() + 1
}

fn duplicate_id(id: usize) -> usize {
    id * 10_usize.pow(digits(id)) + id
}

fn calculate_result(model: Model) -> usize {
    let mut total_invalid = 0;
    let mut half_id = 1;
    let mut full_id = duplicate_id(half_id);
    let mut last_end = 0;

    for range in model.ranges {
        // Validate no overlapping ranges
        if last_end >= range.start {
            panic!("Overlapping");
        }

        while full_id <= range.end {
            if full_id >= range.start {
                total_invalid += full_id;
            }
            half_id += 1;
            full_id = duplicate_id(half_id);
        }

        last_end = range.end;
    }

    total_invalid
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
        let data = include_str!("../tests/day2/sample.txt").lines();
        assert_eq!(run(data), 1227775554);
    }

    #[test]
    #[ignore]
    fn challenge() {
        let data = include_str!("../tests/day2/challenge.txt").lines();
        assert_eq!(run(data), 35367539282);
    }
}

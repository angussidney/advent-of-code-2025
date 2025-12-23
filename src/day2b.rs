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

fn prefix_repeats(str: &str, prefix: &str) -> bool {
    return str.trim_start_matches(prefix).len() == 0;
}

fn calculate_result(model: Model) -> usize {
    let mut total_invalid = 0;

    for range in model.ranges {
        'num: for num in range.start..=range.end {
            let num_str = num.to_string();
            for prefix_len in 1..=num_str.len() / 2 {
                let prefix = &num_str[..prefix_len];
                if prefix_repeats(&num_str, prefix) {
                    total_invalid += num;
                    continue 'num;
                }
            }
        }
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
        assert_eq!(run(data), 4174379265);
    }

    #[test]
    #[ignore]
    fn challenge() {
        let data = include_str!("../tests/day2/challenge.txt").lines();
        assert_eq!(run(data), 45814076230);
    }
}

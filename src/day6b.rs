use std::{iter::zip, str::Lines};

#[derive(Clone, Copy)]
enum Operation {
    Plus,
    Multiply,
}

struct Model {
    problems: Vec<Vec<usize>>,
    ops: Vec<Operation>,
}

fn parse_input(input: Lines) -> Model {
    let mut lines: Vec<Vec<char>> = Vec::new();
    let mut ops_strs: Option<&str> = None;

    for line in input {
        match ops_strs {
            Some(line) => lines.push(line.chars().collect()),
            None => {}
        }
        ops_strs = Some(line);
    }

    let ops: Vec<Operation> = ops_strs
        .unwrap()
        .split_whitespace()
        .map(|c| match c {
            "+" => Operation::Plus,
            "*" => Operation::Multiply,
            _ => panic!("Invalid operation {c}"),
        })
        .collect();

    let length = lines[0].len();
    let mut problems = Vec::new();
    let mut nums: Vec<usize> = Vec::new();
    for i in 0..length {
        let digits: Vec<char> = lines
            .iter()
            .map(|line| line[i])
            .filter(|x| *x != ' ')
            .collect();
        if digits.len() == 0 {
            problems.push(nums);
            nums = Vec::new();
        } else {
            nums.push(digits.iter().collect::<String>().parse().unwrap());
        }
    }
    problems.push(nums);

    Model { problems, ops }
}

fn calculate_result(model: Model) -> usize {
    let mut total = 0;
    for (nums, op) in zip(model.problems, model.ops) {
        total += nums
            .into_iter()
            .reduce(|x, y| match op {
                Operation::Plus => x + y,
                Operation::Multiply => x * y,
            })
            .unwrap();
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
        let data = include_str!("../tests/day6/sample.txt").lines();
        assert_eq!(run(data), 3263827);
    }

    #[test]
    #[ignore]
    fn challenge() {
        let data = include_str!("../tests/day6/challenge.txt").lines();
        assert_eq!(run(data), 10875057285868);
    }
}

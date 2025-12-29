use std::str::{Lines, SplitWhitespace};

#[derive(Clone, Copy)]
enum Operation {
    Plus,
    Multiply,
}

struct Model {
    nums: Vec<Vec<usize>>,
    ops: Vec<Operation>,
}

fn parse_input(input: Lines) -> Model {
    let mut nums: Vec<Vec<usize>> = Vec::new();
    let mut strs: Option<SplitWhitespace<'_>> = None;

    for line in input {
        match strs {
            Some(str_list) => nums.push(str_list.map(|str: &str| str.parse().unwrap()).collect()),
            None => {}
        }
        strs = Some(line.split_whitespace());
    }

    let ops: Vec<Operation> = strs
        .unwrap()
        .map(|c| match c {
            "+" => Operation::Plus,
            "*" => Operation::Multiply,
            _ => panic!("Invalid operation {c}"),
        })
        .collect();

    let length = ops.len();
    for vec in nums.iter() {
        if vec.len() != length {
            panic!("Mismatched line lengths");
        }
    }

    Model { nums, ops }
}

fn calculate_result(model: Model) -> usize {
    let mut total = 0;
    for problem in 0..model.ops.len() {
        let op = model.ops[problem];
        let nums: Vec<_> = model.nums.iter().map(|line| line[problem]).collect();

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
        assert_eq!(run(data), 4277556);
    }

    #[test]
    #[ignore]
    fn challenge() {
        let data = include_str!("../tests/day6/challenge.txt").lines();
        assert_eq!(run(data), 4580995422905);
    }
}

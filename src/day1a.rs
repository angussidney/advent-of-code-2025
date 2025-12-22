use std::str::Lines;

enum Rotation {
    Left(usize),
    Right(usize),
}

struct Model {
    start_rot: isize,
    rotations: Vec<Rotation>,
}

fn parse_input(input: Lines) -> Model {
    let mut rotations = Vec::new();
    for line in input {
        let mut chars = line.chars();
        let dir = chars.next().unwrap();
        let clicks = chars
            .as_str()
            .parse::<usize>()
            .expect("Rotation should be an integer");
        rotations.push(match dir {
            'L' => Rotation::Left(clicks),
            'R' => Rotation::Right(clicks),
            _ => panic!("Invalid direction specified"),
        })
    }
    Model {
        start_rot: 50,
        rotations: rotations,
    }
}

fn calculate_result(model: Model) -> usize {
    let mut pos = model.start_rot;
    let mut zeroes = 0;
    for rot in model.rotations {
        pos = match rot {
            Rotation::Left(clicks) => pos.checked_sub_unsigned(clicks).unwrap(),
            Rotation::Right(clicks) => pos.checked_add_unsigned(clicks).unwrap(),
        };
        pos = pos.rem_euclid(100);

        if pos == 0 {
            zeroes += 1;
        }
    }
    zeroes
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
        let data = include_str!("../tests/day1/sample.txt").lines();
        assert_eq!(run(data), 3);
    }

    #[test]
    #[ignore]
    fn challenge() {
        let data = include_str!("../tests/day1/challenge.txt").lines();
        assert_eq!(run(data), 1147);
    }
}

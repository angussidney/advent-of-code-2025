use std::str::Lines;

#[derive(Copy, Clone)]
enum Rotation {
    Left(usize),
    Right(usize),
}

impl Rotation {
    fn clicks(self) -> usize {
        match self {
            Rotation::Left(clicks) => clicks,
            Rotation::Right(clicks) => clicks,
        }
    }
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
        let prev_pos = pos;
        let full_rots = rot.clicks() / 100;
        let rem_rots = rot.clicks() % 100;

        zeroes += full_rots;

        pos = match rot {
            Rotation::Left(_) => pos.checked_sub_unsigned(rem_rots).unwrap(),
            Rotation::Right(_) => pos.checked_add_unsigned(rem_rots).unwrap(),
        };

        if prev_pos != 0 {
            if pos <= 0 || pos >= 100 {
                zeroes += 1;
            }
        }

        pos = pos.rem_euclid(100);
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
        assert_eq!(run(data), 6);
    }

    #[test]
    #[ignore]
    fn challenge() {
        let data = include_str!("../tests/day1/challenge.txt").lines();
        assert_eq!(run(data), 6789);
    }
}

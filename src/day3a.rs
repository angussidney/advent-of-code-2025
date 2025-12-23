use std::str::Lines;

struct Model {
    banks: Vec<Vec<usize>>,
}

fn parse_input(input: Lines) -> Model {
    let mut banks = Vec::new();

    for bank in input {
        banks.push(
            bank.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect(),
        );
    }

    Model { banks }
}

fn calculate_result(model: Model) -> usize {
    let mut total_jolts = 0;

    for bank in model.banks {
        let first_digit = bank[..bank.len() - 1].iter().max().unwrap();
        let first_index = bank.iter().position(|c| c == first_digit).unwrap();

        let second_digit = bank[first_index + 1..bank.len()].iter().max().unwrap();

        let jolts = first_digit * 10 + second_digit;
        println!("{jolts}");
        total_jolts += jolts;
    }

    total_jolts
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
        let data = include_str!("../tests/day3/sample.txt").lines();
        assert_eq!(run(data), 357);
    }

    #[test]
    #[ignore]
    fn challenge() {
        let data = include_str!("../tests/day3/challenge.txt").lines();
        assert_eq!(run(data), 17613);
    }
}

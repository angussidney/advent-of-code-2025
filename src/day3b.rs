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
        let mut digits = Vec::new();

        let mut last_index = -1;
        for digit in (0..=11).rev() {
            let start_search = (last_index + 1) as usize;
            let end_search = bank.len() - digit;
            let search_digits = &bank[start_search..end_search];

            let digit = search_digits.iter().max().unwrap();
            digits.push(*digit);

            let this_index = start_search + search_digits.iter().position(|c| c == digit).unwrap();
            last_index = this_index as isize;
        }

        let jolts = digits
            .into_iter()
            .reduce(|acc, digit| acc * 10 + digit)
            .unwrap();
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
        assert_eq!(run(data), 3121910778619);
    }

    #[test]
    #[ignore]
    fn challenge() {
        let data = include_str!("../tests/day3/challenge.txt").lines();
        assert_eq!(run(data), 175304218462560);
    }
}

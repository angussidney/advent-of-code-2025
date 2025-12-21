use std::str::Lines;

pub fn run(_input: Lines) -> usize {
    println!("Day1a!");
    3
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        let data = include_str!("../tests/day1a/sample.txt").lines();
        assert_eq!(run(data), 3);
    }

    #[test]
    #[ignore]
    fn challenge() {
        let data = include_str!("../tests/day1a/challenge.txt").lines();
        assert_eq!(run(data), 0);
    }
}

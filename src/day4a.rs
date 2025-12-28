use crate::common::Grid;
use std::str::Lines;

#[derive(Clone, Copy, PartialEq, Eq)]
enum GridSq {
    PaperRoll,
    Empty,
}

struct Model {
    grid: Grid<GridSq>,
}

fn parse_input(input: Lines) -> Model {
    Model {
        grid: Grid::new(input, |c| match c {
            '@' => GridSq::PaperRoll,
            '.' => GridSq::Empty,
            _ => panic!("Invalid grid square"),
        }),
    }
}

fn calculate_result(model: Model) -> usize {
    let adj_rolls = model.grid.map(|coord| {
        model
            .grid
            .iter_diag_adj(coord)
            .filter(|(_, sq)| *sq == GridSq::PaperRoll)
            .count()
    });

    return adj_rolls
        .iter_squares()
        .filter(|(_, count)| *count < 4)
        .count();
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

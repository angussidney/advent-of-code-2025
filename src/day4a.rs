use crate::common::Grid;
use std::str::Lines;

#[derive(Clone, Copy, PartialEq, Eq)]
enum GridSq {
    PaperRoll,
    Empty,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum GridSqAdj {
    PaperRoll(usize),
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
            sq => panic!("Invalid grid square {sq}"),
        }),
    }
}

fn calculate_result(model: Model) -> usize {
    let adj_rolls = model.grid.map(|coord, val| match val {
        GridSq::PaperRoll => GridSqAdj::PaperRoll(
            model
                .grid
                .iter_diag_adj(coord)
                .filter(|(_, sq)| *sq == GridSq::PaperRoll)
                .count(),
        ),
        GridSq::Empty => GridSqAdj::Empty,
    });

    return adj_rolls
        .iter_squares()
        .filter(|(_, adj)| match *adj {
            GridSqAdj::PaperRoll(count) => count < 4,
            GridSqAdj::Empty => false,
        })
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
        let data = include_str!("../tests/day4/sample.txt").lines();
        assert_eq!(run(data), 13);
    }

    #[test]
    #[ignore]
    fn challenge() {
        let data = include_str!("../tests/day4/challenge.txt").lines();
        assert_eq!(run(data), 1409);
    }
}

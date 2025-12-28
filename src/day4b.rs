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
            sq => panic!("Invalid grid square {sq}"),
        }),
    }
}

fn calculate_result(model: Model) -> usize {
    let mut total_removed = 0;
    let mut grid = model.grid;
    loop {
        let mut removed = 0;

        grid = grid.map(|coord, val| match val {
            GridSq::PaperRoll => {
                let adj = grid
                    .iter_diag_adj(coord)
                    .filter(|(_, sq)| *sq == GridSq::PaperRoll)
                    .count();
                if adj < 4 {
                    removed += 1;
                    return GridSq::Empty;
                } else {
                    return GridSq::PaperRoll;
                }
            }
            GridSq::Empty => GridSq::Empty,
        });

        if removed == 0 {
            break;
        }
        total_removed += removed;
    }

    return total_removed;
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
        assert_eq!(run(data), 43);
    }

    #[test]
    #[ignore]
    fn challenge() {
        let data = include_str!("../tests/day4/challenge.txt").lines();
        assert_eq!(run(data), 8366);
    }
}

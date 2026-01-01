use crate::common::{Grid, GridCoord, GridOffset};
use std::str::Lines;

#[derive(Clone, Copy)]
enum Operation {
    Plus,
    Multiply,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum GridSq {
    Start,
    Splitter(usize),
    Empty,
}

impl GridSq {
    fn is_empty(&self) -> bool {
        match self {
            GridSq::Empty => true,
            GridSq::Start => true,
            _ => false,
        }
    }
}

struct Model {
    grid: Grid<GridSq>,
    start: GridCoord,
}

fn parse_input(input: Lines) -> Model {
    let grid = Grid::new(input, |c| match c {
        'S' => GridSq::Start,
        '^' => GridSq::Splitter(0),
        '.' => GridSq::Empty,
        _ => panic!("Invalid input char in grid"),
    });

    let start = grid
        .iter_squares()
        .filter_map(|(coord, sq)| match sq {
            GridSq::Start => Some(coord),
            _ => None,
        })
        .next()
        .expect("S must appear somewhere in the grid");

    Model { grid, start }
}

fn propogate_beam(grid: &Grid<GridSq>, start: GridCoord, n: usize) -> usize {
    let mut coord = start;
    while grid.get(coord).unwrap().is_empty() {
        if let Some(next) = grid.offset_coord(coord, GridOffset::DOWN) {
            coord = next;
            continue;
        } else {
            // Bottomed out
            return n;
        }
    }
    // Found a splitter
    if let Some(GridSq::Splitter(prev)) = grid.get(coord) {
        grid.set(coord, GridSq::Splitter(prev + n));
        return 0; // didn't bottom out
    } else {
        panic!("while loop condition should prevent this");
    }
}

fn calculate_result(model: Model) -> usize {
    let grid = model.grid;

    assert!(propogate_beam(&grid, model.start, 1) == 0);

    let mut timelines = 0;

    for (coord, sq) in grid.iter_squares() {
        if let GridSq::Splitter(n) = sq {
            if let Some(left) = grid.offset_coord(coord, GridOffset::LEFT) {
                timelines += propogate_beam(&grid, left, n);
            }
            if let Some(right) = grid.offset_coord(coord, GridOffset::RIGHT) {
                timelines += propogate_beam(&grid, right, n);
            }
        }
    }

    timelines
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
        let data = include_str!("../tests/day7/sample.txt").lines();
        assert_eq!(run(data), 40);
    }

    #[test]
    #[ignore]
    fn challenge() {
        let data = include_str!("../tests/day7/challenge.txt").lines();
        assert_eq!(run(data), 48989920237096);
    }
}

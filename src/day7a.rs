use crate::common::{Grid, GridCoord, GridOffset};
use std::{collections::HashSet, str::Lines};

#[derive(Clone, Copy, PartialEq, Eq)]
enum GridSq {
    Start,
    Splitter,
    Empty,
    TachyonBeam,
}

struct Model {
    grid: Grid<GridSq>,
    start: GridCoord,
}

fn parse_input(input: Lines) -> Model {
    let grid = Grid::new(input, |c| match c {
        'S' => GridSq::Start,
        '^' => GridSq::Splitter,
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

fn calculate_result(model: Model) -> usize {
    let mut splits = HashSet::new();

    let grid = model.grid;

    let mut todo: Vec<GridCoord> = Vec::new();
    todo.push(model.start);

    while let Some(beam_start) = todo.pop() {
        let mut coord = beam_start;
        // println!("processing {coord:?}");
        loop {
            match grid.get(coord).unwrap() {
                GridSq::TachyonBeam => break, // Hit existing beam; already processed
                GridSq::Splitter => {
                    assert!(splits.insert(coord));
                    if let Some(next) = grid.offset_coord(coord, GridOffset::LEFT) {
                        todo.push(next);
                    }
                    if let Some(next) = grid.offset_coord(coord, GridOffset::RIGHT) {
                        todo.push(next);
                    }
                    // Beam stops going down when it hits a splitter
                    break;
                }
                _ => {
                    // Empty or start (essentially empty)
                    grid.set(coord, GridSq::TachyonBeam);
                    if let Some(next) = grid.offset_coord(coord, GridOffset::DOWN) {
                        coord = next;
                        continue;
                    } else {
                        // Hit bottom of grid; no need to keep going
                        break;
                    }
                }
            }
        }
    }
    splits.len()
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
        assert_eq!(run(data), 21);
    }

    #[test]
    #[ignore]
    fn challenge() {
        let data = include_str!("../tests/day7/challenge.txt").lines();
        assert_eq!(run(data), 1626);
    }
}

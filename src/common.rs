use std::str::Lines;

#[derive(PartialEq, Eq, Clone, Copy)]
struct GridCoord {
    x: usize,
    y: usize,
}

#[derive(PartialEq, Eq, Clone, Copy)]
struct GridOffset {
    dx: isize,
    dy: isize,
}

pub struct Grid<T> {
    grid: Vec<Vec<T>>,
    width: usize,
    height: usize,
}

impl<T: Copy + Clone> Grid<T> {
    pub fn new<F>(input: Lines, mapping: F) -> Grid<T>
    where
        F: Fn(char) -> T,
    {
        let mut rows = Vec::new();
        let mut width: Option<usize> = None;
        let mut height: usize = 0;
        for line in input {
            height += 1;
            let mut row = Vec::new();
            for c in line.chars() {
                row.push(mapping(c));
            }
            match width {
                Some(w) => assert!(w == row.len()),
                None => width = Some(row.len()),
            }
            rows.push(row);
        }
        Grid {
            grid: rows,
            width: width.expect("Must be at least one line in input grid"),
            height: height,
        }
    }

    pub fn get(&self, coord: GridCoord) -> Option<T> {
        if coord.x >= self.width || coord.y >= self.height {
            return None;
        } else {
            return Some(self.grid[coord.y][coord.x]);
        }
    }

    pub fn get_offset(&self, coord: GridCoord, off: GridOffset) -> Option<T> {
        let new_x = coord.x.checked_add_signed(off.dx);
        let new_y = coord.y.checked_add_signed(off.dy);

        return self.get(GridCoord {
            x: new_x?,
            y: new_y?,
        });
    }

    pub fn get_offset_coords(&self, coord: GridCoord, off: GridOffset) -> Option<GridCoord> {
        let new_x = coord.x.checked_add_signed(off.dx)?;
        let new_y = coord.y.checked_add_signed(off.dy)?;

        if new_x >= self.width || new_y >= self.height {
            return None;
        }

        Some(GridCoord { x: new_x, y: new_y })
    }

    pub fn iter_diag_adj(&self, coord: GridCoord) -> GridIter<T> {
        GridIter {
            grid: self,
            index: 0,
            iter_type: GridIterTypes::DiagAdjSquares(coord),
        }
    }

    pub fn iter_cardinal_adj(&self, coord: GridCoord) -> GridIter<T> {
        GridIter {
            grid: self,
            index: 0,
            iter_type: GridIterTypes::CardinalAdjSquares(coord),
        }
    }

    pub fn iter_squares(&self) -> GridIter<T> {
        GridIter {
            grid: self,
            index: 0,
            iter_type: GridIterTypes::AllSquares,
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum GridIterTypes {
    AllSquares,
    DiagAdjSquares(GridCoord),
    CardinalAdjSquares(GridCoord),
}

struct GridIter<'a, T> {
    grid: &'a Grid<T>,
    index: usize,
    iter_type: GridIterTypes,
}

impl<T> GridIter<'_, T> {}

impl<T: Clone + Copy> Iterator for GridIter<'_, T> {
    type Item = (GridCoord, T);

    fn next(&mut self) -> Option<Self::Item> {
        const DIAG_ADJ: [GridOffset; 8] = [
            GridOffset { dx: -1, dy: -1 },
            GridOffset { dx: 0, dy: -1 },
            GridOffset { dx: 1, dy: -1 },
            GridOffset { dx: 1, dy: 0 },
            GridOffset { dx: 1, dy: 1 },
            GridOffset { dx: 0, dy: 1 },
            GridOffset { dx: -1, dy: 1 },
            GridOffset { dx: -1, dy: 0 },
        ];
        const CARD_ADJ: [GridOffset; 4] = [
            GridOffset { dx: -1, dy: 0 },
            GridOffset { dx: 0, dy: -1 },
            GridOffset { dx: 1, dy: 0 },
            GridOffset { dx: 0, dy: 1 },
        ];

        let curr_index = self.index;
        self.index += 1;

        match self.iter_type {
            GridIterTypes::AllSquares => {
                if curr_index >= self.grid.width * self.grid.height {
                    return None;
                }

                let coord = GridCoord {
                    x: curr_index / self.grid.width,
                    y: curr_index % self.grid.width,
                };
                return Some((coord, self.grid.get(coord).unwrap()));
            }
            GridIterTypes::CardinalAdjSquares(coord) => {
                if curr_index >= CARD_ADJ.len() {
                    return None;
                }

                let off = CARD_ADJ[curr_index];
                let new = self.grid.get_offset_coords(coord, off);
                match new {
                    Some(new_coords) => Some((
                        new_coords,
                        self.grid
                            .get(new_coords)
                            .expect("get_offset_coords() and get() disagree"),
                    )),
                    None => self.next(),
                }
            }
            GridIterTypes::DiagAdjSquares(coord) => {
                if curr_index >= DIAG_ADJ.len() {
                    return None;
                }

                let off = DIAG_ADJ[curr_index];
                let new = self.grid.get_offset_coords(coord, off);
                match new {
                    Some(new_coords) => Some((
                        new_coords,
                        self.grid
                            .get(new_coords)
                            .expect("get_offset_coords() and get() disagree"),
                    )),
                    None => self.next(),
                }
            }
        }
    }
}

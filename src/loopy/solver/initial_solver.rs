use super::Solver;
use super::Status;

use super::coordinate::Coordinate;
use super::direction::Direction;

// Easy and obvious solver methods that should be applied at the begining of every puzzle.
impl Solver {
    fn hint_analysis(&mut self) {
        let mut has_4 = false;
        let mut has_2_or_3 = false;
        for i in 0..self.puzzle.size {
            for j in 0..self.puzzle.size {
                match self.puzzle.grid[i][j] {
                    Some(0) => {
                        // If a cell has the 0 hint, then all the edges must be off.
                        let c = Coordinate(i, j);
                        for d in Direction::iter() {
                            self.set(&self.edge_from_cell(&c, &d), false);
                        }
                    },
                    Some(2) => {
                        // If a cell has the 2 hint, then the solution can not be a loop around a
                        // single cell.
                        self.can_be_single_cell = false;
                        has_2_or_3 = true;
                    },
                    Some(3) => {
                        // If a cell has the 3 hint, then the solution can not be a loop around a
                        // single cell.
                        self.can_be_single_cell = false;
                        has_2_or_3 = true;
                    },
                    Some(4) => {
                        // If a cell has the 4 hint, then the solution must be a loop around a this
                        // single cell.
                        has_4 = true;
                        let c = Coordinate(i, j);
                        for d in Direction::iter() {
                            self.set(&self.edge_from_cell(&c, &d), true);
                        }
                    },
                    // If the cell has the 1 hint or no hints, no inferences can be made.
                    _ => {}
                }
            }
        }
        if has_4 && has_2_or_3 {
            self.status = Status::Unsolvable;
        }
    }

    fn corner_hint_analysis(&mut self) {
        for hd in [Direction::RIGHT, Direction::LEFT] {
            let col = if Direction::LEFT.eq(&hd) {0} else {self.puzzle.size - 1};
            for vd in [Direction::UP, Direction::DOWN] {
                let row = if Direction::UP.eq(&vd) {0} else {self.puzzle.size - 1};
                let cell = Coordinate(row, col);
                match self.puzzle.grid[row][col] {
                    Some(1) => {
                        // If 1 hint in a corner, then the "corner-most" edges must be off since
                        // they are either both on (impossible) or both off.
                        self.set(&self.edge_from_cell(&cell, &hd), false);
                        self.set(&self.edge_from_cell(&cell, &vd), false);
                    },
                    Some(2) => {
                        // If 2 hint in a corner, then the side edges coming out of the corner
                        // cell's corners must be on.
                        // For exmaple:
                        //  ┄ ┄ ┄      ┄ ─ ┄
                        // ┆2┆ ┆      ┆2┆ ┆
                        //  ┄ ┄ ┄  ->  ┄ ┄ ┄
                        // ┆ ┆ ┆      │ ┆ ┆
                        //  ┄ ┄ ┄      ┄ ┄ ┄
                        // ┆ ┆ ┆      ┆ ┆ ┆

                        // Unwrap is safe since we are in an hd most cell and going in the
                        // hd.opposite direction. And since we have a two hint we know the
                        // dimensions of the puzzle is more than 1 by 1.
                        let h_cell = self.cell_from_cell(&cell, &hd.opposite()).unwrap();
                        self.set(&self.edge_from_cell(&h_cell, &vd), true);
                        // Unwrap is safe here as well for similar reasons.
                        let v_cell = self.cell_from_cell(&cell, &vd.opposite()).unwrap();
                        self.set(&self.edge_from_cell(&v_cell, &hd), true);

                        // If we have a 3 hint next to a corner 2 hint, then we know that the edge
                        // of the 3 cell opposite to the 2 cell must be on.
                        // TODO: Generalize this argument to work for non-corner cells.
                        if self.is_cell_3(&h_cell) {
                            self.set(&self.edge_from_cell(&h_cell, &hd.opposite()), true)
                        }
                        if self.is_cell_3(&v_cell) {
                            self.set(&self.edge_from_cell(&v_cell, &vd.opposite()), true)
                        }
                    },
                    Some(3) => {
                        // If 3 hint in a corner, then the "corner-most" edges must be on since
                        // they are either both on or both off (impossible).
                        self.set(&self.edge_from_cell(&cell, &hd), true);
                        self.set(&self.edge_from_cell(&cell, &vd), true);
                    },
                    // If no hint, no inferences can be made.
                    // If 0 or 4 hint, no inference more than regular hint analysis can be made.
                    _ => {},
                }
            }
        }
    }

    pub fn handle_adjacent_3(&mut self) {
        // If a 3 is next to another 3, then the edge between them is on as well as the edges on
        // either side of them. We also know that the edges comming out of the sides must be off.
        // For example:
        //  ┄ ┄ ┄ ┄
        // ┆·┆· ·┆·┆
        //  ┄ ┄ ┄ ┄
        // ┆·│3│3│·┆
        //  ┄ ┄ ┄ ┄
        // ┆·┆· ·┆·┆
        //  ┄ ┄ ┄ ┄
        for i in 0..self.puzzle.size {
            for j in 0..self.puzzle.size {
                if self.is_3(i, j) {
                    if self.is_3(i + 1, j) {
                        self.set(&self.h_edges[i][j].clone(), true);
                        self.set(&self.h_edges[i + 1][j].clone(), true);
                        self.set(&self.h_edges[i + 2][j].clone(), true);
                        if j > 0 {
                            self.set(&self.h_edges[i + 1][j - 1].clone(), false);
                        }
                        if j < self.puzzle.size - 1 {
                            self.set(&self.h_edges[i + 1][j + 1].clone(), false);
                        }
                    }
                    if self.is_3(i, j + 1) {
                        self.set(&self.v_edges[i][j].clone(), true);
                        self.set(&self.v_edges[i][j + 1].clone(), true);
                        self.set(&self.v_edges[i][j + 2].clone(), true);
                        if i > 0 {
                            self.set(&self.v_edges[i - 1][j + 1].clone(), false);
                        }
                        if i < self.puzzle.size - 1 {
                            self.set(&self.v_edges[i + 1][j + 1].clone(), false);
                        }
                    }
                }
            }
        }
    }

    fn handle_diagonal_3(&mut self) {
        // If a 3 is diagonal to another 3 (with however many 2s in between), then their edges in
        // the their opposite corners are on.
        for i in 0..self.puzzle.size {
            for j in 0..self.puzzle.size {
                if self.is_3(i, j) {
                    // Look at bottom right diagonal
                    let mut next = 1;
                    while self.is_2(i + next, j + next) {
                        next = next + 1;
                    }
                    if self.is_3(i + next, j + next) {
                        self.set(&self.v_edges[i][j].clone(), true);
                        self.set(&self.h_edges[i][j].clone(), true);
                        self.set(&self.v_edges[i + next][j + next + 1].clone(), true);
                        self.set(&self.h_edges[i + next + 1][j + next].clone(), true);
                    }
                    // Look at top right diagonal
                    next = 1;
                    while j >= next && self.is_2(i + next, j - next) {
                        next = next + 1;
                    }
                    if j >= next && self.is_3(i + next, j - next) {
                        self.set(&self.v_edges[i][j + 1].clone(), true);
                        self.set(&self.h_edges[i][j].clone(), true);
                        self.set(&self.v_edges[i + next][j - next].clone(), true);
                        self.set(&self.h_edges[i + next + 1][j - next].clone(), true);
                    }
                }
            }
        }
    }

    pub fn initial_solve(&mut self) {
        self.hint_analysis();
        self.corner_hint_analysis();
        self.handle_adjacent_3();
        self.handle_diagonal_3();
    }
}

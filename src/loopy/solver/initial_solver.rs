use super::Solver;
use super::Status;

use super::coordinate::Coordinate;
use super::direction::Direction;
use super::direction::HDirection;
use super::direction::VDirection;

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
        for hd in [HDirection::LEFT, HDirection::RIGHT] {
            let col = if HDirection::LEFT.eq(&hd) {0} else {self.puzzle.size};
            for vd in [VDirection::UP, VDirection::DOWN] {
                let row = if VDirection::UP.eq(&vd) {0} else {self.puzzle.size};
                self.remove_entry_at_node(&Coordinate(row, col), &hd.opposite(), &vd.opposite());
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
        // NOTE: This rule has an exception!
        // It's possible, although very rare, that the whole loop is just the boundery of those
        // two 3s. But that won't be the case if there is at least one other hint that is a 2 or 3,
        // or at least one other hint that is a non-adjacent 1.
        let mut number_of_3s = 0;
        let mut has_2 = false;
        for i in 0..self.puzzle.size {
            for j in 0..self.puzzle.size {
                match self.puzzle.grid[i][j] {
                    // TODO, detect non adjacent 1s,
                    Some(2) => { has_2 = true; },
                    Some(3) => { number_of_3s += 1; },
                    _ => {}
                }
            }
        }
        let could_be_loop = number_of_3s <= 2 && !has_2;
        for i in 0..self.puzzle.size {
            for j in 0..self.puzzle.size {
                if self.is_3(i, j) {
                    if self.is_3(i + 1, j) {
                        self.set(&self.h_edges[i][j].clone(), true);
                        if !could_be_loop {
                            self.set(&self.h_edges[i + 1][j].clone(), true);
                        }
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
                        if !could_be_loop {
                            self.set(&self.v_edges[i][j + 1].clone(), true);
                        }
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

    // TODO: Figure out how to generalize this argument
    pub fn handle_3_next_to_1_on_edge(&mut self) {
        let n = self.puzzle.size;
        for i in 1..(self.puzzle.size - 1) {
            if self.is_3(i, 0) {
                if self.is_1(i - 1, 0) || self.is_1(i + 1, 0) {
                    self.set(&self.v_edges[i][0].clone(), true);
                }
            }
            if self.is_3(i, n - 1) {
                if self.is_1(i - 1, n - 1) || self.is_1(i + 1, n - 1) {
                    self.set(&self.v_edges[i][n].clone(), true);
                }
            }
            if self.is_3(0, i) {
                if self.is_1(0, i - 1) || self.is_1(0, i + 1) {
                    self.set(&self.h_edges[0][i].clone(), true);
                }
            }
            if self.is_3(n - 1, i) {
                if self.is_1(n - 1, i - 1) || self.is_1(n - 1, i + 1) {
                    self.set(&self.h_edges[n][i].clone(), true);
                }
            }
        }
    }

    pub fn initial_solve(&mut self) {
        self.hint_analysis();
        self.corner_hint_analysis();
        self.handle_adjacent_3();
        self.handle_diagonal_3();
        self.handle_3_next_to_1_on_edge()
    }
}

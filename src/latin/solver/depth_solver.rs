use super::Solver;
use super::Status;
use super::triple::*;

use std::collections::HashSet;

// Solver methods based on making a guess and seeing if we end up with a contradiction.
impl Solver {
    pub fn depth_solve(& mut self, depth: u8, should_log: bool) -> Vec<Solver> {
        let n = self.puzzle.size as u8;

        while self.status == Status::InProgress {
            // Find a cell with multiple possibilities.
            let mut row_col: RowCol = RowCol{ row: 0, col: 0 };
            let mut possibilities: &HashSet<u8>;
            loop {
                possibilities = self.row_col_map.get(&row_col).unwrap();
                if possibilities.len() > 1 {
                    break;
                }
                row_col.col += 1;
                if row_col.col >= n {
                    row_col.row += 1;
                    row_col.col = 0;
                }
            }

            // Try setting a possibility.
            // Try the highest value first as they tend to give the most information.
            let last = possibilities.iter().max().unwrap();

            let mut copy = self.clone();
            let guess = Triple{ row: row_col.row, col: row_col.col, val: last.clone()};
            if should_log {
                println!("\nStuck! Need to guess");
                println!("{}", self.to_string());
                println!("Guessing {}, in cell ({}, {})", guess.val + 1, guess.row + 1, guess.col + 1);
            }
            copy.to_set.insert(guess.clone());
            let solutions = copy.full_solve(depth + 1, should_log);
            if solutions.len() > 1 {
                // If more than one solution with this guess, then we can stop looking.
                self.status = Status::MultipleSolutions;
                return solutions;
            } else if solutions.is_empty() {
                if should_log {
                    println!("Guess leads to contradiciton; Reverting.");
                }
                // If no solutions with this guess, then we can remove this guess.
                self.to_remove.insert(guess.clone());
                // See if we can make more progress now that this guess is removed.
                self.non_recursive_solve();
            } else {
                // If exactly one solution with this guess, then we need to try it without the guess.
                copy = self.clone();
                copy.to_remove.insert(guess.clone());
                let other_solutions = copy.full_solve(depth + 1, should_log);
                if other_solutions.is_empty() {
                    // If no solutions without this guess, then we had the unique solution with
                    // this guess.
                    self.status = Status::UniqueSolution;
                    return solutions
                } else {
                    // If solutions without this guess, then we have multiple solutions.
                    self.status = Status::MultipleSolutions;
                    return [solutions, other_solutions].concat();
                }
            }

        }
        if self.status == Status::Unsolvable {
            return vec![];
        } else {
            return vec![self.clone()];
        }
    }
}

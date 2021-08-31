use std::collections::HashSet;
use super::Coordinate;
use super::Solver;
use super::Status;

// (row, column, value)
#[derive(Clone, Debug, Copy, Hash, PartialEq, Eq)]
pub struct Triple (usize, usize, u8);

// Solver methods based on making a guess and seeing if we end up with a contradiction.
impl Solver {
    pub fn depth_solve(& mut self, depth: u8, should_log: bool) -> Vec<Solver> {
        let n = self.puzzle.size;
        let mut to_remove: Option<Triple> = None;
        let mut can_skip: HashSet<Triple> = HashSet::new();
        let mut solutions: Vec<Solver> = Vec::new();

        for i in 0..n {
            for j in 0..n {
                let num = self.grid[i][j].len();
                if num == 1 {
                    continue;
                }
                let mut found_solution = false;
                for v in self.grid[i][j].iter() {
                    if can_skip.contains(&Triple(i, j, *v)) {
                        continue;
                    }
                    let mut copy = self.clone();
                    copy.set(&Coordinate(i, j), v);
                    let mut copy_solutions = copy.full_solve(depth + 1, should_log);
                    if copy.depth_needed > self.depth_needed {
                        self.depth_needed = copy.depth_needed;
                    }
                    if copy.status == Status::Unsolvable {
                        to_remove = Some(Triple(i, j, *v));
                        break;
                    } else if copy.status == Status::UniqueSolution || copy.status == Status::MultipleSolutions {
                        solutions.append(&mut copy_solutions);
                        if found_solution || copy.status == Status::MultipleSolutions {
                            self.status = Status::MultipleSolutions;
                            return solutions;
                        }
                        found_solution = true;
                        // If the copy is as solved as possible and that the current state of the
                        // copy can lead to a solution, this means that none of it's available
                        // choices can lead to a contradiction.
                        for x in i..n {
                            for y in 0..n{
                                for value in copy.grid[x][y].iter() {
                                    can_skip.insert(Triple(x, y, *value));
                                }
                            }
                        }
                    }
                }
                if to_remove.is_some() {
                    break;
                }
            }
            if to_remove.is_some() {
                break;
            }
        }

        match to_remove {
            Some(x) => {
                // We found a contradiction, so remove that possibility.
                self.remove(&Coordinate(x.0, x.1), &x.2);
            },
            None => {
                // No contradictions found, so every possible choice from here can lead to a
                // solution.
                self.status = Status::MultipleSolutions;
            }
        }

        return solutions;
    }
}

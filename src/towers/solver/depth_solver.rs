use std::collections::HashSet;
use super::Coordinate;
use super::Solver;
use super::Status;

// Solver methods based on making a guess and seeing if we end up with a contradiction.
impl<'a> Solver<'a> {
    pub fn depth_solve(& mut self, depth: u8) {
        if depth <= 0 {
            return;
        }
        let n = self.puzzle.size;
        let mut to_remove: Option<(Coordinate, u8)> = None;
        let mut to_set: Option<Vec<Vec<HashSet<u8>>>> = None;

        for i in 0..n {
            for j in 0..n {
                let num = self.grid[i][j].len();
                if num == 1 {
                    continue;
                }
                if num > 10 {
                    continue;
                }
                for v in self.grid[i][j].iter() {
                    let mut copy = self.clone();
                    copy.set(&Coordinate(i, j), v);
                    copy.full_solve(depth - 1);
                    if copy.status == Status::Unsolvable {
                        to_remove = Some((Coordinate(i, j), *v));
                        break;
                    } else if copy.status == Status::Solved {
                        to_set = Some(copy.grid.clone());
                        break;
                    }
                }
                if to_remove.is_some() || to_set.is_some() {
                    break;
                }
            }
            if to_remove.is_some() || to_set.is_some() {
                break;
            }
        }

        match to_remove {
            Some(x) => {
                self.remove(&x.0, &x.1);
            },
            None => {
                // Do nothing
            }
        }

        match to_set {
            Some(x) => {
                // Got lucky!
                for i in 0..n {
                    for j in 0..n {
                        self.set(&Coordinate(i, j), x[i][j].iter().next().unwrap());
                    }
                }
            },
            None => {
                // Do nothing
            }
        }
    }
}

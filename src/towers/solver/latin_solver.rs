use super::Coordinate;
use super::Solver;
use super::Status;
mod graph_solver;

// Returns how many times it had to backtrack
// fn increase_counter(counter: &mut Vec<usize>, pos: usize, max: usize) -> usize {
//     let size = counter.len();
//     let mut backtrack = 0;
//     let mut current_pos = pos;
//     counter[current_pos] += 1;
//     while counter[current_pos] > max + current_pos - size {
//         backtrack += 1;
//         if current_pos == 0  {
//             return backtrack;
//         }
//         current_pos -= 1;
//         counter[current_pos] += 1;
//     }
//     for i in (current_pos + 1)..size {
//         counter[i] = counter[i-1] + 1;
//     }
//     return backtrack;
// }
//
// fn columns_that_have_value(row: &Vec<HashSet<u8>>, value: &u8) -> HashSet<u8> {
//     return row.iter().enumerate().filter(|(_index, x)| x.contains(value)).map(|(index, _x)| index as u8).collect();
// }


// Solver methods based on the fact that a towers solution must be a latin square.
impl Solver {
    // If a cell has been solved, then no other cell in the same row or column can be that value.
    pub fn handle_solved_cells(& mut self) {
        while !self.recently_solved.is_empty() && self.status == Status::InProgress {
            let c = self.recently_solved.pop().unwrap();
            let value = self.grid[c.0][c.1].iter().next().unwrap().clone();
            for i in 0..self.puzzle.latin.size {
                if i != c.1 {
                    self.remove(&Coordinate(c.0, i), &value);
                }
                if i != c.0 {
                    self.remove(&Coordinate(i, c.1), &value);
                }
            }
        }
    }

    pub fn handle_unique_in_row(& mut self) {
        while !self.recently_unique_in_row.is_empty() && self.status == Status::InProgress {
            let (row, value) = self.recently_unique_in_row.pop().unwrap();
            for i in 0..self.puzzle.latin.size {
                if self.grid[row][i].contains(&value) {
                    self.set(&Coordinate(row, i), &value);
                }
            }
        }
    }

    pub fn handle_unique_in_column(& mut self) {
        while !self.recently_unique_in_column.is_empty() && self.status == Status::InProgress {
            let (column, value) = self.recently_unique_in_column.pop().unwrap();
            for i in 0..self.puzzle.latin.size {
                if self.grid[i][column].contains(&value) {
                    self.set(&Coordinate(i, column), &value);
                }
            }
        }
    }

    // Returns true if progress can be made using "simple" solving techniques.
    pub fn can_simple_solve(&mut self) -> bool {
        let has_recently_solved = !self.recently_solved.is_empty();
        let has_unique_in_row = !self.recently_unique_in_row.is_empty();
        let has_unique_in_column = !self.recently_unique_in_column.is_empty();
        let has_lead = has_recently_solved || has_unique_in_row || has_unique_in_column;
        return has_lead && self.status == Status::InProgress;
    }

    // Makes progress on the puzzle using "simple" methods.
    // The simple methods are:
    // 1. If a cell is solved, then no other cell in the same row/column can be that value.
    // 2. If a row/column only has one cell that can be a certain value, then that cell has to be
    // that value.
    // This method will apply these two rules over and over again until no more progress can be
    // made using these rules.
    pub fn simple_solve(&mut self) {
        while self.can_simple_solve() {
            self.handle_solved_cells();
            self.handle_unique_in_row();
            self.handle_unique_in_column();
        }
    }

    // pub fn grouping_solve(&mut self) {
    //     let n = self.puzzle.latin.size;
    //     for i in 0..n {
    //         // The values of the cells in row i (indexed by column);
    //         let row: Vec<&HashSet<u8>> = self.grid[i].iter().collect();
    //         let to_remove = Solver::row_group_solve(&row);
    //         for remove in to_remove {
    //             self.remove(&Coordinate(i, remove.0), &remove.1);
    //         }
    //     }
    //     for i in 0..n {
    //         // The values of the cells in column i (indexed by row);
    //         let column: Vec<&HashSet<u8>> = self.grid.iter().map(|x| &x[i]).collect();
    //         let to_remove = Solver::row_group_solve(&column);
    //         for remove in to_remove {
    //             self.remove(&Coordinate(remove.0, i), &remove.1);
    //         }
    //     }
    //     for i in 0..n {
    //         // The columns that contain value i (indexed by row);
    //         let value: Vec<HashSet<u8>> = self.grid.iter().map(|x| columns_that_have_value(x, &(i as u8))).collect();
    //         let value_ref: Vec<&HashSet<u8>> = value.iter().collect();
    //         let to_remove = Solver::row_group_solve(&value_ref);
    //         for remove in to_remove {
    //             self.remove(&Coordinate(remove.0, remove.1 as usize), &(i as u8));
    //         }
    //     }
    // }

    // If k cells in a single row only have k possibilities among them, then those k possibilities
    // must be within those cells.
    // Looking at all k-subsets of the row is potentially O(2^n).
    // But there are some short cuts we can take to bring it down to O(?) TODO: figure out
    // 1. Can skip the 0-subset and the n-subset (since they are trivial).
    // 2. Can skip 1-subsets and  (n-1)-subsets (simple_solve takes care of them).
    // 3. If a cell has x or more possibilities, then we don't have to consider k-subests that
    //    contain that cell if k < x.
    // 3. TODO: figure out
    // fn row_group_solve(row: &Vec<&HashSet<u8>>) -> Vec<(usize, u8)> {
    //     let n = row.len();
    //     let mut to_remove: Vec<(usize, u8)> = Vec::new();
    //     for k in 2..(n-1) {
    //         // Find all k-subsets with at most k different potential values.
    //         let mut counter: Vec<usize> = Vec::new();
    //         let mut possibilities: Vec<HashSet<u8>> = Vec::new();
    //         for i in 0..k {
    //             counter.push(i);
    //         }
    //         let mut reached_end = false;
    //         while !reached_end {
    //             while possibilities.len() < k {
    //                 let empty_set = HashSet::new();
    //                 let current_set: &HashSet<u8> = match possibilities.last() {
    //                     Some(x) => x,
    //                     None => &empty_set
    //                 };
    //                 let next_column = counter[possibilities.len()];
    //                 let next_set: &HashSet<u8> = row[next_column];
    //                 // We can skip solved cells to save ourselves a bit of time.
    //                 let union_set: HashSet<u8> = match next_set.len() {
    //                     1 => HashSet::new(),
    //                     _ => current_set.union(next_set).map(|x| *x).collect(),
    //                 };
    //                 if next_set.len() > 1 && union_set.len() <= k {
    //                     possibilities.push(union_set);
    //                 } else {
    //                     let to_remove = increase_counter(& mut counter, possibilities.len(), n);
    //                     for _i in 0..to_remove {
    //                         match possibilities.pop() {
    //                             Some(_x) => {
    //                                 // Do nothing
    //                             },
    //                             None => {
    //                                 reached_end = true;
    //                                 break;
    //                             }
    //                         };
    //                     }
    //                     if reached_end {
    //                         break;
    //                     }
    //                 }
    //             }
    //             if !reached_end {
    //                 // Only the cells in the counter can hosts the values in the last entry of
    //                 // possibilities.
    //                 for index in 0..n {
    //                     if !counter.contains(&index) {
    //                         for value in possibilities.last().unwrap() {
    //                           to_remove.push((index, *value));
    //                       }
    //                     }
    //                 }
    //                 possibilities.pop();
    //                 let backtrack = increase_counter(& mut counter, possibilities.len(), n);
    //                 for _i in 0..backtrack {
    //                     match possibilities.pop() {
    //                         Some(_x) => {
    //                             // Do nothing
    //                         },
    //                         None => {
    //                             reached_end = true;
    //                             break;
    //                         }
    //                     };
    //                 }
    //             }
    //         }
    //
    //     }
    //     return to_remove;
    // }

    // pub fn graph_solve(&mut self) -> Vec<(HashSet<Possibility>, Vec<Vec<HashSet<u8>>>)> {
    //     let mut g: graph_solver::Graph = graph_solver::Graph::new(&self.grid);
    //     let to_remove = g.find_impossibilities();
    //     for remove in to_remove {
    //         self.remove(&Coordinate(remove.0 as usize, remove.1 as usize), &remove.2);
    //     }
    //     // Return maximal implied grids incase we want to do row analysis on them.
    //     return g.maximal_implied_grids();
    // }
}

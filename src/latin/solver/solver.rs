use super::super::puzzle::Puzzle;
use super::triple::*;

use std::time::Instant;
use std::collections::HashSet;
use std::collections::HashMap;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Status {
    // No solution exist
    Unsolvable,
    // Only one solution exists
    UniqueSolution,
    // Many solutions exist
    MultipleSolutions,
    // Don't know if solvable or not yet
    InProgress,
}

fn possibilities_to_detailed_string(p: &HashSet<u8>, size: usize) -> String {
    let mut result = String::with_capacity(size);
    for i in 0..size {
        if p.contains(&(i as u8)) {
            result.push_str(&(i+1).to_string());
        } else {
            result.push_str("_");
        }
    }
    return result;
}

#[derive(Clone, Debug)]
pub struct Solver {
    pub puzzle: Puzzle,
    pub all_triples: HashSet<Triple>,
    pub row_col_map: HashMap<RowCol, HashSet<u8>>,
    pub row_val_map: HashMap<RowVal, HashSet<u8>>,
    pub col_val_map: HashMap<ColVal, HashSet<u8>>,
    pub change_flag: bool,
    pub status: Status,
    pub depth_needed: u8,
}

impl Solver {
    pub fn to_string(&self) -> String {
      let n = self.puzzle.size as u8;
      let mut rows: Vec<String> = Vec::new();

      for row in 0..n {
          let mut row_strings: Vec<String> = Vec::new();
          for col in 0..n {
              let c = RowCol {row, col};
              let p = self.row_col_map.get(&c).unwrap();
              row_strings.push(possibilities_to_detailed_string(p, n as usize));
          }
          rows.push(row_strings.join(" "));
      }

      // Join and return rows.
      return rows.join("\n");
    }

    pub fn new(p: Puzzle) -> Solver {
        let n = p.size;
        let mut all_triples: HashSet<Triple> = HashSet::new();
        let mut row_col_map: HashMap<RowCol, HashSet<u8>> = HashMap::new();
        let mut row_val_map: HashMap<RowVal, HashSet<u8>> = HashMap::new();
        let mut col_val_map: HashMap<ColVal, HashSet<u8>> = HashMap::new();

        for i in 0..n as u8 {
            for j in 0..n as u8 {
                row_col_map.insert(RowCol{row: i, col: j}, HashSet::new());
                row_val_map.insert(RowVal{row: i, val: j}, HashSet::new());
                col_val_map.insert(ColVal{col: i, val: j}, HashSet::new());
            }
        }

        for i in 0..n {
            for j in 0..n {
                match p.grid[i][j] {
                    Some(x) => {
                        let t = Triple{row: i as u8, col: j as u8, val: x};
                        all_triples.insert(t);
                        row_col_map.get_mut(&t.get_row_col()).unwrap().insert(t.val);
                        row_val_map.get_mut(&t.get_row_val()).unwrap().insert(t.col);
                        col_val_map.get_mut(&t.get_col_val()).unwrap().insert(t.row);
                    },
                    None => {
                        for k in 0..n {
                            let t = Triple{row: i as u8, col: j as u8, val: k as u8};
                            all_triples.insert(t);
                            row_col_map.get_mut(&t.get_row_col()).unwrap().insert(t.val);
                            row_val_map.get_mut(&t.get_row_val()).unwrap().insert(t.col);
                            col_val_map.get_mut(&t.get_col_val()).unwrap().insert(t.row);
                        }
                    },
                };
            }
        }

        return Solver {
            puzzle: p,
            all_triples,
            row_col_map,
            row_val_map,
            col_val_map,
            change_flag: false,
            status: Status::InProgress,
            depth_needed: 0,
        }
    }

    pub fn remove(&mut self, t: &Triple) {
        if self.all_triples.remove(&t) {
            let mut set;

            set = self.row_col_map.get_mut(&t.get_row_col()).unwrap();
            set.remove(&t.val);
            if set.is_empty() {
                self.status = Status::Unsolvable;
            }

            set = self.row_val_map.get_mut(&t.get_row_val()).unwrap();
            set.remove(&t.col);
            if set.is_empty() {
                self.status = Status::Unsolvable;
            }

            set = self.col_val_map.get_mut(&t.get_col_val()).unwrap();
            set.remove(&t.row);
            if set.is_empty() {
                self.status = Status::Unsolvable;
            }

            let n = self.puzzle.size;
            if self.status == Status::InProgress && self.all_triples.len() == n * n {
                self.status = Status::UniqueSolution;
            }
        }
    }

    pub fn set(&mut self, t: &Triple) {
        let n = self.puzzle.size as u8;
        for i in 0..n {
            if t.val != i {
                self.remove(&Triple{row: t.row, col: t.col, val: i});
            }
            if t.col != i {
                self.remove(&Triple{row: t.row, col: i, val: t.val});
            }
            if t.row != i {
                self.remove(&Triple{row: i, col: t.col, val: t.val});
            }
        }
    }

    // Solve the puzzle using all non-recursive ways we know of.
    pub fn non_recursive_solve(&mut self) {
        self.change_flag = true;
        while self.change_flag && self.status == Status::InProgress {
            self.change_flag = false;
            // println!("{}\n", self.to_string());
        }
    }

    pub fn full_solve(&mut self, depth: u8, should_log: bool) -> Vec<Solver> {
        self.depth_needed = depth;
        let start = Instant::now();
        let mut solutions: Vec<Solver> = Vec::new();

        self.non_recursive_solve();
        // println!("After non-recursive solve:\n{}\n", self.to_string());
        if self.status == Status::InProgress {
            solutions = self.depth_solve(depth, should_log);
        } else if self.status == Status::UniqueSolution {
            solutions.push(self.clone());
        }

        let duration = start.elapsed();
        let indent = " ".repeat(8 * depth as usize);
        if should_log {
            // println!("{}", self.to_string());
            println!("\n{}Done! Total Time: {}.{:>6}", indent, duration.as_secs(), duration.as_micros() % 1000000);
            println!("{}Status: {:?}", indent, self.status);
            println!("{}Depth: {:?}", indent, depth);
            println!("{}Solutions #: {}", indent, solutions.len());
            // println!("{}\n", self.inside_tracker.to_string());
        }

        return solutions;
    }
}

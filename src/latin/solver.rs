pub mod triple;
mod depth_solver;
mod implication_solver;

use super::puzzle::Puzzle;
use triple::*;
use implication_solver::*;

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
    pub all_set: HashSet<Triple>,
    pub row_col_map: HashMap<RowCol, HashSet<u8>>,
    pub row_val_map: HashMap<RowVal, HashSet<u8>>,
    pub col_val_map: HashMap<ColVal, HashSet<u8>>,

    pub to_set: HashSet<Triple>,
    pub to_remove: HashSet<Triple>,

    pub implication_tracker: ImplicationsTracker,

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
        let mut to_set: HashSet<Triple> = HashSet::new();
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
                        to_set.insert(Triple{row: i as u8, col: j as u8, val: x});
                    },
                    None => {},
                }
                for k in 0..n {
                    let t = Triple{row: i as u8, col: j as u8, val: k as u8};
                    all_triples.insert(t);
                    row_col_map.get_mut(&t.get_row_col()).unwrap().insert(t.val);
                    row_val_map.get_mut(&t.get_row_val()).unwrap().insert(t.col);
                    col_val_map.get_mut(&t.get_col_val()).unwrap().insert(t.row);
                }
            }
        }

        return Solver {
            puzzle: p,
            all_triples,
            all_set: HashSet::new(),
            row_col_map,
            row_val_map,
            col_val_map,
            to_set,
            to_remove: HashSet::new(),
            implication_tracker: ImplicationsTracker::new(n as u8),
            status: Status::InProgress,
            depth_needed: 0,
        }
    }

    pub fn remove(&mut self, t: &Triple) {
        if self.all_triples.remove(&t) {
            let mut set;

            set = self.row_col_map.get_mut(&t.get_row_col()).unwrap();
            set.remove(&t.val);
            match set.len() {
                0 => { self.status = Status::Unsolvable; },
                1 => {
                    let val = set.iter().next().unwrap().clone();
                    self.to_set.insert(t.with_val(val));
                },
                _ => {},
            }

            set = self.row_val_map.get_mut(&t.get_row_val()).unwrap();
            set.remove(&t.col);
            match set.len() {
                0 => { self.status = Status::Unsolvable; },
                1 => {
                    let col = set.iter().next().unwrap().clone();
                    self.to_set.insert(t.with_col(col));
                },
                _ => {},
            }

            set = self.col_val_map.get_mut(&t.get_col_val()).unwrap();
            set.remove(&t.row);
            match set.len() {
                0 => { self.status = Status::Unsolvable; },
                1 => {
                    let row = set.iter().next().unwrap().clone();
                    self.to_set.insert(t.with_row(row));
                },
                _ => {},
            }

            let n = self.puzzle.size;
            if self.status == Status::InProgress && self.all_triples.len() == n * n {
                self.status = Status::UniqueSolution;
            }
        }
    }

    pub fn remove_conflict_with_set(&mut self, t: &Triple) {
        let n = self.puzzle.size as u8;
        for i in 0..n {
            if t.val != i {
                let o = Triple{row: t.row, col: t.col, val: i};
                if self.all_triples.contains(&o) {
                    self.to_remove.insert(o);
                }
            }
            if t.col != i {
                let o = Triple{row: t.row, col: i, val: t.val};
                if self.all_triples.contains(&o) {
                    self.to_remove.insert(o);
                }
            }
            if t.row != i {
                let o = Triple{row: i, col: t.col, val: t.val};
                if self.all_triples.contains(&o) {
                    self.to_remove.insert(o);
                }
            }
        }
    }

    fn process_to_set(&mut self) {
        let to_set:HashSet<Triple> = self.to_set.drain().collect();
        self.all_set = &self.all_set | &to_set;
        for t in to_set {
            let result = self.implication_tracker.set_triple(&t);
            self.sort_binary_triples(result);
            self.remove_conflict_with_set(&t);
        }
    }

    fn process_to_remove(&mut self) {
        let to_remove: HashSet<Triple> = self.to_remove.drain().collect();
        for t in to_remove {
            let result = self.implication_tracker.remove_triple(&t);
            self.sort_binary_triples(result);
            self.remove(&t);
        }
    }

    fn sort_binary_triples(&mut self, s: HashSet<BinaryTriple>) {
        for t in s {
            match t.negated {
                false => {
                    if !self.all_set.contains(&t.t) {
                        self.to_set.insert(t.t.clone());
                    }
                },
                true => {
                    if self.all_triples.contains(&t.t) {
                        self.to_remove.insert(t.t.clone()); 
                    }
                },
            };
        }
    }

    // Solve the puzzle using all non-recursive ways we know of.
    pub fn non_recursive_solve(&mut self) {
        while (!self.to_set.is_empty() || !self.to_remove.is_empty()) && self.status == Status::InProgress {
            self.process_to_set();
            self.process_to_remove();

            if self.to_set.is_empty() && self.to_remove.is_empty() {
                self.implication_tracker.hypothetical_syllogism();
                self.implication_tracker.disjunctive_syllogism();
                self.implication_tracker.hypothetical_syllogism();

                for t in self.implication_tracker.get_contradictions() {
                    if t.negated {
                        if !self.all_set.contains(&t.t) {
                            self.to_set.insert(t.t.clone());
                        }
                    } else {
                        if self.all_triples.contains(&t.t) {
                            self.to_remove.insert(t.t.clone());
                        }
                    }
                }
                if self.to_set.is_empty() && self.to_remove.is_empty() {
                    let result = self.implication_tracker.get_disjunction_elimination_inferences(self.puzzle.size as u8);
                    self.sort_binary_triples(result);
                }
            }

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
        if should_log {
            let indent = " ".repeat(8 * depth as usize);
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

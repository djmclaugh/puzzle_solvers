use super::puzzle::Puzzle;
use super::puzzle::row;
use super::puzzle::column;
use super::puzzle::calculate_view;
use std::collections::HashSet;
mod row_solver;
mod latin_solver;
mod depth_solver;

use std::time::Instant;

// (row, column)
#[derive(Clone)]
#[derive(Debug)]
#[derive(Copy)]
pub struct Coordinate (usize, usize);

#[derive(Debug)]
enum Direction {
    NORTH,
    EAST,
    SOUTH,
    WEST,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Status {
    // No solution exists
    Unsolvable,
    // No solution exists
    UniqueSolution,
    // No solution exists
    MultipleSolutions,
    // Don't know if solvable or not yet
    InProgress,
}

fn view_to_string(view: &Option<u8>) -> String {
    match view {
        Some(x) => x.to_string(),
        None => String::from("?")
    }
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

pub struct Solver {
    puzzle: Puzzle,
    grid: Vec<Vec<HashSet<u8>>>,
    solved_count: usize,
    recently_solved: Vec<Coordinate>,
    value_count_by_row: Vec<Vec<u8>>,
    value_count_by_column: Vec<Vec<u8>>,
    recently_unique_in_row: Vec<(usize, u8)>,
    recently_unique_in_column: Vec<(usize, u8)>,
    change_flag: bool,
    status: Status,
    pub depth_needed: u8,
}

fn get_vec<'a>(grid: &'a Vec<Vec<HashSet<u8>>>, d: &Direction, i: usize) -> Vec<&'a HashSet<u8>> {
    let mut vec = match d {
        Direction::NORTH | Direction::SOUTH => column(grid, i),
        Direction::EAST | Direction::WEST => row(grid, i),
    };

    match d {
        Direction::EAST | Direction::SOUTH => { vec.reverse(); },
        Direction::NORTH | Direction::WEST => {},
    }

    return vec;
}

// i1 = the row/column chosen
// i2 = how far in the row/column
fn get_coordinate<'a>(d: &Direction, n: usize, i1: usize, i2: usize) -> Coordinate {
    let mut c = Coordinate(i1, i2);
    // If we're starting from the end of the row columns, then we need to do n - that index instead.
    match d {
        Direction::EAST | Direction::SOUTH => { c.1 = n - c.1 - 1; },
        Direction::NORTH | Direction::WEST => {},
    }
    // If we're looking from the north or south, then i1 is actually the column and i2 is the row.
    match d {
        Direction::NORTH | Direction::SOUTH => {
            let temp = c.1;
            c.1 = c.0;
            c.0 = temp;
        },
        Direction::EAST | Direction::WEST => {},
    };
    return c;
}

impl Solver {
    pub fn to_detailed_string(&self) -> String {
      let n = self.puzzle.latin.size;
      let mut rows: Vec<String> = Vec::new();
      // North hints
      let mut row: Vec<String> = Vec::new();
      row.push(String::from(" "));
      let hints: Vec<String> = self.puzzle.north.iter().map(view_to_string).collect();
      row.extend(hints);
      row.push(String::from(" "));

      rows.push(row.join(&(" ".repeat(n))));

      // Middle Rows
      for i in 0..n {
          row = Vec::new();
          row.push(view_to_string(&self.puzzle.west[i]));
          let content: Vec<String> = self.grid[i].iter().map(|x| possibilities_to_detailed_string(x, n)).collect();
          row.extend(content);
          row.push(view_to_string(&self.puzzle.east[i]));
          rows.push(row.join(" "));
      }

      // South hints
      row = Vec::new();
      row.push(String::from(" "));
      let hints: Vec<String> = self.puzzle.south.iter().map(view_to_string).collect();
      row.extend(hints);
      row.push(String::from(" "));

      rows.push(row.join(&(" ".repeat(n))));

      // Join and return rows.
      return rows.join("\n");
    }

    pub fn new(p: Puzzle) -> Solver {
        let n = p.latin.size;
        let mut grid: Vec<Vec<HashSet<u8>>> = Vec::new();
        let mut recently_solved: Vec<Coordinate> = Vec::new();
        let mut value_count_by_row = Vec::new();
        let mut value_count_by_column = Vec::new();
        let mut recently_unique_in_row: Vec<(usize, u8)> = Vec::new();
        let mut recently_unique_in_column: Vec<(usize, u8)> = Vec::new();

        for i in 0..n {
            value_count_by_row.push(Vec::new());
            value_count_by_column.push(Vec::new());
            grid.push(Vec::new());
            for _j in 0..n {
                value_count_by_row[i].push(0);
                value_count_by_column[i].push(0);
                grid[i].push(HashSet::new());
            }
        }

        for row in 0..n {
            for column in 0..n {
                match p.latin.grid[row][column] {
                    Some(x) => {
                        grid[row][column].insert(x);
                        value_count_by_row[row][x as usize] += 1;
                        value_count_by_column[column][x as usize] += 1;
                        recently_solved.push(Coordinate(row, column));
                    },
                    None => {
                        for i in 0..(n as u8) {
                            grid[row][column].insert(i);
                            value_count_by_row[row][i as usize] += 1;
                            value_count_by_column[column][i as usize] += 1;
                        }
                    }
                }
            }
        }

        for i in 0..n {
            for j in 0..n {
                if value_count_by_row[i][j] == 1 {
                    recently_unique_in_row.push((i, j as u8));
                }
                if value_count_by_column[i][j] == 1 {
                    recently_unique_in_column.push((i, j as u8));
                }
            }
        }

        return Solver {
            puzzle: p,
            grid,
            solved_count: recently_solved.len(),
            recently_solved,
            value_count_by_row,
            value_count_by_column,
            recently_unique_in_row,
            recently_unique_in_column,
            change_flag: false,
            status: Status::InProgress,
            depth_needed: 0,
        }
    }

    fn clone (&self) -> Solver {
        return Solver {
            puzzle: self.puzzle.clone(),
            grid: self.grid.clone(),
            solved_count: self.solved_count,
            recently_solved: self.recently_solved.clone(),
            value_count_by_row: self.value_count_by_row.clone(),
            value_count_by_column: self.value_count_by_column.clone(),
            recently_unique_in_row: self.recently_unique_in_row.clone(),
            recently_unique_in_column: self.recently_unique_in_column.clone(),
            change_flag: self.change_flag,
            status: self.status,
            depth_needed: self.depth_needed,
        }
    }

    fn remove(& mut self, c: &Coordinate, value: &u8) {
        let n = self.puzzle.latin.size;
        let set = self.grid[c.0].get_mut(c.1).unwrap();
        let has_removed = set.remove(value);
        if has_removed {
            self.value_count_by_row[c.0][*value as usize] -= 1;
            self.value_count_by_column[c.1][*value as usize] -= 1;
            if set.len() == 1 {
                self.solved_count += 1;
                self.recently_solved.push(c.clone());
            }
            if self.value_count_by_row[c.0][*value as usize] == 1 {
                self.recently_unique_in_row.push((c.0, *value));
            }
            if self.value_count_by_column[c.1][*value as usize] == 1 {
                self.recently_unique_in_column.push((c.1, *value));
            }
            self.change_flag = true;
            if set.len() == 0 {
                self.status = Status::Unsolvable;
            }
        }
        if has_removed && self.solved_count == n * n {
            if self.satisfies_contraints() {
                self.status = Status::UniqueSolution;
            } else {
                self.status = Status::Unsolvable;
            }
        }
    }

    fn set(& mut self, c: &Coordinate, value: &u8) -> bool {
        for i in 0..self.puzzle.latin.size {
            let u = i as u8;
            if u != *value {
                self.remove(c, &u);
            }
        }
        return true;
    }

    fn satisfies_contraints(&self) -> bool {
        let n = self.puzzle.latin.size;
        // Check if each element in each row is unique.
        for row in 0..n {
            let mut seen: HashSet<u8> = HashSet::new();
            for column in 0..n {
                if self.grid[row][column].len() != 1 {
                    return false;
                } else {
                    seen.insert(*self.grid[row][column].iter().next().unwrap());
                }
            }
            if seen.len() != n {
                return false;
            }
        }
        // Check if each element in each column is unique.
        for column in 0..n {
            let mut seen: HashSet<u8> = HashSet::new();
            for row in 0..n {
                if self.grid[row][column].len() != 1 {
                    return false;
                } else {
                    seen.insert(*self.grid[row][column].iter().next().unwrap());
                }
            }
            if seen.len() != n {
                return false;
            }
        }
        // Check if each view is respected
        for d in [Direction::NORTH, Direction::EAST, Direction::SOUTH, Direction::WEST] {
            let views: &Vec<Option<u8>> = match d {
                Direction::NORTH => &self.puzzle.north,
                Direction::EAST => &self.puzzle.east,
                Direction::SOUTH => &self.puzzle.south,
                Direction::WEST => &self.puzzle.west,
            };
            for index in 0..n {
                if views[index].is_none() {
                    continue;
                }
                let view = views[index].unwrap();
                let values: Vec<&u8> = get_vec(&self.grid, &d, index).iter().map(|x| x.iter().next().unwrap()).collect();
                if calculate_view(&values) != view {
                    return false;
                }
            }
        }
        return true;
    }

    fn view_solve(& mut self) {
        for i in 0..self.puzzle.latin.size {
            for d in [Direction::NORTH, Direction::EAST, Direction::SOUTH, Direction::WEST] {
                if self.status == Status::Unsolvable {
                    return;
                }
                let still_potentially_solvable = self.analyze_view(d, i);
                if !still_potentially_solvable {
                    self.status = Status::Unsolvable;
                    return;
                }
            }
        }
    }

    // fn view_solve_with_grid(& mut self, grid: &Vec<Vec<HashSet<u8>>>) -> bool {
    //     for i in 0..self.puzzle.size {
    //         for d in [Direction::NORTH, Direction::EAST, Direction::SOUTH, Direction::WEST] {
    //             let still_potentially_solvable = self.analyze_view_with_grid(d, i, grid);
    //             if !still_potentially_solvable {
    //                 return false;
    //             }
    //         }
    //     }
    //     return true;
    // }

    // fn brute_force_view_solve(& mut self) {
    //     for i in 0..self.puzzle.size {
    //         // println!("Brute force: {} of {}", i + 1, self.puzzle.size);
    //         for d in [Direction::NORTH, Direction::EAST, Direction::SOUTH, Direction::WEST] {
    //             let still_potentially_solvable = self.brute_force_view(d, i);
    //             if !still_potentially_solvable {
    //                 self.status = Status::Unsolvable;
    //                 return;
    //             }
    //         }
    //     }
    // }

    fn analyze_view(& mut self, from: Direction, index: usize) -> bool {
        let n = self.puzzle.latin.size;
        let view:Option<u8> = match from {
            Direction::NORTH => self.puzzle.north[index],
            Direction::EAST => self.puzzle.east[index],
            Direction::SOUTH => self.puzzle.south[index],
            Direction::WEST => self.puzzle.west[index],
        };
        if view.is_none() {
            return true;
        }

        let (still_potentially_solvable, to_remove) = row_solver::solve(view.unwrap(), &get_vec(&self.grid, &from, index));

        for i in to_remove {
            let c = get_coordinate(&from, n, index, i.0);
            self.remove(&c, &i.1);
        }

        return still_potentially_solvable;
    }

    // fn analyze_view_with_grid(& mut self, from: Direction, index: usize, grid: &Vec<Vec<HashSet<u8>>>) -> bool {
    //     let view:Option<u8> = match from {
    //         Direction::NORTH => self.puzzle.north[index],
    //         Direction::EAST => self.puzzle.east[index],
    //         Direction::SOUTH => self.puzzle.south[index],
    //         Direction::WEST => self.puzzle.west[index],
    //     };
    //     if view.is_none() {
    //         return true;
    //     }
    //
    //     let (still_potentially_solvable, _to_remove) = row_solver::solve(view.unwrap(), &get_vec(grid, &from, index));
    //
    //     return still_potentially_solvable;
    // }

    // fn brute_force_view(& mut self, from: Direction, index: usize) -> bool {
    //     let n = self.puzzle.size;
    //     let view: Option<u8> = match from {
    //         Direction::NORTH => self.puzzle.north[index],
    //         Direction::EAST => self.puzzle.east[index],
    //         Direction::SOUTH => self.puzzle.south[index],
    //         Direction::WEST => self.puzzle.west[index],
    //     };
    //     if view.is_none() {
    //         return true;
    //     }
    //
    //     let (still_potentially_solvable, to_remove) = row_solver::trial_solve(view.unwrap(), &get_vec(&self.grid, &from, index));
    //
    //     for i in to_remove {
    //         let c = get_coordinate(&from, n, index, i.0);
    //         self.remove(&c, &i.1);
    //     }
    //
    //     return still_potentially_solvable;
    // }

    // Solve the puzzle using all non-recursive ways we know of.
    pub fn non_recursive_solve(&mut self) {
        self.initial_view_solve();

        self.change_flag = true;

        while self.change_flag && self.status == Status::InProgress {
            self.change_flag = false;
            self.simple_solve();
            self.view_solve();
        }
    }

    pub fn full_solve(&mut self, depth: u8, should_log: bool) -> Vec<Solver> {
        self.depth_needed = depth;
        // let start = Instant::now();
        let mut solutions: Vec<Solver> = Vec::new();

        self.change_flag = true;
        while self.change_flag {
            self.non_recursive_solve();
            self.change_flag = false;
            if self.status == Status::InProgress {
                solutions = self.depth_solve(depth, should_log);
            } else {
                solutions = Vec::new();
                solutions.push(self.clone());
            }
        }

        // let duration = start.elapsed();
        // let indent = " ".repeat((8 * depth) as usize);
        // if should_log {
            // println!("\n{}Done! Total Time: {}.{:>6}", indent, duration.as_secs(), duration.as_micros() % 1000000);
            // println!("{}Status: {:?}", indent, self.status);
            // println!("{}Depth: {:?}", indent, depth);
        // }

        return solutions;
    }

    fn initial_view_solve(&mut self) {
        let n = self.puzzle.latin.size;

        let mut north_hints = Vec::new();
        for i in 0..n {
            match self.puzzle.north[i] {
                Some(x) => { north_hints.push((i, x)); },
                None => {
                    // Do nothing
                }
            }
        }
        for (column, view) in north_hints {
            if view == 1 {
                self.set(&Coordinate(0, column), &((n-1) as u8));
            } else {
                for row in 0..n {
                    for value in (1 + ((n + row) as u8) - view)..(n as u8) {
                        self.remove(&Coordinate(row, column), &value);
                    }
                }
            }
        }

        let mut east_hints = Vec::new();
        for i in 0..n {
            match self.puzzle.east[i] {
                Some(x) => { east_hints.push((i, x)); },
                None => {
                    // Do nothing
                }
            }
        }
        for (row, view) in east_hints {
            if view == 1 {
                self.set(&Coordinate(row, n - 1), &((n-1) as u8));
            } else {
                for i in 0..n {
                    let column = n - i - 1;
                    for value in (1 + ((n + i) as u8) - view)..(n as u8) {
                        self.remove(&Coordinate(row, column), &value);
                    }
                }
            }
        }

        let mut south_hints = Vec::new();
        for i in 0..n {
            match self.puzzle.south[i] {
                Some(x) => { south_hints.push((i, x)); },
                None => {
                    // Do nothing
                }
            }
        }
        for (column, view) in south_hints {
            if view == 1 {
                self.set(&Coordinate(n - 1, column), &((n-1) as u8));
            } else if view != 0 {
                for i in 0..n {
                    let row = n - i - 1;
                    for value in (1 + ((n + i) as u8) - view)..(n as u8) {
                        self.remove(&Coordinate(row, column), &value);
                    }
                }
            }
        }

        let mut west_hints = Vec::new();
        for i in 0..n {
            match self.puzzle.west[i] {
                Some(x) => { west_hints.push((i, x)); },
                None => {
                    // Do nothing
                }
            }
        }
        for (row, view) in west_hints {
            if view == 1 {
                self.set(&Coordinate(row, 0), &((n-1) as u8));
            } else {
                for column in 0..n {
                    for value in (1 + ((n + column) as u8) - view)..(n as u8) {
                        self.remove(&Coordinate(row, column), &value);
                    }
                }
            }
        }
    }
}

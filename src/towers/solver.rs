use super::puzzle::Puzzle;
use std::collections::HashSet;
mod row_solver;
mod latin_solver;

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
    // One or multiple solutions exist
    Solved,
    // No solution exists
    Unsolvable,
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

pub struct Solver<'a> {
    puzzle: &'a Puzzle,
    grid: Vec<Vec<HashSet<u8>>>,
    solved_count: usize,
    recently_solved: Vec<Coordinate>,
    value_count_by_row: Vec<Vec<u8>>,
    value_count_by_column: Vec<Vec<u8>>,
    recently_unique_in_row: Vec<(usize, u8)>,
    recently_unique_in_column: Vec<(usize, u8)>,
    change_flag: bool,
    status: Status,
}

pub fn row(grid: &Vec<Vec<HashSet<u8>>>, index: usize) -> Vec<&HashSet<u8>> {
    return grid[index].iter().map(|x| x).collect();
}

pub fn column(grid: &Vec<Vec<HashSet<u8>>>, index: usize) -> Vec<&HashSet<u8>> {
    return grid.iter().map(|x| &x[index]).collect();
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

impl<'a> Solver<'a> {
    pub fn to_detailed_string(&self) -> String {
      let n = self.puzzle.size;
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

    pub fn new(p: &Puzzle) -> Solver {
        let n = p.size;
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
                match p.grid[row][column] {
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
        }
    }

    fn clone (&self) -> Solver {
        return Solver {
            puzzle: self.puzzle,
            grid: self.grid.clone(),
            solved_count: self.solved_count,
            recently_solved: self.recently_solved.clone(),
            value_count_by_row: self.value_count_by_row.clone(),
            value_count_by_column: self.value_count_by_column.clone(),
            recently_unique_in_row: self.recently_unique_in_row.clone(),
            recently_unique_in_column: self.recently_unique_in_column.clone(),
            change_flag: self.change_flag,
            status: self.status,
        }
    }

    fn remove(& mut self, c: &Coordinate, value: &u8) {
        let set = self.grid[c.0].get_mut(c.1).unwrap();
        if set.remove(value) {
            self.value_count_by_row[c.0][*value as usize] -= 1;
            self.value_count_by_column[c.1][*value as usize] -= 1;
            if set.len() == 1 {
                self.solved_count += 1;
                if self.solved_count == self.puzzle.size * self.puzzle.size {
                    self.status = Status::Solved;
                }
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
    }

    fn set(& mut self, c: &Coordinate, value: &u8) -> bool {
        for i in 0..self.puzzle.size {
            let u = i as u8;
            if u != *value {
                self.remove(c, &u);
            }
        }
        return true;
    }

    fn view_solve(& mut self) {
        for i in 0..self.puzzle.size {
            for d in [Direction::NORTH, Direction::EAST, Direction::SOUTH, Direction::WEST] {
                let still_potentially_solvable = self.analyze_view(d, i);
                if !still_potentially_solvable {
                    self.status = Status::Unsolvable;
                    return;
                }
            }
        }
    }

    fn view_solve_with_grid(& mut self, grid: &Vec<Vec<HashSet<u8>>>) -> bool {
        for i in 0..self.puzzle.size {
            for d in [Direction::NORTH, Direction::EAST, Direction::SOUTH, Direction::WEST] {
                let still_potentially_solvable = self.analyze_view_with_grid(d, i, grid);
                if !still_potentially_solvable {
                    return false;
                }
            }
        }
        return true;
    }

    fn brute_force_view_solve(& mut self) {
        for i in 0..self.puzzle.size {
            // println!("Brute force: {} of {}", i + 1, self.puzzle.size);
            for d in [Direction::NORTH, Direction::EAST, Direction::SOUTH, Direction::WEST] {
                let still_potentially_solvable = self.brute_force_view(d, i);
                if !still_potentially_solvable {
                    self.status = Status::Unsolvable;
                    return;
                }
            }
        }
    }

    fn analyze_view(& mut self, from: Direction, index: usize) -> bool {
        let n = self.puzzle.size;
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

    fn analyze_view_with_grid(& mut self, from: Direction, index: usize, grid: &Vec<Vec<HashSet<u8>>>) -> bool {
        let view:Option<u8> = match from {
            Direction::NORTH => self.puzzle.north[index],
            Direction::EAST => self.puzzle.east[index],
            Direction::SOUTH => self.puzzle.south[index],
            Direction::WEST => self.puzzle.west[index],
        };
        if view.is_none() {
            return true;
        }

        let (still_potentially_solvable, _to_remove) = row_solver::solve(view.unwrap(), &get_vec(grid, &from, index));

        return still_potentially_solvable;
    }

    fn brute_force_view(& mut self, from: Direction, index: usize) -> bool {
        let n = self.puzzle.size;
        let view: Option<u8> = match from {
            Direction::NORTH => self.puzzle.north[index],
            Direction::EAST => self.puzzle.east[index],
            Direction::SOUTH => self.puzzle.south[index],
            Direction::WEST => self.puzzle.west[index],
        };
        if view.is_none() {
            return true;
        }

        let (still_potentially_solvable, to_remove) = row_solver::trial_solve(view.unwrap(), &get_vec(&self.grid, &from, index));

        for i in to_remove {
            let c = get_coordinate(&from, n, index, i.0);
            self.remove(&c, &i.1);
        }

        return still_potentially_solvable;
    }

    // Solve the puzzle using all non-recursive ways we know of.
    pub fn non_recursive_solve(&mut self) {
        self.initial_view_solve();

        self.change_flag = true;

        while self.change_flag && self.status == Status::InProgress {
            self.change_flag = false;
            self.simple_solve();
            self.view_solve();

            // I don't think grouping solve is exponential, but I haven't proven it yet, so I'm going
            // to treat it as exponential and only use it if I can't make progress otherwise.
            if !self.change_flag && self.status == Status::InProgress {
                self.grouping_solve();
            }

            // I don't think graph solve is exponential, but I haven't proven it yet, so I'm going to
            // treat it as exponential and only use it if I can't make progress otherwise.
            if !self.change_flag && self.status == Status::InProgress {
                let now = Instant::now();
                let maximal_graphs = self.graph_solve();
                println!("Graph solve: {:>8}", now.elapsed().as_micros());
                if !self.change_flag {
                    let now = Instant::now();
                    for (class, grid) in maximal_graphs {
                        if !self.view_solve_with_grid(&grid) {
                            for p in class {
                                self.remove(&Coordinate(p.0 as usize, p.1 as usize), &p.2);
                            }
                        }
                    }
                    println!("Cross solve: {:>8}", now.elapsed().as_micros());
                }
            }

            // Since brute force solving the view is potentially exponential, only do it if we
            // can't make progress with a more efficient method.
            if !self.change_flag && self.status == Status::InProgress {
                let now = Instant::now();
                self.brute_force_view_solve();
                println!("Force views: {:>8}", now.elapsed().as_micros());
            }

            // Uncomment to see the progress of the solve at each step.
            // println!("{}\n", self.to_detailed_string());
        }
    }

    pub fn full_solve(&mut self) {
        let start = Instant::now();

        self.non_recursive_solve();

        let duration = start.elapsed();
        println!("\nDone! Total Time: {:>8}", duration.as_micros());

        println!("\nStatus: {:?}", self.status);
        if duration.as_secs() >= 1 {
            println!("X Took more than a second.");
        } else {
            println!("Under a second!");
        }
    }

    fn initial_view_solve(&mut self) {
        let n = self.puzzle.size;

        let north_hints = self.puzzle.north.iter()
                .enumerate()
                .filter(|(_column, view)| view.is_some())
                .map(|(column, view)| (column, view.unwrap()));
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

        let east_hints = self.puzzle.east.iter()
                .enumerate()
                .filter(|(_row, view)| view.is_some())
                .map(|(row, view)| (row, view.unwrap()));
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

        let south_hints = self.puzzle.south.iter()
                .enumerate()
                .filter(|(_column, view)| view.is_some())
                .map(|(column, view)| (column, view.unwrap()));
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

        let west_hints = self.puzzle.west.iter()
                .enumerate()
                .filter(|(_row, view)| view.is_some())
                .map(|(row, view)| (row, view.unwrap()));
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

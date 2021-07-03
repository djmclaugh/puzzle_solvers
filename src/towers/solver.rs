use super::puzzle::Puzzle;
use std::collections::HashSet;
mod row_solver;
mod latin_solver;

use std::time::{Instant};

// (row, column)
#[derive(Clone)]
#[derive(Debug)]
#[derive(Copy)]
pub struct Coordinate (usize, usize);

struct Solver<'a> {
    puzzle: &'a mut Puzzle,
    recently_solved: Vec<Coordinate>,
    value_count_by_row: Vec<Vec<u8>>,
    value_count_by_column: Vec<Vec<u8>>,
    recently_unique_in_row: Vec<(usize, u8)>,
    recently_unique_in_column: Vec<(usize, u8)>,
    change_flag: bool,
}

#[derive(Debug)]
enum Direction {
    NORTH,
    EAST,
    SOUTH,
    WEST,
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
    fn new(p: & mut Puzzle) -> Solver {
        let n = p.size;
        let mut recently_solved: Vec<Coordinate> = Vec::new();
        let mut value_count_by_row = Vec::new();
        let mut value_count_by_column = Vec::new();
        let mut recently_unique_in_row: Vec<(usize, u8)> = Vec::new();
        let mut recently_unique_in_column: Vec<(usize, u8)> = Vec::new();

        for i in 0..n {
            value_count_by_row.push(Vec::new());
            value_count_by_column.push(Vec::new());
            for _j in 0..n {
                value_count_by_row[i].push(0);
                value_count_by_column[i].push(0);
            }
        }

        for row in 0..n {
            for column in 0..n {
                if p.grid[row][column].len() == 1 {
                    recently_solved.push(Coordinate(row, column));
                }
                for value in p.grid[row][column].iter() {
                  value_count_by_row[row][*value as usize] += 1;
                  value_count_by_column[column][*value as usize] += 1;
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
            recently_solved,
            value_count_by_row,
            value_count_by_column,
            recently_unique_in_row,
            recently_unique_in_column,
            change_flag: false,
        }
    }

    fn remove(& mut self, c: &Coordinate, value: &u8) -> bool {
        let set = self.puzzle.grid[c.0].get_mut(c.1).unwrap();
        if set.remove(value) {
            self.value_count_by_row[c.0][*value as usize] -= 1;
            self.value_count_by_column[c.1][*value as usize] -= 1;
            if set.len() == 1 {
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
                return false;
            }
        }
        return true;
    }

    fn set(& mut self, c: &Coordinate, value: &u8) -> bool {
        for i in 0..self.puzzle.size {
            let u = i as u8;
            if u != *value {
                if !self.remove(c, &u) {
                    return false;
                }
            }
        }
        return true;
    }

    fn view_solve(& mut self) -> bool {
        for i in 0..self.puzzle.size {
            for d in [Direction::NORTH, Direction::EAST, Direction::SOUTH, Direction::WEST] {
                let still_potentially_solvable = self.analyze_view(d, i);
                if !still_potentially_solvable {
                    return false;
                }
            }
        }
        return true;
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

    fn brute_force_view_solve(& mut self) -> bool {
        for i in 0..self.puzzle.size {
            // println!("Brute force: {} of {}", i + 1, self.puzzle.size);
            for d in [Direction::NORTH, Direction::EAST, Direction::SOUTH, Direction::WEST] {
                let still_potentially_solvable = self.brute_force_view(d, i);
                if !still_potentially_solvable {
                    return false;
                }
            }
        }
        return true;
    }

    fn analyze_view(& mut self, from: Direction, index: usize) -> bool {
        let n = self.puzzle.size;
        let view:u8 = match from {
            Direction::NORTH => self.puzzle.north[index],
            Direction::EAST => self.puzzle.east[index],
            Direction::SOUTH => self.puzzle.south[index],
            Direction::WEST => self.puzzle.west[index],
        };
        if view == 0 {
            return true;
        }

        let (still_potentially_solvable, to_remove) = row_solver::solve(view, &get_vec(&self.puzzle.grid, &from, index));

        for i in to_remove {
            let c = get_coordinate(&from, n, index, i.0);
            if !self.remove(&c, &i.1) {
                println!("{}", self.puzzle.to_detailed_string());
                println!("{:?} {}", c, i.1);
                panic!();
            }
        }

        return still_potentially_solvable;
    }

    fn analyze_view_with_grid(& mut self, from: Direction, index: usize, grid: &Vec<Vec<HashSet<u8>>>) -> bool {
        let view:u8 = match from {
            Direction::NORTH => self.puzzle.north[index],
            Direction::EAST => self.puzzle.east[index],
            Direction::SOUTH => self.puzzle.south[index],
            Direction::WEST => self.puzzle.west[index],
        };
        if view == 0 {
            return true;
        }

        let (still_potentially_solvable, _to_remove) = row_solver::solve(view, &get_vec(grid, &from, index));

        return still_potentially_solvable;
    }

    fn brute_force_view(& mut self, from: Direction, index: usize) -> bool {
        let n = self.puzzle.size;
        let view:u8 = match from {
            Direction::NORTH => self.puzzle.north[index],
            Direction::EAST => self.puzzle.east[index],
            Direction::SOUTH => self.puzzle.south[index],
            Direction::WEST => self.puzzle.west[index],
        };
        if view == 0 {
            return true;
        }

        let (still_potentially_solvable, to_remove) = row_solver::trial_solve(view, &get_vec(&self.puzzle.grid, &from, index));

        for i in to_remove {
            let c = get_coordinate(&from, n, index, i.0);
            if !self.remove(&c, &i.1) {
                println!("{}", self.puzzle.to_detailed_string());
                println!("{:?} {}", c, i.1);
                panic!();
            }
        }

        return still_potentially_solvable;
    }
}

pub fn solve(p: &mut Puzzle) {
    let start = Instant::now();
    initial_view_solve(p);
    let mut solver = Solver::new(p);
    //println!("{}", solver.puzzle.to_detailed_string());
    //solver.analyze_view(Direction::WEST, 4);
    solver.change_flag = true;

    while solver.change_flag {
        solver.change_flag = false;
        solver.simple_solve();
        solver.view_solve();
        // I don't think grouping solve is exponential, but I haven't proven it yet, so I'm going
        // to treat it as exponential and only use it if I can't make progress otherwise.
        if !solver.change_flag {
            solver.grouping_solve();
        }

        // I don't think graph solve is exponential, but I haven't proven it yet, so I'm going to
        // treat it as exponential and only use it if I can't make progress otherwise.
        if !solver.change_flag {
            let now = Instant::now();
            let maximal_graphs = solver.graph_solve();
            println!("Graph solve: {:>8}", now.elapsed().as_micros());
            if !solver.change_flag {
                let now = Instant::now();
                for (class, grid) in maximal_graphs {
                    if !solver.view_solve_with_grid(&grid) {
                        for p in class {
                            solver.remove(&Coordinate(p.0 as usize, p.1 as usize), &p.2);
                        }
                    }
                }
                println!("Cross solve: {:>8}", now.elapsed().as_micros());
            }
        }

        // Since brute force solving the view is potentially exponential, only do it if we
        // can't make progress with a more efficient method.
        if !solver.change_flag {
            let now = Instant::now();
            solver.brute_force_view_solve();
            println!("Force views: {:>8}", now.elapsed().as_micros());
        }
    }

    // I can see what this cell can't be this value, but I need to think how I can describe the
    // process to find the reasoning...
    //solver.remove(&Coordinate(1, 3), &2);

    // println!("{}\n", solver.puzzle.to_detailed_string());
    // solver.brute_force_view_solve();
    let duration = start.elapsed();
    println!("\nDone! Total Time: {:>8}", duration.as_micros());
    if duration.as_secs() >= 1 {
        println!("X Took more than a second.");
    } else {
        println!("Under a second!");
    }
}


fn initial_view_solve(p: &mut Puzzle) {
    let n = p.size;

    for (column, view) in p.north.iter().enumerate() {
        if *view == 1 {
            p.grid[0][column].clear();
            p.grid[0][column].insert((n - 1) as u8);
        } else if *view != 0 {
            for row in 0..n {
                for value in (1 + ((p.size + row) as u8) - view)..(n as u8) {
                    p.grid[row][column].remove(&(value as u8));
                }
            }
        }
    }
    for (row, view) in p.east.iter().enumerate() {
        if *view == 1 {
            p.grid[row][n - 1].clear();
            p.grid[row][n - 1].insert((n - 1) as u8);
        } else if *view != 0 {
            for i in 0..n {
                let column = n - i - 1;
                for value in (1 + ((p.size + i) as u8) - view)..(n as u8) {
                    p.grid[row][column].remove(&(value as u8));
                }
            }
        }
    }
    for (column, view) in p.south.iter().enumerate() {
        if *view == 1 {
            p.grid[n - 1][column].clear();
            p.grid[n - 1][column].insert((n - 1) as u8);
        } else if *view != 0 {
            for i in 0..n {
                let row = n - i - 1;
                for value in (1 + ((p.size + i) as u8) - view)..(n as u8) {
                    p.grid[row][column].remove(&(value as u8));
                }
            }
        }
    }
    for (row, view) in p.west.iter().enumerate() {
        if *view == 1 {
            p.grid[row][0].clear();
            p.grid[row][0].insert((n - 1) as u8);
        } else if *view != 0 {
            for column in 0..n {
                for value in (1 + ((p.size + column) as u8) - view)..(n as u8) {
                    p.grid[row][column].remove(&(value as u8));
                }
            }
        }
    }
}

use super::puzzle::Puzzle;
use std::collections::HashSet;
use std::collections::HashMap;
mod row_solver;

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

fn get_vec<'a>(p: & 'a Puzzle, d: &Direction, i: usize) -> Vec<&'a HashSet<u8>> {
    let mut vec = match d {
        Direction::NORTH | Direction::SOUTH => p.column(i),
        Direction::EAST | Direction::WEST => p.row(i),
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

    fn handle_solved_cells(& mut self) {
        while self.recently_solved.len() > 0 {
            let c = self.recently_solved.pop().unwrap();
            let value = self.puzzle.grid[c.0][c.1].iter().next().unwrap().clone();
            for i in 0..self.puzzle.size {
                if i != c.1 {
                    self.remove(&Coordinate(c.0, i), &value);
                }
                if i != c.0 {
                    self.remove(&Coordinate(i, c.1), &value);
                }
            }
        }
    }

    fn handle_unique_in_row(& mut self) {
        while self.recently_unique_in_row.len() > 0 {
            let (row, value) = self.recently_unique_in_row.pop().unwrap();
            for i in 0..self.puzzle.size {
                if self.puzzle.grid[row][i].contains(&value) {
                    self.set(&Coordinate(row, i), &value);
                }
            }
        }
    }

    fn handle_unique_in_column(& mut self) {
        while self.recently_unique_in_column.len() > 0 {
            let (column, value) = self.recently_unique_in_column.pop().unwrap();
            for i in 0..self.puzzle.size {
                if self.puzzle.grid[i][column].contains(&value) {
                    self.set(&Coordinate(i, column), &value);
                }
            }
        }
    }

    fn row_pair_solve(& mut self, index: usize) {
        let n = self.puzzle.size;
        let mut pairs: HashMap<(u8, u8), usize> = HashMap::new();
        for i in 0..n {
            if self.puzzle.grid[index][i].len() == 2 {
                let min = *self.puzzle.grid[index][i].iter().min().unwrap();
                let max = *self.puzzle.grid[index][i].iter().max().unwrap();
                let pair = (min, max);
                match pairs.get(&pair) {
                    Some(position) => {
                        // This pair appeared before.
                        // No other cell can have one of these two values.
                        for j in 0..n {
                            if j != *position && j != i {
                                self.remove(&Coordinate(index, j), &min);
                                self.remove(&Coordinate(index, j), &max);
                            }
                        }
                    },
                    None => {
                        // Add this pair to our map
                        pairs.insert(pair, i);
                    },
                };
            }
        }
    }

    fn column_pair_solve(& mut self, index: usize) {
        let n = self.puzzle.size;
        let mut pairs: HashMap<(u8, u8), usize> = HashMap::new();
        for i in 0..n {
            if self.puzzle.grid[i][index].len() == 2 {
                let min = *self.puzzle.grid[i][index].iter().min().unwrap();
                let max = *self.puzzle.grid[i][index].iter().max().unwrap();
                let pair = (min, max);
                match pairs.get(&pair) {
                    Some(position) => {
                        // This pair appeared before.
                        // No other cell can have one of these two values.
                        for j in 0..n {
                            if j != *position && j != i {
                                self.remove(&Coordinate(j, index), &min);
                                self.remove(&Coordinate(j, index), &max);
                            }
                        }
                    },
                    None => {
                        // Add this pair to our map
                        pairs.insert(pair, i);
                    },
                };
            }
        }
    }

    // For any given value, if two rows can only put that value in the same two columns, then no
    // other cell in those columns can be that value.
    fn rows_with_matching_pair_solve(& mut self) {
        for i in 0..self.puzzle.size {
            let rows_with_two_options_left:Vec<usize> = self.value_count_by_row.iter().enumerate().filter(|(_index, x)| x[i] == 2).map(|(index, _x)| index).collect();
            if rows_with_two_options_left.len() < 2 {
                continue;
            }
            let mut pairs: HashMap<(usize, usize), usize> = HashMap::new();
            for row in rows_with_two_options_left {
                let mut columns = self.puzzle.grid[row].iter().enumerate().filter(|(_index, x)| x.contains(&(i as u8))).map(|(index, _x)| index);
                let pair: (usize, usize) = (columns.next().unwrap(), columns.next().unwrap());
                if pairs.contains_key(&pair) {
                    let other_row = pairs.get(&pair).unwrap();
                    // Remove all intances of i in the two columns
                    // (unless they are in one of the two rows)
                    for j in 0..self.puzzle.size {
                        if j != row && j != *other_row {
                            self.remove(&Coordinate(j, pair.0), &(i as u8));
                            self.remove(&Coordinate(j, pair.1), &(i as u8));
                        }
                    }

                } else {
                    pairs.insert(pair, row);
                }
            }
        }
    }

    fn columns_with_matching_pair_solve(& mut self) {
        for i in 0..self.puzzle.size {
            let columns_with_two_options_left:Vec<usize> = self.value_count_by_column.iter().enumerate().filter(|(_index, x)| x[i] == 2).map(|(index, _x)| index).collect();
            if columns_with_two_options_left.len() < 2 {
                continue;
            }
            let mut pairs: HashMap<(usize, usize), usize> = HashMap::new();
            for column in columns_with_two_options_left {
                let mut rows = self.puzzle.grid.iter().enumerate().filter(|(_index, x)| x[column].contains(&(i as u8))).map(|(index, _x)| index);
                let pair: (usize, usize) = (rows.next().unwrap(), rows.next().unwrap());
                if pairs.contains_key(&pair) {
                    let other_column = pairs.get(&pair).unwrap();
                    // Remove all intances of i in the two columns
                    // (unless they are in one of the two rows)
                    for j in 0..self.puzzle.size {
                        if j != column && j != *other_column {
                            self.remove(&Coordinate(pair.0, j), &(i as u8));
                            self.remove(&Coordinate(pair.1, j), &(i as u8));
                        }
                    }

                } else {
                    pairs.insert(pair, column);
                }
            }
        }
    }

    fn pair_solve(& mut self) {
        for i in 0..self.puzzle.size {
            self.row_pair_solve(i);
            self.column_pair_solve(i);
        }
        self.rows_with_matching_pair_solve();
        self.columns_with_matching_pair_solve();
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

    fn brute_force_view_solve(& mut self) -> bool {
        for i in 0..self.puzzle.size {
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

        let (still_potentially_solvable, to_remove) = row_solver::solve(view, &get_vec(self.puzzle, &from, index));

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

        let (still_potentially_solvable, to_remove) = row_solver::trial_solve(view, &get_vec(self.puzzle, &from, index));

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

    fn simple_solve(& mut self) {
        while self.recently_solved.len() > 0 || self.recently_unique_in_row.len() > 0 || self.recently_unique_in_column.len() > 0 {
            self.handle_solved_cells();
            self.handle_unique_in_row();
            self.handle_unique_in_column();
        }
    }
}

pub fn solve(p: &mut Puzzle) {
    initial_view_solve(p);
    let mut solver = Solver::new(p);
    //println!("{}", solver.puzzle.to_detailed_string());
    //solver.analyze_view(Direction::WEST, 4);
    solver.simple_solve();
    solver.view_solve();
    solver.pair_solve();

    while solver.change_flag {
        solver.change_flag = false;
        solver.simple_solve();
        solver.view_solve();
        solver.pair_solve();
        // Since brute force solving the view is potentially exponential, only do it if we
        // can't make progress with a more efficient method.
        if !solver.change_flag {
            println!("Brute force rows");
            solver.brute_force_view_solve();
        }
    }

    // solver.view_solve();
    // solver.view_solve();
    // println!("{}\n", solver.puzzle.to_detailed_string());
    // solver.view_solve();
    // solver.analyze_view(Direction::SOUTH, 0);
    // solver.analyze_view(Direction::SOUTH, 1);
    // solver.analyze_view(Direction::SOUTH, 2);
    // solver.analyze_view(Direction::SOUTH, 3);
    // solver.analyze_view(Direction::SOUTH, 4);
    // solver.analyze_view(Direction::SOUTH, 5);
    // solver.analyze_view(Direction::EAST, 0);
    // solver.analyze_view(Direction::EAST, 1);
    // solver.analyze_view(Direction::EAST, 2);
    // solver.analyze_view(Direction::EAST, 3);
    // solver.analyze_view(Direction::EAST, 4);
    // solver.analyze_view(Direction::EAST, 5);
    // solver.analyze_view(Direction::WEST, 0);
    // solver.analyze_view(Direction::WEST, 1);
    // solver.analyze_view(Direction::WEST, 2);
    // solver.analyze_view(Direction::WEST, 3);
    // solver.analyze_view(Direction::WEST, 4);
    // solver.analyze_view(Direction::WEST, 5);
    // solver.analyze_view(Direction::NORTH, 0);
    // solver.analyze_view(Direction::NORTH, 1);
    // solver.analyze_view(Direction::NORTH, 2);
    // solver.analyze_view(Direction::NORTH, 3);
    // solver.analyze_view(Direction::NORTH, 4);
    // solver.analyze_view(Direction::NORTH, 5);
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

use super::puzzle::Puzzle;
use super::solver::Solver;
use super::solver::coordinate::Coordinate;

use rand::seq::SliceRandom;
use rand::thread_rng;

// (row, column, value)
#[derive(Clone, Debug, Copy, PartialEq, Eq, Hash)]
pub struct Hint (usize, usize, u8);

pub fn make_puzzle(size: usize) -> Puzzle {
    let mut cells_with_hint: Vec<Coordinate> = Vec::new();
    let mut empty_cells: Vec<Coordinate> = Vec::new();
    let mut grid: Vec<Vec<Option<u8>>> = Vec::new();
    for i in 0..size {
        grid.push(Vec::new());
        for j in 0..size {
            grid[i].push(Option::None);
            empty_cells.push(Coordinate(i, j));
        }
    }

    // Add hints until uniquely solvable
    let mut rng = thread_rng();
    empty_cells.shuffle(&mut rng);

    let mut next_coordinate = empty_cells.pop().unwrap();
    let mut possible_values = Vec::from([0, 1, 2, 3]);
    possible_values.shuffle(&mut rng);
    let mut last_hint = (next_coordinate.0, next_coordinate.1, possible_values.pop().unwrap());
    grid[last_hint.0][last_hint.1] = Option::Some(last_hint.2);
    let mut solver = Solver::new(Puzzle::from_grid(&grid, 0));
    println!("{}", solver.to_string());
    let mut solutions = solver.full_solve(0, false);

    while solutions.len() != 1 {
        if solutions.len() > 1 {
            cells_with_hint.push(next_coordinate);
            match empty_cells.pop() {
                Some(x) => { next_coordinate = x; },
                None => {
                    // Reached a dead end.
                    // Try again lol
                    println!("{}", solver.to_string());
                    return make_puzzle(size);
                }
            }
            possible_values = Vec::from([0, 1, 2, 3]);
            possible_values.shuffle(&mut rng);
        }
        match possible_values.pop() {
            Some(x) => { last_hint = (next_coordinate.0, next_coordinate.1, x); },
            None => {
                println!("{}", solver.to_string());
                println!("{:?}", last_hint);
                panic!();
                last_hint = (next_coordinate.0, next_coordinate.1, 4);
            },
        }
        grid[last_hint.0][last_hint.1] = Option::Some(last_hint.2);
        solver = Solver::new(Puzzle::from_grid(&grid, 0));
        println!("{}", solver.to_string());
        solutions = solver.full_solve(0, false);
    }
    cells_with_hint.push(next_coordinate);

    // Remove hints that keep it uniquely solvable
    cells_with_hint.shuffle(&mut rng);
    let mut difficulty = solver.depth_needed;
    for cell in cells_with_hint {
        let hint = grid[cell.0][cell.1].unwrap();
        grid[cell.0][cell.1] = Option::None;
        solver = Solver::new(Puzzle::from_grid(&grid, 0));
        println!("{}", solver.to_string());
        solutions = solver.full_solve(0, false);
        if solutions.len() == 1 {
            difficulty = solver.depth_needed;
        } else {
            grid[cell.0][cell.1] = Option::Some(hint);
        }
    }

    return Puzzle::from_grid(&grid, difficulty);
}

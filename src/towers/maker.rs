use super::puzzle::Puzzle;
use super::solver::Solver;
use crate::latin::square::Square;
use rand::seq::SliceRandom;
use rand::thread_rng;

pub fn make_puzzle(size: u8) -> Puzzle {
    // Start with a random latin square
    let square: Square = Square::random(size);

    // Generate the full hints puzzle
    let p = Puzzle::from_square(&square, 0);

    let mut hints_to_remove: Vec<bool> = vec![false; p.number_of_hints()];
    let mut cell_hints: Vec<usize> = Vec::new();
    let mut view_hints: Vec<usize> = Vec::new();

    for i in 0..(size*size) as usize {
        cell_hints.push(i);
    }
    for i in (size*size) as usize..(size*size + 4*size) as usize {
        view_hints.push(i);
    }
    let mut rng = thread_rng();
    // keep view hints after cell hits to make the puzzle more towers-like instead of more latin square-like.
    cell_hints.shuffle(&mut rng);
    view_hints.shuffle(&mut rng);

    let mut difficulty = 0;

    for i in cell_hints.iter().chain(view_hints.iter()) {
        hints_to_remove[*i] = true;
        let temp_puzzle = p.with_hints_removed(&hints_to_remove, difficulty);
        let mut s = Solver::new(temp_puzzle);
        let solutions = s.full_solve(0, false);
        if solutions.len() > 1 {
            // No longer uniquely solveable, don't remove this hint.
            hints_to_remove[*i] = false;
        } else {
            // How hard was it to solve
            difficulty = s.depth_needed;
        }
    }

    return p.with_hints_removed(&hints_to_remove, difficulty);
}

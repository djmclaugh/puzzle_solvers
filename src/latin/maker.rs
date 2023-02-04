use crate::perm::permutation::random_perm;
use super::puzzle::Puzzle;
use super::solver::Solver;

use rand::seq::SliceRandom;
use rand::thread_rng;

pub fn random_filled(n: u8) -> Puzzle {
    let mut rows:Vec<Vec<u8>> = Vec::new();

    // Any set of rows that satisfy the latin square rules can be extended to a full latin square.
    // TODO: Learned this from a comment in Simon Tatham's implementation. I should find and link a proof.
    // Create a full latin square by adding one row at a time.
    for _i in 0..n {
        rows.push(random_perm(n, &rows));
    }

    let grid:Vec<Vec<Option<u8>>> = rows.iter().map(|row| row.iter().map(|val| Some(val.clone())).collect()).collect();
    return Puzzle { size: n as usize, grid, difficulty: 0 };
}

pub fn make_puzzle(size: u8) -> Puzzle {
    // Start with a random latin square
    let p: Puzzle = random_filled(size);

    let mut hints_to_remove: Vec<bool> = vec![false; p.number_of_hints()];
    let mut cell_hints: Vec<usize> = Vec::new();

    for i in 0..(size*size) as usize {
        cell_hints.push(i);
    }
    let mut rng = thread_rng();
    cell_hints.shuffle(&mut rng);

    let mut difficulty = 0;

    for i in cell_hints.iter() {
        hints_to_remove[*i] = true;
        let temp_puzzle = p.with_hints_removed(&hints_to_remove, difficulty);
        let mut s = Solver::new(temp_puzzle);
        let solutions = s.full_solve(0, false);
        if solutions.len() > 1 {
            // No longer uniquely solveable, don't remove this hint.
            hints_to_remove[*i] = false;
        } else {
            // How hard was it to solve
            difficulty = solutions[0].depth_needed;
        }
    }
    return p.with_hints_removed(&hints_to_remove, difficulty);
}

use crate::perm::permutation::random_perm;
use super::puzzle::Puzzle;

pub fn random_filled(n: u8) -> Puzzle {
    let mut rows:Vec<Vec<u8>> = Vec::new();

    for _i in 0..n {
        rows.push(random_perm(n, &rows));
    }

    let grid:Vec<Vec<Option<u8>>> = rows.iter().map(|row| row.iter().map(|val| Some(val.clone())).collect()).collect();
    return Puzzle { size: n as usize, grid };
}

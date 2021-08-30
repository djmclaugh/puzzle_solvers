use super::puzzle::Puzzle;
use super::solver::Solver;

pub fn make_puzzle(size: usize) -> Puzzle {
    // Start with a random latin square
    let mut grid: Vec<Vec<u8>> = Vec::new();
    grid.push(Vec::new());
    grid.push(Vec::new());
    grid.push(Vec::new());

    grid[0].push(0);
    grid[0].push(1);
    grid[0].push(2);

    grid[1].push(1);
    grid[1].push(2);
    grid[1].push(0);

    grid[2].push(2);
    grid[2].push(0);
    grid[2].push(1);

    // Generate the full hints puzzle
    let p = Puzzle::from_grid(&grid);

    // Find all minimal sets of hints that produce a unique solution.
    return p.with_hints_removed(vec![
        true, true, true,
        true, true, true,
        true, true, true,
        true, true, true,
        true, true, true,
        true, true, true,
        false, true, true,]);

}

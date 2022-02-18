use super::Solver;
use super::Status;

// Solver methods based on making a guess and seeing if we end up with a contradiction.
impl Solver {
    pub fn depth_solve(& mut self, depth: u8, should_log: bool) -> Vec<Solver> {
        let n = self.puzzle.size;

        while !self.remaining_edges.is_empty() {
            let mut solutions: Vec<Solver> = Vec::new();
            // Prioritize edges next to hints as they tend to lead to more inferences.
            let e = if self.remaining_edges_next_to_hints.is_empty() {
                self.remaining_edges.iter().next().unwrap().clone()
            } else {
                self.remaining_edges_next_to_hints.iter().next().unwrap().clone()
            };
            // Check if setting the edge off leads to a contradiction
            let mut copy = self.clone();
            copy.set(&e, false);
            let mut copy_solutions = copy.full_solve(depth + 1, should_log);
            solutions.append(&mut copy_solutions);
            if copy.depth_needed > self.depth_needed {
                self.depth_needed = copy.depth_needed;
            }
            // If setting the edge off leads to a contradiction, then the edge must be on.
            if copy.status == Status::Unsolvable {
                self.set(&e, true);
                // Now that we have new information, let's apply our non-recursive methods as well.
                self.non_recursive_solve();
                // If we end up with an unsovlable or uniquely solvable puzzle, then we are done.
                if self.status == Status::Unsolvable {
                    return solutions;
                } else if self.status == Status::UniqueSolution {
                    solutions.push(self.clone());
                    return solutions;
                }
                // Since we know that the edge must be on, no need to continue with the next part.
                continue;
            } else if solutions.len() > 1 {
                // If multiple solutions were found already, then we can stop looking
                self.status = Status::MultipleSolutions;
                return solutions;
            }
            // If setting the edge off leads to a single solution, check what happens when we set
            // the edge on.
            copy = self.clone();
            copy.set(&e, true);
            copy_solutions = copy.full_solve(depth + 1, should_log);
            solutions.append(&mut copy_solutions);
            if copy.depth_needed > self.depth_needed {
                self.depth_needed = copy.depth_needed;
            }
            if copy.status == Status::Unsolvable {
                // If single solutions when off and no solutions when on, then there only a single
                // solution in total.
                self.status =  Status::UniqueSolution;
                return solutions;
            } else {
                // If there is a single solution when off and at least one solution when on, then
                // there are multiple solutions.
                self.status = Status::MultipleSolutions;
                return solutions;
            }
        }

        if self.satisfies_contraints() {
            self.status = Status::UniqueSolution;
            return Vec::from([self.clone()]);
        } else {
            self.status = Status::Unsolvable;
            return Vec::new();
        }
    }
}

use super::Solver;
use super::Status;

// Solver methods based on making a guess and seeing if we end up with a contradiction.
impl Solver {
    pub fn depth_solve(& mut self, depth: u8, should_log: bool) -> Vec<Solver> {
        let n = self.puzzle.size;
        let mut solutions: Vec<Solver> = Vec::new();

        for i in 0..(n+1) {
            for j in 0..n {
                if !self.h_edges[i][j].is_on && !self.h_edges[i][j].is_off {
                    let mut copy = self.clone();
                    copy.set(&self.h_edges[i][j], false);
                    let mut copy_solutions = copy.full_solve(depth + 1, should_log);
                    solutions.append(&mut copy_solutions);
                    if copy.depth_needed > self.depth_needed {
                        self.depth_needed = copy.depth_needed;
                    }
                    if copy.status == Status::Unsolvable {
                        self.set(&self.h_edges[i][j].clone(), true);
                        self.non_recursive_solve();
                        if self.status == Status::Unsolvable {
                            return solutions;
                        } else if self.status == Status::UniqueSolution {
                            solutions.push(self.clone());
                            return solutions;
                        }
                    } else if solutions.len() > 1 {
                        self.status = Status::MultipleSolutions;
                        return solutions;
                    } else {
                        let mut copy = self.clone();
                        copy.set(&self.h_edges[i][j], true);
                        let mut copy_solutions = copy.full_solve(depth + 1, should_log);
                        solutions.append(&mut copy_solutions);
                        if copy.depth_needed > self.depth_needed {
                            self.depth_needed = copy.depth_needed;
                        }
                        if copy.status == Status::Unsolvable {
                            self.set(&self.h_edges[i][j].clone(), false);
                            self.non_recursive_solve();
                            if self.status == Status::Unsolvable {
                                return solutions;
                            } else if self.status == Status::UniqueSolution {
                                solutions.push(self.clone());
                                return solutions;
                            }
                        } else if solutions.len() > 1 {
                            self.status = Status::MultipleSolutions;
                            return solutions;
                        }
                    }
                }

                if !self.v_edges[j][i].is_on && !self.v_edges[j][i].is_off {
                    let mut copy = self.clone();
                    copy.set(&self.v_edges[j][i], false);
                    let mut copy_solutions = copy.full_solve(depth + 1, should_log);
                    solutions.append(&mut copy_solutions);
                    if copy.depth_needed > self.depth_needed {
                        self.depth_needed = copy.depth_needed;
                    }
                    if copy.status == Status::Unsolvable {
                        self.set(&self.v_edges[j][i].clone(), true);
                        self.non_recursive_solve();
                        if self.status == Status::Unsolvable {
                            return solutions;
                        } else if self.status == Status::UniqueSolution {
                            solutions.push(self.clone());
                            return solutions;
                        }
                    } else if solutions.len() > 1 {
                        self.status = Status::MultipleSolutions;
                        return solutions;
                    } else {
                        let mut copy = self.clone();
                        copy.set(&self.v_edges[j][i], true);
                        let mut copy_solutions = copy.full_solve(depth + 1, should_log);
                        solutions.append(&mut copy_solutions);
                        if copy.depth_needed > self.depth_needed {
                            self.depth_needed = copy.depth_needed;
                        }
                        if copy.status == Status::Unsolvable {
                            self.set(&self.v_edges[j][i].clone(), false);
                            self.non_recursive_solve();
                            if self.status == Status::Unsolvable {
                                return solutions;
                            } else if self.status == Status::UniqueSolution {
                                solutions.push(self.clone());
                                return solutions;
                            }
                        } else if solutions.len() > 1 {
                            self.status = Status::MultipleSolutions;
                            return solutions;
                        }
                    }
                }
            }
        }

        return solutions;
    }
}

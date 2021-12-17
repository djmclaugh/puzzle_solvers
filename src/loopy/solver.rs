pub mod coordinate;
mod path_tracker;
mod depth_solver;

use coordinate::Coordinate;
use path_tracker::PathTracker;
use super::puzzle::Puzzle;
use std::slice::Iter;
use std::time::Instant;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Direction {
    UP,
    RIGHT,
    DOWN,
    LEFT,
}
impl Direction {
    pub fn iter() -> Iter<'static, Direction> {
        static DIRECTIONS: [Direction; 4] = [Direction::UP, Direction::RIGHT, Direction::DOWN, Direction::LEFT];
        return DIRECTIONS.iter();
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum EdgeType {
    HORIZONTAL,
    VERTICAL
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Status {
    // No solution exist
    Unsolvable,
    // Only one solution exists
    UniqueSolution,
    // Many solutions exist
    MultipleSolutions,
    // Don't know if solvable or not yet
    InProgress,
}

fn cell_to_string(view: &Option<u8>) -> String {
    match view {
        Some(x) => x.to_string(),
        None => String::from("·")
    }
}

#[derive(Clone)]
#[derive(Debug)]
#[derive(Copy)]
pub struct Edge {
    is_on: bool,
    is_off: bool,
    row: usize,
    col: usize,
    edge_type: EdgeType,
}

#[derive(Clone, Debug)]
pub struct Solver {
    puzzle: Puzzle,
    h_edges: Vec<Vec<Edge>>,
    v_edges: Vec<Vec<Edge>>,
    recently_affected_cells: Vec<Coordinate>,
    recently_affected_nodes: Vec<Coordinate>,
    recently_extended_paths: Vec<Coordinate>,
    paths: PathTracker,
    change_flag: bool,
    num_off: usize,
    pub status: Status,
    pub depth_needed: u8,
}

impl Solver {
    pub fn to_string(&self) -> String {
      let n = self.puzzle.size;
      let mut rows: Vec<String> = Vec::new();

      for i in 0..n {
          let mut row = Vec::new();
          for j in 0..n {
              row.push(String::from(" "));
              if self.h_edges[i][j].is_on {
                  row.push(String::from("─"));
              } else if self.h_edges[i][j].is_off {
                  row.push(String::from(" "));
              } else {
                  row.push(String::from("┄"));
              }
          }
          rows.push(row.join(""));
          row = Vec::new();
          for j in 0..n {
              if self.v_edges[i][j].is_on {
                  row.push(String::from("│"));
              } else if self.v_edges[i][j].is_off {
                  row.push(String::from(" "));
              } else {
                  row.push(String::from("┆"));
              }
              row.push(cell_to_string(&self.puzzle.grid[i][j]));
          }
          if self.v_edges[i][n].is_on {
              row.push(String::from("│"));
          } else if self.v_edges[i][n].is_off {
              row.push(String::from(" "));
          } else {
              row.push(String::from("┆"));
          }
          rows.push(row.join(""));
      }
      let mut row = Vec::new();
      for j in 0..n {
          row.push(String::from(" "));
          if self.h_edges[n][j].is_on {
              row.push(String::from("─"));
          } else if self.h_edges[n][j].is_off {
              row.push(String::from(" "));
          } else {
              row.push(String::from("┄"));
          }
      }
      rows.push(row.join(""));

      // Join and return rows.
      return rows.join("\n");
    }

    pub fn new(p: Puzzle) -> Solver {
        let n = p.size;
        let mut h_edges: Vec<Vec<Edge>> = Vec::new();
        let mut v_edges: Vec<Vec<Edge>> = Vec::new();

        for i in 0..n {
            h_edges.push(Vec::new());
            v_edges.push(Vec::new());
            for j in 0..n {
                h_edges[i].push(Edge{is_on: false, is_off: false, row: i, col: j, edge_type: EdgeType::HORIZONTAL});
                v_edges[i].push(Edge{is_on: false, is_off: false, row: i, col: j, edge_type: EdgeType::VERTICAL});
            }
            v_edges[i].push(Edge{is_on: false, is_off: false, row: i, col: n, edge_type: EdgeType::VERTICAL});
        }
        h_edges.push(Vec::new());
        for j in 0..n {
            h_edges[n].push(Edge{is_on: false, is_off: false, row: n, col: j, edge_type: EdgeType::HORIZONTAL});
        }

        return Solver {
            puzzle: p,
            h_edges,
            v_edges,
            paths: PathTracker::new(),
            num_off: 0,
            change_flag: false,
            status: Status::InProgress,
            recently_affected_cells: Vec::new(),
            recently_affected_nodes: Vec::new(),
            recently_extended_paths: Vec::new(),
            depth_needed: 0,
        }
    }

    fn edge_from_cell(&self, c: &Coordinate, d: &Direction) -> Edge {
        return match d {
            Direction::UP => self.h_edges[c.0][c.1],
            Direction::DOWN => self.h_edges[c.0 + 1][c.1],
            Direction::LEFT => self.v_edges[c.0][c.1],
            Direction::RIGHT => self.v_edges[c.0][c.1 + 1],
        }
    }

    fn edges_from_cell(&self, c: &Coordinate) -> (Edge, Edge, Edge, Edge) {
        return (
            self.edge_from_cell(c, &Direction::UP),
            self.edge_from_cell(c, &Direction::RIGHT),
            self.edge_from_cell(c, &Direction::DOWN),
            self.edge_from_cell(c, &Direction::LEFT),
        )
    }

    fn edge_from_node(&self, c: &Coordinate, d: &Direction) -> Option<Edge> {
        return match d {
            Direction::UP => {
                if c.0 == 0 { Option::None }
                else { Option::Some(self.v_edges[c.0 - 1][c.1]) }
            },
            Direction::DOWN => {
                if c.0 == self.puzzle.size { Option::None }
                else { Option::Some(self.v_edges[c.0][c.1]) }
            },
            Direction::LEFT => {
                if c.1 == 0 { Option::None }
                else { Option::Some(self.h_edges[c.0][c.1 - 1]) }
            },
            Direction::RIGHT => {
                if c.1 == self.puzzle.size { Option::None }
                else { Option::Some(self.h_edges[c.0][c.1]) }
            },
        }
    }

    fn edges_from_node(&self, c: &Coordinate) -> (Option<Edge>, Option<Edge>, Option<Edge>, Option<Edge>) {
        return (
            self.edge_from_node(c, &Direction::UP),
            self.edge_from_node(c, &Direction::RIGHT),
            self.edge_from_node(c, &Direction::DOWN),
            self.edge_from_node(c, &Direction::LEFT),
        )
    }

    fn nodes_from_edge(&self, e: &Edge) -> (Coordinate, Coordinate) {
        return match e.edge_type {
            EdgeType::HORIZONTAL => {
                (Coordinate(e.row, e.col), Coordinate(e.row, e.col + 1))
            },
            EdgeType::VERTICAL => {
                (Coordinate(e.row, e.col), Coordinate(e.row + 1, e.col))
            },
        }
    }

    fn cells_from_edge(&self, e: &Edge) -> (Option<Coordinate>, Option<Coordinate>) {
        match e.edge_type {
            EdgeType::HORIZONTAL => {
                let a = match e.row == 0 {
                    true => Option::None,
                    false => Option::Some(Coordinate(e.row - 1, e.col)),
                };
                let b = match e.row == self.puzzle.size {
                    true => Option::None,
                    false => Option::Some(Coordinate(e.row, e.col)),
                };
                (a, b)
            },
            EdgeType::VERTICAL => {
                let a = match e.col == 0 {
                    true => Option::None,
                    false => Option::Some(Coordinate(e.row, e.col - 1)),
                };
                let b = match e.col == self.puzzle.size {
                    true => Option::None,
                    false => Option::Some(Coordinate(e.row, e.col)),
                };
                (a, b)
            },
        }
    }

    fn set(& mut self, edge: &Edge, on: bool) {
        let nodes = self.nodes_from_edge(edge);
        let actual_edge = match edge.edge_type {
            EdgeType::HORIZONTAL => self.h_edges.get_mut(edge.row).unwrap().get_mut(edge.col).unwrap(),
            EdgeType::VERTICAL => self.v_edges.get_mut(edge.row).unwrap().get_mut(edge.col).unwrap(),
        };
        let mut has_changed = false;
        if on && !actual_edge.is_on {
            actual_edge.is_on = true;
            self.paths.add_edge(&nodes.0, &nodes.1);
            self.recently_extended_paths.push(nodes.0);
            self.recently_extended_paths.push(nodes.1);
            has_changed = true;
        }
        if !on && !actual_edge.is_off {
            actual_edge.is_off = true;
            self.num_off += 1;
            has_changed = true;
        }
        if has_changed {
            self.change_flag = true;
            let new_nodes = self.nodes_from_edge(edge);
            self.recently_affected_nodes.push(new_nodes.0);
            self.recently_affected_nodes.push(new_nodes.1);
            let new_cells = self.cells_from_edge(edge);
            if new_cells.0.is_some() {
                self.recently_affected_cells.push(new_cells.0.unwrap());
            }
            if new_cells.1.is_some() {
                self.recently_affected_cells.push(new_cells.1.unwrap());
            }
        }
        if edge.is_on && edge.is_off {
            self.status = Status::Unsolvable;
        }
    }

    fn apply_local_single_loop_contraints(& mut self) {
        while !self.recently_extended_paths.is_empty() && self.status == Status::InProgress {
            let node = self.recently_extended_paths.pop().unwrap();
            let edges = self.edges_from_node(&node);
            for e in [edges.0, edges.1, edges.2, edges.3] {
                match e {
                    Some(x) => {
                        if !x.is_on && !x.is_off {
                            let nodes = self.nodes_from_edge(&x);
                            if self.paths.num_paths() > 1 && self.paths.would_create_loop(&nodes.0, &nodes.1) {
                                self.set(&x, false);
                            }
                        }
                    },
                    None => {
                        // Do nothing
                    },
                }
            }
        }
    }

    fn apply_node_constraints(& mut self) {
        while !self.recently_affected_nodes.is_empty() && self.status == Status::InProgress {
            let node = self.recently_affected_nodes.pop().unwrap();
            let edges = self.edges_from_node(&node);
            let mut real_edges: Vec<Edge> = Vec::new();
            for e in [edges.0, edges.1, edges.2, edges.3] {
                match e {
                    Some(x) => { real_edges.push(x); },
                    None => {},
                }
            }

            let mut on_count = 0;
            let mut off_count = 0;

            for e in real_edges.iter() {
                if e.is_on { on_count += 1; }
                if e.is_off { off_count += 1; }
            }

            if on_count > 2 {
                self.status = Status::Unsolvable;
            } else if on_count == 2 {
                // All other edges should be set to off.
                for e in real_edges.iter() {
                    if !e.is_on { self.set(&e, false); }
                }
            } else if on_count == 1 {
                // At least one other edge should be on.
                if on_count + off_count == real_edges.len() {
                    // So if all the edges are set, we have a contradiction.
                    self.status = Status::Unsolvable;
                } else if on_count + off_count == real_edges.len() - 1 {
                    // If only one edge is not set, then that one should be set to on.
                    for e in real_edges.iter() {
                        if !e.is_on && !e.is_off { self.set(&e, true); }
                    }
                }
                // Otherwise, there is nothing we can do for now.
            } else if on_count == 0 {
                // If only one edge is not set, then that one should be set to off.
                if on_count + off_count == real_edges.len() - 1 {
                    for e in real_edges.iter() {
                        if !e.is_on && !e.is_off { self.set(&e, false); }
                    }
                }
                // Otherwise, there is nothing we can do for now.
            }
        }
    }

    fn apply_cell_constraints(& mut self) {
        while !self.recently_affected_cells.is_empty() && self.status == Status::InProgress {
            let cell = self.recently_affected_cells.pop().unwrap();
            let hint = match self.puzzle.grid[cell.0][cell.1] {
                Some(x) => { x },
                None => { continue; },
            };
            let edges_tuple = self.edges_from_cell(&cell);
            let mut edges: Vec<Edge> = Vec::new();
            for e in [edges_tuple.0, edges_tuple.1, edges_tuple.2, edges_tuple.3] {
                edges.push(e);
            }

            let mut on_count = 0;
            let mut unknown_count = 0;

            for e in edges.iter() {
                if e.is_on { on_count += 1; }
                if !e.is_on && !e.is_off { unknown_count += 1; }
            }

            if on_count > hint {
                self.status = Status::Unsolvable;
            } else if on_count == hint {
                // All unknown edges should be set to off.
                for e in edges.iter() {
                    if !e.is_on && !e.is_off { self.set(&e, false); }
                }
            }

            if on_count + unknown_count < hint {
                self.status = Status::Unsolvable;
            } else if on_count + unknown_count == hint {
                // All unknown edges should be set to on.
                for e in edges.iter() {
                    if !e.is_on && !e.is_off { self.set(&e, true); }
                }
            }
        }
    }

    fn satisfies_contraints(&self) -> bool {
        let n = self.puzzle.size;
        // Check if there is a single loop.
        if !self.paths.has_loop() || self.paths.num_paths() != 1 {
            return false;
        }
        // Check if each edge is either on or off (if neither, then assume off).
        for i in 0..n {
            for j in 0..(n+1) {
                if self.h_edges[j][i].is_on && self.h_edges[j][i].is_off { return false; }
                if self.v_edges[i][j].is_on && self.v_edges[i][j].is_off { return false; }
            }
        }
        // Check if each hint is satisfied.
        for i in 0..n {
            for j in 0..n {
                match self.puzzle.grid[i][j] {
                    None => {
                        // Do nothing
                    },
                    Some(x) => {
                        let edges = self.edges_from_cell(&Coordinate(i, j));
                        let mut count = 0;
                        for e in [edges.0, edges.1, edges.2, edges.3] {
                            if e.is_on { count += 1; }
                        }
                        if count != x {
                            return false;
                        }
                    }
                }
            }
        }
        // If all three checks passed, then the puzzle is solved!
        return true;
    }

    // Solve the puzzle using all non-recursive ways we know of.
    pub fn non_recursive_solve(&mut self) {
        self.initial_solve();
        self.change_flag = true;
        while self.change_flag && self.status == Status::InProgress {
            self.change_flag = false;
            self.apply_local_single_loop_contraints();
            self.apply_cell_constraints();
            self.apply_node_constraints();
            if self.status == Status::InProgress && self.paths.has_loop() {
                // If a loop has been made, then the puzzle is over.
                if self.satisfies_contraints() {
                    self.status = Status::UniqueSolution;
                } else {
                    self.status = Status::Unsolvable;
                }
            }
        }
        // If all the edges are off, then it's impossible to solve
        if self.num_off == 2 * self.puzzle.size * (self.puzzle.size + 1) {
            self.status = Status::Unsolvable;
        }
    }

    pub fn full_solve(&mut self, depth: u8, should_log: bool) -> Vec<Solver> {
        self.depth_needed = depth;
        let start = Instant::now();
        let mut solutions: Vec<Solver> = Vec::new();

        self.change_flag = true;
        while self.change_flag == true {
            self.non_recursive_solve();
            self.change_flag = false;
            if self.status == Status::InProgress {
                solutions = self.depth_solve(depth, should_log);
            } else {
                solutions = Vec::new();
                if self.status == Status::UniqueSolution {
                    solutions.push(self.clone());
                }
            }
        }

        let duration = start.elapsed();
        let indent = " ".repeat(8 * depth as usize);
        if should_log {
            println!("\n{}Done! Total Time: {}.{:>6}", indent, duration.as_secs(), duration.as_micros() % 1000000);
            println!("{}Status: {:?}", indent, self.status);
            println!("{}Depth: {:?}", indent, depth);
        }

        return solutions;
    }

    fn initial_solve(&mut self) {
        let n = self.puzzle.size;
        // Apply 0s
        for i in 0..n {
            for j in 0..n {
                match self.puzzle.grid[i][j] {
                    Some(0) => {
                        let c = Coordinate(i, j);
                        for d in Direction::iter() {
                            self.set(&self.edge_from_cell(&c, &d), false);
                        }
                    },
                    _ => {
                        // Do Nothing
                    }
                }
            }
        }
        // Look at corners;
        let top_left = Coordinate(0, 0);
        match self.puzzle.grid[0][0] {
            Some(1) => {
                self.set(&self.edge_from_cell(&top_left, &Direction::UP), false);
                self.set(&self.edge_from_cell(&top_left, &Direction::LEFT), false);
            },
            Some(2) => {
                self.set(&self.edge_from_cell(&Coordinate(0, 1), &Direction::UP), true);
                self.set(&self.edge_from_cell(&Coordinate(1, 0), &Direction::LEFT), true);
            },
            Some(3) => {
                self.set(&self.edge_from_cell(&top_left, &Direction::UP), true);
                self.set(&self.edge_from_cell(&top_left, &Direction::LEFT), true);
            },
            _ => {
                // Do nothing
            },
        }
        let bottom_left = Coordinate(n - 1, 0);
        match self.puzzle.grid[n - 1][0] {
            Some(1) => {
                self.set(&self.edge_from_cell(&bottom_left, &Direction::DOWN), false);
                self.set(&self.edge_from_cell(&bottom_left, &Direction::LEFT), false);
            },
            Some(2) => {
                self.set(&self.edge_from_cell(&Coordinate(n - 1, 1), &Direction::DOWN), true);
                self.set(&self.edge_from_cell(&Coordinate(n - 2, 0), &Direction::LEFT), true);
            },
            Some(3) => {
                self.set(&self.edge_from_cell(&bottom_left, &Direction::DOWN), true);
                self.set(&self.edge_from_cell(&bottom_left, &Direction::LEFT), true);
            },
            _ => {
                // Do nothing
            },
        }
        let top_right = Coordinate(0, n - 1);
        match self.puzzle.grid[0][n - 1] {
            Some(1) => {
                self.set(&self.edge_from_cell(&top_right, &Direction::UP), false);
                self.set(&self.edge_from_cell(&top_right, &Direction::RIGHT), false);
            },
            Some(2) => {
                self.set(&self.edge_from_cell(&Coordinate(0, n - 2), &Direction::UP), true);
                self.set(&self.edge_from_cell(&Coordinate(1, n - 1), &Direction::RIGHT), true);
            },
            Some(3) => {
                self.set(&self.edge_from_cell(&top_right, &Direction::UP), true);
                self.set(&self.edge_from_cell(&top_right, &Direction::RIGHT), true);
            },
            _ => {
                // Do nothing
            },
        }
        let bottom_right = Coordinate(n - 1, n - 1);
        match self.puzzle.grid[n - 1][n - 1] {
            Some(1) => {
                self.set(&self.edge_from_cell(&bottom_right, &Direction::DOWN), false);
                self.set(&self.edge_from_cell(&bottom_right, &Direction::RIGHT), false);
            },
            Some(2) => {
                self.set(&self.edge_from_cell(&Coordinate(n-1, n-2), &Direction::DOWN), true);
                self.set(&self.edge_from_cell(&Coordinate(n-2, n-1), &Direction::RIGHT), true);
            },
            Some(3) => {
                self.set(&self.edge_from_cell(&bottom_right, &Direction::DOWN), true);
                self.set(&self.edge_from_cell(&bottom_right, &Direction::RIGHT), true);
            },
            _ => {
                // Do nothing
            },
        }
    }
}

use super::puzzle::Puzzle;

use std::slice::Iter;
use std::time::Instant;

// (row, column)
#[derive(Clone)]
#[derive(Debug)]
#[derive(Copy)]
pub struct Coordinate (usize, usize);

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
enum Status {
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

#[derive(Clone)]
#[derive(Debug)]
pub struct Solver {
    puzzle: Puzzle,
    h_edges: Vec<Vec<Edge>>,
    v_edges: Vec<Vec<Edge>>,
    recently_affected_cells: Vec<Coordinate>,
    recently_affected_nodes: Vec<Coordinate>,
    change_flag: bool,
    status: Status,
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
            change_flag: false,
            status: Status::InProgress,
            recently_affected_cells: Vec::new(),
            recently_affected_nodes: Vec::new(),
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
                else { Option::Some(self.v_edges[c.0][c.1 - 1]) }
            },
            Direction::RIGHT => {
                if c.0 == self.puzzle.size { Option::None }
                else { Option::Some(self.v_edges[c.0 - 1][c.1]) }
            },
        }
    }

    fn set(& mut self, edge: &Edge, on: bool) {
        let actual_edge = match edge.edge_type {
            EdgeType::HORIZONTAL => self.h_edges.get_mut(edge.row).unwrap().get_mut(edge.col).unwrap(),
            EdgeType::VERTICAL => self.v_edges.get_mut(edge.row).unwrap().get_mut(edge.col).unwrap(),
        };
        let mut has_changed = false;
        if on && !actual_edge.is_on {
            actual_edge.is_on = true;
            has_changed = true;
        }
        if !on && !actual_edge.is_off {
            actual_edge.is_off = true;
            has_changed = true;
        }
        if has_changed {
            self.change_flag = true;
            match actual_edge.edge_type {
                EdgeType::HORIZONTAL => {
                    self.recently_affected_nodes.push(Coordinate(actual_edge.row, actual_edge.col));
                    self.recently_affected_nodes.push(Coordinate(actual_edge.row, actual_edge.col + 1));
                    if actual_edge.row != 0 {
                        self.recently_affected_cells.push(Coordinate(actual_edge.row - 1, actual_edge.col));
                    }
                    if actual_edge.row != self.puzzle.size {
                        self.recently_affected_cells.push(Coordinate(actual_edge.row, actual_edge.col));
                    }
                },
                EdgeType::VERTICAL => {
                    self.recently_affected_nodes.push(Coordinate(actual_edge.row, actual_edge.col));
                    self.recently_affected_nodes.push(Coordinate(actual_edge.row + 1, actual_edge.col));
                    if actual_edge.col != 0 {
                        self.recently_affected_cells.push(Coordinate(actual_edge.row, actual_edge.col - 1));
                    }
                    if actual_edge.col != self.puzzle.size {
                        self.recently_affected_cells.push(Coordinate(actual_edge.row, actual_edge.col));
                    }
                },
            }
        }
        if edge.is_on && edge.is_off {
            self.status = Status::Unsolvable;
        }
    }

    // fn satisfies_contraints(&self) -> bool {
    //     let n = self.puzzle.size;
    //     // Check if each element in each row is unique.
    //     for row in 0..n {
    //         let mut seen: HashSet<u8> = HashSet::new();
    //         for column in 0..n {
    //             if self.grid[row][column].len() != 1 {
    //                 return false;
    //             } else {
    //                 seen.insert(*self.grid[row][column].iter().next().unwrap());
    //             }
    //         }
    //         if seen.len() != n {
    //             return false;
    //         }
    //     }
    //     // Check if each element in each column is unique.
    //     for column in 0..n {
    //         let mut seen: HashSet<u8> = HashSet::new();
    //         for row in 0..n {
    //             if self.grid[row][column].len() != 1 {
    //                 return false;
    //             } else {
    //                 seen.insert(*self.grid[row][column].iter().next().unwrap());
    //             }
    //         }
    //         if seen.len() != n {
    //             return false;
    //         }
    //     }
    //     // Check if each view is respected
    //     for d in [Direction::NORTH, Direction::EAST, Direction::SOUTH, Direction::WEST] {
    //         let views: &Vec<Option<u8>> = match d {
    //             Direction::NORTH => &self.puzzle.north,
    //             Direction::EAST => &self.puzzle.east,
    //             Direction::SOUTH => &self.puzzle.south,
    //             Direction::WEST => &self.puzzle.west,
    //         };
    //         for index in 0..n {
    //             if views[index].is_none() {
    //                 continue;
    //             }
    //             let view = views[index].unwrap();
    //             let values: Vec<&u8> = get_vec(&self.grid, &d, index).iter().map(|x| x.iter().next().unwrap()).collect();
    //             if calculate_view(&values) != view {
    //                 return false;
    //             }
    //         }
    //     }
    //     return true;
    // }

    // Solve the puzzle using all non-recursive ways we know of.
    pub fn non_recursive_solve(&mut self) {
        self.initial_solve();
        self.change_flag = true;
        while self.change_flag && self.status == Status::InProgress {
            self.change_flag = false;
            // self.simple_solve();
        }
    }

    pub fn full_solve(&mut self, depth: u8, should_log: bool) -> Vec<Solver> {
        self.depth_needed = depth;
        let start = Instant::now();
        let mut solutions: Vec<Solver> = Vec::new();

        self.change_flag = true;
        while self.change_flag {
            self.non_recursive_solve();
            self.change_flag = false;
            if self.status == Status::InProgress {
                // solutions = self.depth_solve(depth, should_log);
            } else {
                solutions = Vec::new();
                solutions.push(self.clone());
            }
        }

        let duration = start.elapsed();
        let indent = " ".repeat((8 * depth) as usize);
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

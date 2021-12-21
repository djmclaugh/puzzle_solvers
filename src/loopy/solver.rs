pub mod coordinate;
mod path_tracker;
mod depth_solver;
mod direction;
mod edge;

use coordinate::Coordinate;
use direction::Direction;
use direction::HDirection;
use direction::VDirection;
use edge::Edge;
use edge::EdgeType;
use path_tracker::PathTracker;
use super::puzzle::Puzzle;

use std::time::Instant;
use std::collections::HashSet;

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

#[derive(Clone, Debug)]
pub struct Solver {
    puzzle: Puzzle,
    h_edges: Vec<Vec<Edge>>,
    v_edges: Vec<Vec<Edge>>,
    recently_affected_cells: HashSet<Coordinate>,
    recently_affected_nodes: HashSet<Coordinate>,
    recently_affected_corners: HashSet<Coordinate>,
    recently_extended_paths: HashSet<Coordinate>,
    paths: PathTracker,
    change_flag: bool,
    can_be_single_cell: bool,
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
              row.push(self.h_edges[i][j].to_string());
          }
          rows.push(row.join(""));
          row = Vec::new();
          for j in 0..n {
              row.push(self.v_edges[i][j].to_string());
              row.push(cell_to_string(&self.puzzle.grid[i][j]));
          }
          row.push(self.v_edges[i][n].to_string());
          rows.push(row.join(""));
      }
      let mut row = Vec::new();
      for j in 0..n {
          row.push(String::from(" "));
          row.push(self.h_edges[n][j].to_string());
      }
      rows.push(row.join(""));

      // Join and return rows.
      return rows.join("\n");
    }

    pub fn new(p: Puzzle) -> Solver {
        let n = p.size;
        let mut h_edges: Vec<Vec<Edge>> = Vec::new();
        let mut v_edges: Vec<Vec<Edge>> = Vec::new();
        let mut can_be_single_cell = true;

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
            can_be_single_cell,
            change_flag: false,
            status: Status::InProgress,
            recently_affected_cells: HashSet::new(),
            recently_affected_nodes: HashSet::new(),
            recently_affected_corners: HashSet::new(),
            recently_extended_paths: HashSet::new(),
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

    fn edges_from_cell(&self, c: &Coordinate) -> [Edge; 4] {
        return [
            self.edge_from_cell(c, &Direction::UP),
            self.edge_from_cell(c, &Direction::RIGHT),
            self.edge_from_cell(c, &Direction::DOWN),
            self.edge_from_cell(c, &Direction::LEFT),
        ]
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

    fn edges_from_node(&self, c: &Coordinate) -> [Option<Edge>; 4] {
        return [
            self.edge_from_node(c, &Direction::UP),
            self.edge_from_node(c, &Direction::RIGHT),
            self.edge_from_node(c, &Direction::DOWN),
            self.edge_from_node(c, &Direction::LEFT),
        ]
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

    fn nodes_from_cell(&self, c: & Coordinate) -> [Coordinate; 4] {
        return [
            Coordinate(c.0, c.1),
            Coordinate(c.0 + 1, c.1),
            Coordinate(c.0, c.1 + 1),
            Coordinate(c.0 + 1, c.1 + 1),
        ];
    }

    fn cells_from_edge(&self, e: &Edge) -> [Option<Coordinate>; 2] {
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
                return [a, b];
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
                return [a, b];
            },
        }
    }

    fn node_from_cell(&self, cell: &Coordinate, hd: &HDirection, vd: &VDirection) -> Coordinate {
        let row = match vd {
            VDirection::UP => cell.0,
            VDirection::DOWN => cell.0 + 1,
        };
        let col = match hd {
            HDirection::LEFT => cell.1,
            HDirection::RIGHT => cell.1 + 1,
        };
        return Coordinate(row, col);
    }

    fn cell_from_node(&self, n: &Coordinate, hd: &HDirection, vd: &VDirection) -> Option<Coordinate> {
        let row = match vd {
            VDirection::UP => {
                if n.0 == 0 { return Option::None; }
                n.0 - 1
            },
            VDirection::DOWN => {
                if n.0 == self.puzzle.size { return Option::None; }
                n.0
            },
        };
        let col = match hd {
            HDirection::LEFT => {
                if n.1 == 0 { return Option::None; }
                n.1 - 1
            },
            HDirection::RIGHT => {
                if n.1 == self.puzzle.size { return Option::None; }
                n.1
            },
        };
        return Option::Some(Coordinate(row, col));
    }

    fn cells_from_node(&self, n: &Coordinate) -> [Option<Coordinate>; 4] {
        return [
            self.cell_from_node(n, &HDirection::RIGHT, &VDirection::UP),
            self.cell_from_node(n, &HDirection::RIGHT, &VDirection::DOWN),
            self.cell_from_node(n, &HDirection::LEFT, &VDirection::DOWN),
            self.cell_from_node(n, &HDirection::LEFT, &VDirection::UP),
        ]
    }

    fn node_from_node(&self, n: &Coordinate, hd: &HDirection, vd: &VDirection) -> Option<Coordinate> {
        let row = match vd {
            VDirection::UP => {
                if n.0 == 0 { return Option::None; }
                n.0 - 1
            },
            VDirection::DOWN => {
                if n.0 == self.puzzle.size { return Option::None; }
                n.0 + 1
            },
        };
        let col = match hd {
            HDirection::LEFT => {
                if n.1 == 0 { return Option::None; }
                n.1 - 1
            },
            HDirection::RIGHT => {
                if n.1 == self.puzzle.size { return Option::None; }
                n.1 + 1
            },
        };
        return Option::Some(Coordinate(row, col));
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
            if actual_edge.is_off {
                self.status = Status::Unsolvable;
            }
            self.paths.add_edge(&nodes.0, &nodes.1);
            self.recently_extended_paths.insert(nodes.0);
            self.recently_extended_paths.insert(nodes.1);
            has_changed = true;
        }
        if !on && !actual_edge.is_off {
            actual_edge.is_off = true;
            if actual_edge.is_on {
                self.status = Status::Unsolvable;
            }
            self.num_off += 1;
            has_changed = true;
        }
        if has_changed {
            self.change_flag = true;
            let new_nodes = self.nodes_from_edge(edge);
            self.recently_affected_nodes.insert(new_nodes.0);
            self.recently_affected_nodes.insert(new_nodes.1);
            self.recently_affected_corners.insert(new_nodes.0);
            self.recently_affected_corners.insert(new_nodes.1);
            let new_cells = self.cells_from_edge(edge);
            for cell in new_cells {
                match cell {
                    Some(x) => {
                        let corners = self.nodes_from_cell(&x);
                        self.recently_affected_cells.insert(x);
                        for corner in corners {
                            self.recently_affected_corners.insert(corner);
                        }
                    },
                    None => {
                        // Do nothing,
                    }
                }
            }
        }
    }

    fn apply_local_single_loop_contraints(& mut self) {
        let paths: Vec<Coordinate> = self.recently_extended_paths.iter().cloned().collect();
        for node in paths {
            if self.status != Status::InProgress {
                return;
            }
            self.recently_extended_paths.remove(&node);
            let edges = self.edges_from_node(&node);
            for e in edges {
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
        let nodes: Vec<Coordinate> = self.recently_affected_nodes.iter().cloned().collect();
        for node in nodes {
            if self.status != Status::InProgress {
                return;
            }
            self.recently_affected_nodes.remove(&node);
            let edges = self.edges_from_node(&node);
            let mut real_edges: Vec<Edge> = Vec::new();
            for e in edges {
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
        let cells: Vec<Coordinate> = self.recently_affected_cells.iter().cloned().collect();
        for cell in cells {
            if self.status != Status::InProgress {
                return;
            }
            self.recently_affected_cells.remove(&cell);
            let hint = match self.puzzle.grid[cell.0][cell.1] {
                Some(x) => { x },
                None => { continue; },
            };
            let edges = self.edges_from_cell(&cell);

            let mut on_count = 0;
            let mut unknown_count = 0;

            for e in edges {
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
                for e in edges {
                    if !e.is_on && !e.is_off { self.set(&e, true); }
                }
            }
        }
    }

    fn apply_no_corner_entry(&mut self, corner: &Coordinate, hd: &HDirection, vd: &VDirection) {
        let h_corner_edge_option: Option<Edge> = self.edge_from_node(corner, &hd.to_direction());
        let v_corner_edge_option: Option<Edge> = self.edge_from_node(corner, &vd.to_direction());
        // We know that either both of the corner edges are on or both are off.
        // If either edge doesn't exist, then the other has to be off.
        if h_corner_edge_option.is_none() {
            match v_corner_edge_option {
                Some(e) => { self.set(&e, false); },
                None => {}
            }
            return;
        }
        if v_corner_edge_option.is_none() {
            match h_corner_edge_option {
                Some(e) => { self.set(&e, false); },
                None => {}
            }
            return;
        }
        // If both edges exist, then we know they have to match.
        let h_corner_edge: Edge = h_corner_edge_option.unwrap();
        let v_corner_edge: Edge = v_corner_edge_option.unwrap();
        if h_corner_edge.is_off {
            self.set(&v_corner_edge, false);
            return;
        }
        if h_corner_edge.is_on {
            self.set(&v_corner_edge, true);
            return;
        }
        if v_corner_edge.is_off {
            self.set(&h_corner_edge, false);
            return;
        }
        if v_corner_edge.is_on {
            self.set(&h_corner_edge, true);
            return;
        }
        // If the cell has a hint, we might be able to know more
        let cell = match self.cell_from_node(corner, hd, vd) {
            Some(x) => x,
            None => { return; },
        };
        let hint = match self.puzzle.grid[cell.0][cell.1] {
            Some(x) => x,
            None => { return; },
        };

        let h_other_edge = self.edge_from_cell(&cell, &vd.to_direction());
        let v_other_edge = self.edge_from_cell(&cell, &hd.to_direction());

        if hint == 0 {
            // If the hint is 0, then all the edges should already be off, nothing else to do.
        } else if hint == 1 {
            // Since this cell can have at most one edge on, we know the the corner edges must be
            // off.
            self.set(&h_corner_edge, false);
            self.set(&v_corner_edge, false);
        } else if hint == 2 {
            // If one of the other edges is already on, then the two corner edges must be off.
            if h_other_edge.is_on || v_other_edge.is_on {
                self.set(&h_corner_edge, false);
                self.set(&v_corner_edge, false);
                return;
            }
            // If one of the other edges is already off, then the two corner edges must be on.
            if h_other_edge.is_off || v_other_edge.is_off {
                self.set(&h_corner_edge, true);
                self.set(&v_corner_edge, true);
                return;
            }
            // If both other edges are unknown, then we can still know that the "no corner entry"
            // will propogate in the opposite corner.
            let opposite_corner = self.node_from_cell(&cell, hd, vd);
            self.apply_no_corner_entry(&opposite_corner, hd, vd);

            // Not only that but the "corner entry" propogate in the other two corners
            let other_corner_1 = self.node_from_cell(&cell, &hd.opposite(), vd);
            self.apply_corner_entry(&other_corner_1, &hd.opposite(), vd);
            let other_corner_2 = self.node_from_cell(&cell, hd, &vd.opposite());
            self.apply_corner_entry(&other_corner_2, hd, &vd.opposite());

        } else if hint == 3 {
            // Since we can't afford two edges being off, both corner edges must be on.
            self.set(&h_corner_edge, true);
            self.set(&v_corner_edge, true);
        }
    }

    fn apply_corner_entry(&mut self, corner: &Coordinate, hd: &HDirection, vd: &VDirection) {
        let h_corner_edge_option: Option<Edge> = self.edge_from_node(corner, &hd.to_direction());
        let v_corner_edge_option: Option<Edge> = self.edge_from_node(corner, &vd.to_direction());
        // We know that exactly one of those edges has to be on.
        // If either edge doesn't exist, then the other has to be on.
        if h_corner_edge_option.is_none() {
            match v_corner_edge_option {
                Some(e) => { self.set(&e, true); },
                None => { self.status = Status::Unsolvable; }
            }
            return;
        }
        if v_corner_edge_option.is_none() {
            match h_corner_edge_option {
                Some(e) => { self.set(&e, true); },
                None => { self.status = Status::Unsolvable; }
            }
            return;
        }
        // If both edges exist, then we know one of them has to be on.
        // So if one is off, the other has to be on.
        let h_corner_edge: Edge = h_corner_edge_option.unwrap();
        let v_corner_edge: Edge = v_corner_edge_option.unwrap();
        if h_corner_edge.is_off {
            self.set(&v_corner_edge, true);
            return;
        }
        if v_corner_edge.is_off {
            self.set(&h_corner_edge, true);
            return;
        }
        // If the cell has a hint, we might be able to know more
        let cell = match self.cell_from_node(corner, hd, vd) {
            Some(x) => x,
            None => { return; },
        };
        let hint = match self.puzzle.grid[cell.0][cell.1] {
            Some(x) => x,
            None => { return; },
        };

        let h_other_edge = self.edge_from_cell(&cell, &vd.to_direction());
        let v_other_edge = self.edge_from_cell(&cell, &hd.to_direction());

        if hint == 0 {
            // Impossible
            self.status = Status::Unsolvable;
        } else if hint == 1 {
            // Since we already know that one of the corner edges is on, we know that the two other
            // edges must be off.
            self.set(&h_other_edge, false);
            self.set(&v_other_edge, false);
        } else if hint == 2 {
            // Exactly one of the other edges must be on.
            if h_other_edge.is_off {
                self.set(&v_other_edge, true);
                return;
            }
            if v_other_edge.is_off {
                self.set(&h_other_edge, true);
                return;
            }
            // Even if we weren't able to set one of the edges, we know that one of them is on so
            // the next corner will be entered as well.
            match self.node_from_node(&corner, hd, vd) {
                Some(c) => { self.apply_corner_entry(&c, hd, vd); },
                None => {
                    // This shouldn't really happen, so it means there's a bug somewhere...
                    panic!("This shouldn't happen");
                },
            }
        } else if hint == 3 {
            // Since we know that only one of the corner edges is on, we know that the two other
            // edges must be on.
            self.set(&h_other_edge, true);
            self.set(&v_other_edge, true);
        }
    }

    fn apply_unknown_corner_with_known_complement(&mut self, corner: &Coordinate) {
        let mut unknown_count = 0;
        let mut on_count = 0;
        let edges = self.edges_from_node(&corner);
        for e in edges {
            match e {
                Some(x) => {
                    if x.is_on { on_count += 1; }
                    else if !x.is_off { unknown_count += 1; }
                },
                None => {},
            }
        }

        // If we have two on edges, then the other edges must be off. But since there's another
        // rule that takes care of this, we don't need to do anything here.
        if on_count == 2 {
            return;
        }

        // These type of arguments only work if the node has exactly two unknown edges that happen
        // to form a corner.
        // Check if there are exactly two unknown edges.
        if unknown_count != 2 {
            return;
        }
        // Check if the two unknown edges form a corner (and determine which corner).
        let is_unknown = |e: Option<Edge>| e.is_some() && !e.unwrap().is_off && !e.unwrap().is_on;
        let hd: HDirection;
        let vd: VDirection;
        if is_unknown(edges[0]) && is_unknown(edges[1]) {
            // Bottom left corner of cell
            hd = HDirection::RIGHT;
            vd = VDirection::UP;
        } else if is_unknown(edges[1]) && is_unknown(edges[2]) {
            // Top left corner of cell
            hd = HDirection::RIGHT;
            vd = VDirection::DOWN;
        } else if is_unknown(edges[2]) && is_unknown(edges[3]) {
            // Top right corner of cell
            hd = HDirection::LEFT;
            vd = VDirection::DOWN;
        } else if is_unknown(edges[3]) && is_unknown(edges[0]) {
            // Bottom right corner of cell
            hd = HDirection::LEFT;
            vd = VDirection::UP;
        } else {
            // Unknown edges form a straight line. Can't apply arguments.
            return;
        }
        if on_count == 0 {
            self.apply_no_corner_entry(corner, &hd, &vd);
        } else if on_count == 1 {
            self.apply_corner_entry(corner, &hd, &vd);
        }
    }

    fn apply_single_edge_remaining_in_corner_of_cell(&mut self, cell: &Coordinate) {
        let hint = match self.puzzle.grid[cell.0][cell.1] {
            Some(x) => x,
            None => { return; },
        };
        let edges = self.edges_from_cell(cell);
        let mut on_count = 0;
        let mut unknown_count = 0;
        for e in edges {
            if e.is_on { on_count += 1; }
            else if !e.is_off { unknown_count += 1; }
        }
        // This argument can only be applied if there are exactly 2 unknown edges and exactly 1 on
        // edge left to find.
        if unknown_count != 2 || on_count + 1 != hint {
            return;
        }
        // This argument can also only be applied if the two unknown edges form a corner.
        let hd: HDirection;
        let vd: VDirection;
        let is_unknown = |e: Edge| !e.is_off && !e.is_on;
        if is_unknown(edges[0]) && is_unknown(edges[1]) {
            hd = HDirection::RIGHT;
            vd = VDirection::UP;
        } else if is_unknown(edges[1]) && is_unknown(edges[2]) {
            hd = HDirection::RIGHT;
            vd = VDirection::DOWN;
        } else if is_unknown(edges[2]) && is_unknown(edges[3]) {
            hd = HDirection::LEFT;
            vd = VDirection::DOWN;
        } else if is_unknown(edges[3]) && is_unknown(edges[0]) {
            hd = HDirection::LEFT;
            vd = VDirection::UP;
        } else {
            // Unknown edges form a straight line. Can't apply arguments.
            return;
        }
        // We know that exactly one of those edges will be on, so we know that corner will be
        // entered.
        self.apply_corner_entry(&self.node_from_cell(cell, &hd, &vd), &hd, &vd);
    }

    fn check_if_no_outgoing_corner(&mut self, cell: &Coordinate) {
        // If a cell only has 0 or one potential outgoing corners, then the whole cell must be
        // chosen or turned off. But if the whole cell is chosen, this will form a loop, so
        // there must be no other paths started.
        let is_closed = |e: Option<Edge>| e.is_none() || e.unwrap().is_off;
        let mut closed_corners: Vec<(HDirection, VDirection)> = Vec::new();
        for hd in [HDirection::RIGHT, HDirection::LEFT] {
            for vd in [VDirection::UP, VDirection::DOWN] {
                let corner = self.node_from_cell(cell, &hd, &vd);
                let is_v_edge_off = is_closed(self.edge_from_node(&corner, &vd.to_direction()));
                let is_h_edge_off = is_closed(self.edge_from_node(&corner, &hd.to_direction()));
                if  is_v_edge_off && is_h_edge_off {
                    closed_corners.push((hd, vd));
                }
            }
        }
        if closed_corners.len() >= 3 {
            // Then all the edges of the cell must be on or all the edges of the cell must be off.
            // First check that all the edges in the cell are unknown, because if at least one is
            // known then we can set the others.
            let edges = self.edges_from_cell(cell).clone();
            for e in edges {
                if e.is_on {
                    for x in edges { self.set(&x, true); }
                    return;
                }
                if e.is_off {
                    for x in edges { self.set(&x, false); }
                    return;
                }
            }
            // If all the edges in the cell are unknown and there is at least one other path in
            // progress, then that means all these edges must be off otherwise we would have a loop
            // that won't contain all the egdes.
            if self.paths.num_paths() > 0 || !self.can_be_single_cell {
                for x in edges { self.set(&x, false); }
            }
        } else if closed_corners.len() == 2 {
            // If the cell has 2 consecutive closed corners, then 3 of the edges form a path. All
            // 3 must be on or all 3 must be off.
            let corners_are_same_column = closed_corners[0].0.eq(&closed_corners[1].0);
            let corners_are_same_row = closed_corners[0].1.eq(&closed_corners[1].1);
            if !corners_are_same_row && !corners_are_same_column {
                // The two corners are opposite, can't use this argument
                return;
            }
            let hint = match self.puzzle.grid[cell.0][cell.1] {
                Some(x) => x,
                None => { return; },
            };
            if hint == 0 {
                // Then we should have already turned all the edges off.
            } else if hint == 1 {
                // Then we should have already turned the relevant edges off since we only need
                // one corner at a time.
            } else if hint == 2 {
                // This is a contradiction since this cell can only have 0, 1, 3, or 4 edges on.
                self.status = Status::Unsolvable;
                // Turn the edges on and off to highlight where the contradiction is.
                let edges = self.edges_from_cell(cell).clone();
                for x in edges {
                    self.set(&x, false);
                    self.set(&x, true);
                }
            } else if hint == 3 {
                // Then we should have already turned the relevant edges on since we only need
                // one corner at a time.
            } else if hint == 4 {
                // Then we should have already turned all the edges off.
            }
        }

    }

    fn apply_corner_arguments(&mut self) {
        let corners: Vec<Coordinate> = self.recently_affected_corners.iter().cloned().collect();
        for corner in corners {
            if self.status != Status::InProgress {
                return;
            }
            self.recently_affected_corners.remove(&corner);
            self.apply_unknown_corner_with_known_complement(&corner);
            for cell in self.cells_from_node(&corner) {
                match cell {
                    Some(x) => {
                        self.apply_single_edge_remaining_in_corner_of_cell(&x);
                        self.check_if_no_outgoing_corner(&x);
                    },
                    None => {},
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
                        for e in edges {
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
            // println!("Before corner arguments:\n{}", self.to_string());
            self.apply_corner_arguments();
            // println!("After corner arguments:\n{}\n", self.to_string());
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

        self.non_recursive_solve();
        if self.status == Status::InProgress {
            // solutions = self.depth_solve(depth, should_log);
        } else if self.status == Status::UniqueSolution {
            solutions.push(self.clone());
        }

        let duration = start.elapsed();
        let indent = " ".repeat(8 * depth as usize);
        if should_log {
            // println!("{}", self.to_string());
            println!("\n{}Done! Total Time: {}.{:>6}", indent, duration.as_secs(), duration.as_micros() % 1000000);
            println!("{}Status: {:?}", indent, self.status);
            println!("{}Depth: {:?}", indent, depth);
            println!("{}Solutions #: {}", indent, solutions.len());
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
                    Some(2) => {
                        self.can_be_single_cell = false;
                    },
                    Some(3) => {
                        self.can_be_single_cell = false;
                    },
                    Some(4) => {
                        let c = Coordinate(i, j);
                        for d in Direction::iter() {
                            self.set(&self.edge_from_cell(&c, &d), true);
                        }
                    },
                    _ => {
                        // Do Nothing
                    }
                }
            }
        }

        let is_3 = |p: &Puzzle, i: usize, j: usize| {
            if !(i < n && j < n) {
                return false;
            }
            let hint = p.grid[i][j];
            return hint.is_some() && hint.unwrap() == 3;
        };

        for i in 0..n {
            for j in 0..n {
                if is_3(&self.puzzle, i, j) {
                    // If a 3 is next to another 3, then the edge between them is on as well as the
                    // edges on eiter side of them.
                    if is_3(&self.puzzle, i + 1, j) {
                        self.set(&self.h_edges[i][j].clone(), true);
                        self.set(&self.h_edges[i + 1][j].clone(), true);
                        self.set(&self.h_edges[i + 2][j].clone(), true);
                    }
                    if is_3(&self.puzzle, i, j + 1) {
                        self.set(&self.v_edges[i][j].clone(), true);
                        self.set(&self.v_edges[i][j + 1].clone(), true);
                        self.set(&self.v_edges[i][j + 2].clone(), true);
                    }
                    // If a 3 is diagonal to another 3, then teir edges in the their opposite
                    // corners are on.
                    if is_3(&self.puzzle, i + 1, j + 1) {
                        self.set(&self.v_edges[i][j].clone(), true);
                        self.set(&self.h_edges[i][j].clone(), true);
                        self.set(&self.v_edges[i + 1][j + 2].clone(), true);
                        self.set(&self.h_edges[i + 2][j + 1].clone(), true);
                    }
                    if j > 0 && is_3(&self.puzzle, i + 1, j - 1) {
                        self.set(&self.v_edges[i][j + 1].clone(), true);
                        self.set(&self.h_edges[i][j].clone(), true);
                        self.set(&self.v_edges[i + 1][j - 1].clone(), true);
                        self.set(&self.h_edges[i + 2][j - 1].clone(), true);
                    }
                }
            }
        }

        // Look at corners
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

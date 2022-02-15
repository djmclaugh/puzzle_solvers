pub mod coordinate;
mod inside_tracker;
mod path_tracker;
mod depth_solver;
mod corner_entry_solver;
mod initial_solver;
mod navigation;
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
    paths_endpoints_to_check: HashSet<(Coordinate, Coordinate)>,
    corner_solver_data: corner_entry_solver::CornerSolverData,
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
            corner_solver_data: corner_entry_solver::CornerSolverData::new(),
            paths: PathTracker::new(),
            num_off: 0,
            can_be_single_cell: true,
            change_flag: false,
            status: Status::InProgress,
            recently_affected_cells: HashSet::new(),
            recently_affected_nodes: HashSet::new(),
            recently_affected_corners: HashSet::new(),
            paths_endpoints_to_check: HashSet::new(),
            depth_needed: 0,
        }
    }

    fn is_value(&self, v: u8, i:usize, j:usize) -> bool {
        return self.puzzle.is_value(v, i, j);
    }
    fn is_2(&self, i:usize, j:usize) -> bool {
        return self.is_value(2, i, j);
    }
    fn is_3(&self, i:usize, j:usize) -> bool {
        return self.is_value(3, i, j);
    }

    fn is_cell_value(&self, v: u8, c: &Coordinate) -> bool {
        return self.puzzle.is_value(v, c.0, c.1);
    }
    fn is_cell_3(&self, c:&Coordinate) -> bool {
        return self.is_cell_value(3, c);
    }

    pub fn potential_degree(&self, c: &Coordinate) -> u8 {
        let mut count = 0;
        for e in self.edges_from_node(c) {
            if e.is_some() && !e.unwrap().is_off {
                count += 1;
            }
        }
        return count;
    }

    pub fn has_dead_end(&self) -> bool {
        for row in 0..(self.puzzle.size + 1) {
            for col in 0..(self.puzzle.size + 1) {
                if self.potential_degree(&Coordinate(row, col)) == 1 {
                    return true;
                }
            }
        }
        return false;
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
            let endpoints = self.paths.add_edge(&nodes.0, &nodes.1);
            match endpoints {
                Some(x) => { self.paths_endpoints_to_check.insert(x); },
                None => {},
            };
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
            let new_cells = self.cells_from_edge(edge);
            for cell in new_cells {
                match cell {
                    Some(x) => {
                        let corners = self.nodes_from_cell(&x);
                        self.recently_affected_cells.insert(x);
                        for corner in corners {
                            self.recently_affected_nodes.insert(corner);
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
        let endpoints_list: Vec<(Coordinate, Coordinate)> = self.paths_endpoints_to_check.iter().cloned().collect();
        for endpoints in endpoints_list {
            if self.status != Status::InProgress {
                return;
            }
            // If the endpoints of a path is an edge, then the path must be that edge or that edge
            // must off (otherwise it would close the loop and we would have more than two loops)
            let edge: Edge;
            if endpoints.0.0 == endpoints.1.0 {
                if endpoints.0.1 + 1 == endpoints.1.1 {
                    edge = self.edge_from_node(&endpoints.0, &Direction::RIGHT).unwrap();
                } else if endpoints.0.1 == endpoints.1.1 + 1 {
                    edge = self.edge_from_node(&endpoints.0, &Direction::LEFT).unwrap();
                } else {
                    continue;
                }
            } else if endpoints.0.1 == endpoints.1.1 {
                if endpoints.0.0 + 1 == endpoints.1.0 {
                    edge = self.edge_from_node(&endpoints.0, &Direction::DOWN).unwrap();
                } else if endpoints.0.0 == endpoints.1.0 + 1 {
                    edge = self.edge_from_node(&endpoints.0, &Direction::UP).unwrap();
                } else {
                    continue;
                }
            } else {
                continue;
            }
            if !edge.is_on && !edge.is_off {
                if self.paths.num_paths() > 1 {
                    self.set(&edge, false);
                } else {
                    // There's a chance that this is the last edge missing.
                    // Try setting the edge and see if the contraints are satisfied.
                    // let mut copy = self.clone();
                    // copy.set(&edge, true);
                    // self.set(&edge, copy.satisfies_contraints());
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

            // If two consecutive edges are different, then the corner in between the edges will be
            // used as an entry point.
            // This is usually detected using corner arguments. However, if there are two unknown
            // edges in a cell and we know that exactly one of them is set, then we know that they
            // must be different (and the corner argument wouldn't be able to tell).
            // TODO: Figure out how to generalize this argument.
            if hint == 3 && on_count == 2 && unknown_count == 2 {
                if edges[0].is_on && edges[1].is_on {
                    // Top and right edges are on, so bottom left corner will be used as entry.
                    let hd = HDirection::LEFT;
                    let vd = VDirection::DOWN;
                    self.enter_node(&self.node_from_cell(&cell, &hd, &vd), &hd, &vd);
                } else if edges[1].is_on && edges[2].is_on {
                    // Bottom and right edges are on, so top left corner will be used as entry.
                    let hd = HDirection::LEFT;
                    let vd = VDirection::UP;
                    self.enter_node(&self.node_from_cell(&cell, &hd, &vd), &hd, &vd);
                } else if edges[2].is_on && edges[3].is_on {
                    // Bottom and left edges are on, so top right corner will be used as entry.
                    let hd = HDirection::RIGHT;
                    let vd = VDirection::UP;
                    self.enter_node(&self.node_from_cell(&cell, &hd, &vd), &hd, &vd);
                } else if edges[3].is_on && edges[0].is_on {
                    // Top and left edges are on, so bottom right corner will be used as entry.
                    let hd = HDirection::RIGHT;
                    let vd = VDirection::DOWN;
                    self.enter_node(&self.node_from_cell(&cell, &hd, &vd), &hd, &vd);
                }
            }

            // TODO: Figure out how to generalize this argument.
            if hint == 2 && on_count == 0 && unknown_count == 3 {
                // Then we know that the corners opposite to the off edge will touch.
                if edges[0].is_off {
                    // Top edge off
                    if cell.0 < self.puzzle.size - 1 {
                        if cell.1 > 0 {
                            self.apply_corner_touch(&Coordinate(cell.0 + 1, cell.1 - 1), &HDirection::RIGHT, &VDirection::UP)
                        }
                        if cell.1 < self.puzzle.size - 1 {
                            self.apply_corner_touch(&Coordinate(cell.0 + 1, cell.1 + 1), &HDirection::LEFT, &VDirection::UP)
                        }
                    }
                }
                if edges[2].is_off {
                    // Bottom edge off
                    if cell.0 > 0 {
                        if cell.1 > 0 {
                            self.apply_corner_touch(&Coordinate(cell.0 - 1, cell.1 - 1), &HDirection::RIGHT, &VDirection::DOWN)
                        }
                        if cell.1 < self.puzzle.size - 1 {
                            self.apply_corner_touch(&Coordinate(cell.0 - 1, cell.1 + 1), &HDirection::LEFT, &VDirection::DOWN)
                        }
                    }
                }
                if edges[3].is_off {
                    // Left edge off
                    if cell.1 < self.puzzle.size - 1 {
                        if cell.0 > 0 {
                            self.apply_corner_touch(&Coordinate(cell.0 - 1, cell.1 + 1), &HDirection::LEFT, &VDirection::DOWN)
                        }
                        if cell.0 < self.puzzle.size - 1 {
                            self.apply_corner_touch(&Coordinate(cell.0 + 1, cell.1 + 1), &HDirection::LEFT, &VDirection::UP)
                        }
                    }
                }
                if edges[1].is_off {
                    // Right edge off
                    if cell.1 > 0 {
                        if cell.0 > 0 {
                            self.apply_corner_touch(&Coordinate(cell.0 - 1, cell.1 - 1), &HDirection::RIGHT, &VDirection::DOWN)
                        }
                        if cell.0 < self.puzzle.size - 1 {
                            self.apply_corner_touch(&Coordinate(cell.0 + 1, cell.1 - 1), &HDirection::RIGHT, &VDirection::UP)
                        }
                    }
                }
            }
        }
    }

    fn apply_corner_touch(&mut self, cell: &Coordinate, hd: &HDirection, vd: &VDirection) {
        // If we know that one of the corners of the cell is touched from the outside, then at most
        // one of the two edges from this cell that also touch that corner can be on.
        let hint = match self.puzzle.grid[cell.0][cell.1] {
            Some(x) => x,
            None => { return; },
        };
        let h_opposite_edge = self.edge_from_cell(cell, &hd.opposite().to_direction());
        let v_opposite_edge = self.edge_from_cell(cell, &vd.opposite().to_direction());
        if hint == 2 {
            if h_opposite_edge.is_off {
                self.set(&v_opposite_edge, true);
            }
            if v_opposite_edge.is_off {
                self.set(&h_opposite_edge, true);
            }
            // If the opposite corner is also touched, then we actually know that this opposite
            // touched corner must enter.
            let opposite_corner = self.node_from_cell(cell, &hd.opposite(), &vd.opposite());
            let is_on = |e: Option<Edge>| e.is_some() && e.unwrap().is_on;
            if is_on(self.edge_from_node(&opposite_corner, &hd.opposite().to_direction())) {
                match self.edge_from_node(&opposite_corner, &vd.opposite().to_direction()) {
                    Some(e) => { self.set(&e, false); },
                    None => {}
                }
            }
            if is_on(self.edge_from_node(&opposite_corner, &vd.opposite().to_direction())) {
                match self.edge_from_node(&opposite_corner, &hd.opposite().to_direction()) {
                    Some(e) => { self.set(&e, false); },
                    None => {}
                }
            }
        } else if hint == 3 {
            self.set(&h_opposite_edge, true);
            self.set(&v_opposite_edge, true);
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
            self.remove_entry_at_node(corner, &hd, &vd);
        } else if on_count == 1 {
            self.enter_node(corner, &hd, &vd);
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
        self.enter_node(&self.node_from_cell(cell, &hd, &vd), &hd, &vd);
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
        // TODO: Consider using the endpoints of paths instead of recently changed values.
        // One problem with recently changed values is that a corner argument might only apply
        // once something happens far away.
        // TODO: Endpoints are not enough!
        // Consider
        //  ┄ ┄ ┄
        // ┆·┆2┆·┆
        //    ┄ ┄
        //  0 ·┆·┆
        //    ┄ ┄
        // The first pass of corner arguments will tell us that the 2 doesn't enter from the bottom
        // and so must enter fromthe bottom right. But we can't really do anything with that just
        // now.
        // But if ever we deduce that
        //  ┄ ┄ ┄
        // ┆·┆2┆·┆
        //    ┄ ─
        //  0 ·┆·┆
        //    ┄ ┄
        // Then rechecking that the 2 doesn't enter from the bottom left tells us that the 2 must
        // enter from the bottom right and that we must have
        //  ┄ ┄ ┄
        // ┆·┆2┆·┆
        //    ┄ ─
        //  0 · ·┆
        //    ┄ ┄
        let corners: Vec<Coordinate> = self.recently_affected_corners.iter().cloned().collect();
        for corner in corners {
            if self.status != Status::InProgress {
                return;
            }
            self.recently_affected_corners.remove(&corner);

            let is_on = |e: Option<Edge>| match e { Some(e) => e.is_on, None => false };
            let is_off = |e: Option<Edge>| match e { Some(e) => e.is_off, None => true };

            // for vd in [VDirection::UP, VDirection::DOWN] {
            //     if is_on(self.edge_from_node(&corner, &vd.to_direction())) {
            //         for hd in [HDirection::RIGHT, HDirection::LEFT] {
            //             match self.cell_from_node(&corner, &hd.opposite(), &vd.opposite()) {
            //                 Some(cell) => self.apply_corner_touch(&cell, &hd, &vd),
            //                 None => {},
            //             };
            //         }
            //     }
            // }
            // for hd in [HDirection::LEFT, HDirection::RIGHT] {
            //     if is_on(self.edge_from_node(&corner, &hd.to_direction())) {
            //         for vd in [VDirection::UP, VDirection::DOWN] {
            //             match self.cell_from_node(&corner, &hd.opposite(), &vd.opposite()) {
            //                 Some(cell) => self.apply_corner_touch(&cell, &hd, &vd),
            //                 None => {},
            //             };
            //         }
            //     }
            // }

            for hd in [HDirection::LEFT, HDirection::RIGHT] {
                let e1 = self.edge_from_node(&corner, &hd.to_direction());
                for vd in [VDirection::UP, VDirection::DOWN] {
                    let e2 = self.edge_from_node(&corner, &vd.to_direction());
                    if (is_on(e1) && is_off(e2)) || (is_off(e1) && is_on(e2)) {
                        // println!("Enter {:?} {:?} {:?}", corner, hd.opposite(), vd.opposite());
                        self.enter_node(&corner, &hd.opposite(), &vd.opposite());
                        // println!("{}\n", self.to_string());
                    }
                    if (is_on(e1) && is_on(e2)) || (is_off(e1) && is_off(e2)) {
                        // println!("Remove {:?} {:?} {:?}", corner, hd.opposite(), vd.opposite());
                        self.remove_entry_at_node(&corner, &hd.opposite(), &vd.opposite());
                        // println!("{}\n", self.to_string());
                    }
                }
            }

            // self.apply_unknown_corner_with_known_complement(&corner);
            // for cell in self.cells_from_node(&corner) {
            //     match cell {
            //         Some(x) => {
            //             self.apply_single_edge_remaining_in_corner_of_cell(&x);
            //             self.check_if_no_outgoing_corner(&x);
            //         },
            //         None => {},
            //     }
            // }
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
        // Only bother doing the initial solve if no edges have been found yet.
        if self.paths.num_paths() == 0 {
            self.initial_solve();
            println!("After initial solve:\n{}", self.to_string());
        }
        self.change_flag = true;
        while self.change_flag && self.status == Status::InProgress {
            self.change_flag = false;
            self.reset_corner_data();
            self.apply_local_single_loop_contraints();
            println!("After single loop arguments:\n{}\n", self.to_string());
            self.apply_cell_constraints();
            println!("After cell arguments:\n{}\n", self.to_string());
            self.apply_corner_arguments();
            println!("After corner arguments:\n{}\n", self.to_string());
            self.apply_node_constraints();
            println!("After node arguments:\n{}\n", self.to_string());
            self.outer_inner_border_argument();
            println!("After border arguments:\n{}\n", self.to_string());
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
        // println!("After non-recursive solve:\n{}\n", self.to_string());
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
}

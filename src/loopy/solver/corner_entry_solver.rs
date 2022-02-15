use super::Solver;
use super::Status;

use super::coordinate::Coordinate;
use super::direction::HDirection;
use super::direction::VDirection;
use super::edge::Edge;

use std::collections::HashSet;

// Every node in the puzzle will end up with degree 0 or 2.
// Every node in the puzzle is the corner of some cells.
// Given a cell and one of it's corner, there are four options.
// Option 1: the node has degree 0.
// Option 2: the node has degree 2 and both edges are outside the cell
// Option 3: the node has degree 2 and both edges are inside the cell
// Option 4: the node has degree 2 and one edge is inside the cell an the other one is outside
// the cell.
// I call a node that fall into Option 4 an entry/exit corner of the cell.
// We can use this concept for many inferences.
// For example we know 1 and 3 cells have exactly 2 entry corners and that they must be adjacent.
// We know that opposite corners of a 2 cell are either both entry corners or neither are.
// We know that the entry corner of a cell is also the entry corner of the cell in that direction.
//
// This impl is a collection of such inferences.

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EntryStatus {
    EntryForSure,
    PotentialEntry,
    NotEntryForSure,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct NodeDirectionTriplet {
    node: Coordinate,
    hd: HDirection,
    vd: VDirection,
}

#[derive(Clone, Debug)]
pub struct CornerSolverData {
    pub entries_looked_at: HashSet<NodeDirectionTriplet>,
    pub removals_looked_at: HashSet<NodeDirectionTriplet>,
}

impl CornerSolverData {
    pub fn new() -> CornerSolverData {
        return CornerSolverData{
            entries_looked_at: HashSet::new(),
            removals_looked_at: HashSet::new(),
        }
    }
    fn reset(&mut self) {
        self.entries_looked_at.clear();
        self.removals_looked_at.clear();
    }
}

impl Solver {
    pub fn reset_corner_data(&mut self) {
        self.corner_solver_data.reset();
    }

    fn get_entry_status(&mut self, cell: &Coordinate, hd: &HDirection, vd: &VDirection) -> EntryStatus {
        let n = self.node_from_cell(cell, hd, vd);
        let h_edge = self.edge_from_cell(cell, &vd.to_direction());
        let v_edge = self.edge_from_cell(cell, &hd.to_direction());
        let h_outside_edge = self.edge_from_node(&n, &hd.to_direction());
        let v_outside_edge = self.edge_from_node(&n, &vd.to_direction());
        let is_on = |e: Option<Edge>| e.is_some() && e.unwrap().is_on;
        let is_off = |e: Option<Edge>| !e.is_some() || e.unwrap().is_off;

        if h_edge.is_on && v_edge.is_on { return EntryStatus::NotEntryForSure; }
        if h_edge.is_off && v_edge.is_off { return EntryStatus::NotEntryForSure; }
        if is_on(h_outside_edge) && is_on(v_outside_edge) { return EntryStatus::NotEntryForSure; }
        if is_off(h_outside_edge) && is_off(v_outside_edge) { return EntryStatus::NotEntryForSure; }

        if h_edge.is_on && v_edge.is_off { return EntryStatus::EntryForSure; }
        if h_edge.is_off && v_edge.is_on { return EntryStatus::EntryForSure; }
        if is_on(h_outside_edge) && is_off(v_outside_edge) { return EntryStatus::EntryForSure; }
        if is_off(h_outside_edge) && is_on(v_outside_edge) { return EntryStatus::EntryForSure; }

        return EntryStatus::PotentialEntry;
    }

    pub fn enter_node(&mut self, node: &Coordinate, hd: &HDirection, vd: &VDirection) {
        let h_edge = self.edge_from_node(&node, &hd.to_direction());
        let v_edge = self.edge_from_node(&node, &vd.to_direction());
        if h_edge.is_none() {
            if v_edge.is_some() { self.set(&v_edge.unwrap(), true); }
        } else if v_edge.is_none() {
            if h_edge.is_some() { self.set(&h_edge.unwrap(), true); }
        } else {
            let triple = NodeDirectionTriplet {
                node: node.clone(), hd: hd.clone(), vd: vd.clone(),
            };
            if self.corner_solver_data.entries_looked_at.contains(&triple) {
                return;
            } else {
                self.corner_solver_data.entries_looked_at.insert(triple);
                let c = self.cell_from_node(&node, &hd, &vd).unwrap();
                self.apply_entry_node_inference(&c, &hd.opposite(), &vd.opposite());
            }
        }
    }

    pub fn remove_entry_at_node(&mut self, node: &Coordinate, hd: &HDirection, vd: &VDirection) {
        let h_edge = self.edge_from_node(&node, &hd.to_direction());
        let v_edge = self.edge_from_node(&node, &vd.to_direction());
        if h_edge.is_none() {
            if v_edge.is_some() { self.set(&v_edge.unwrap(), false); }
        } else if v_edge.is_none() {
            if h_edge.is_some() { self.set(&h_edge.unwrap(), false); }
        } else {
            let triple = NodeDirectionTriplet {
                node: node.clone(), hd: hd.clone(), vd: vd.clone(),
            };
            if self.corner_solver_data.removals_looked_at.contains(&triple) {
                return;
            } else {
                self.corner_solver_data.removals_looked_at.insert(triple);
                let c = self.cell_from_node(&node, &hd, &vd).unwrap();
                self.apply_non_entry_node_inference(&c, &hd.opposite(), &vd.opposite());
            }
        }
    }

    fn apply_entry_node_inference(&mut self, cell: &Coordinate, hd: &HDirection, vd: &VDirection) {
        let opp_corner = self.node_from_cell(cell, &hd.opposite(), &vd.opposite());
        let h_edge = self.edge_from_cell(cell, &vd.to_direction());
        let v_edge = self.edge_from_cell(cell, &hd.to_direction());
        let other_h_edge = self.edge_from_cell(cell, &vd.opposite().to_direction());
        let other_v_edge = self.edge_from_cell(cell, &hd.opposite().to_direction());

        // If corner is an entry corner for the cell, then h_edge and v_edge don't match.
        if h_edge.is_on { self.set(&v_edge, false); }
        if h_edge.is_off { self.set(&v_edge, true); }
        if v_edge.is_on { self.set(&h_edge, false); }
        if v_edge.is_off { self.set(&h_edge, true); }

        // If the cell has a hint, we might be able to make more inferences.
        let hint = self.puzzle.grid[cell.0][cell.1];
        match hint {
            Some(0) | Some(4) => { self.status = Status::Unsolvable; return; },
            Some(1) => {
                self.set(&other_v_edge, false);
                self.set(&other_h_edge, false);
            },
            Some(3) => {
                self.set(&other_v_edge, true);
                self.set(&other_h_edge, true);
            },
            Some(2) => {
                if other_h_edge.is_on { self.set(&other_v_edge, false); }
                if other_h_edge.is_off { self.set(&other_v_edge, true); }
                if other_v_edge.is_on { self.set(&other_h_edge, false); }
                if other_v_edge.is_off { self.set(&other_h_edge, true); }

                self.enter_node(&opp_corner, &hd.opposite(), &vd.opposite());
            }
            _ => {},
        }

        // If there is a corner entry, then there must be a corner exit.
        let mut potential_exits: Vec<(HDirection, VDirection)> = Vec::new();
        for h in [HDirection::RIGHT, HDirection::LEFT] {
            for v in [VDirection::UP, VDirection::DOWN] {
                if !hd.eq(&h) || !vd.eq(&v) {
                    if !self.get_entry_status(cell, &h, &v).eq(&EntryStatus::NotEntryForSure) {
                        potential_exits.push((h.clone(), v.clone()));
                    }
                }
            }
        }
        if potential_exits.len() == 1 {
            // Since this is the only potential exit, then it must be an exit.
            let exit_h = potential_exits[0].0;
            let exit_v = potential_exits[0].1;
            let exit_corner = self.node_from_cell(cell, &exit_h, &exit_v);
            self.enter_node(&exit_corner, &exit_h, &exit_v);
        }
    }

    fn apply_non_entry_node_inference(&mut self, cell: &Coordinate, hd: &HDirection, vd: &VDirection) {
        let opp_corner = self.node_from_cell(cell, &hd.opposite(), &vd.opposite());
        let h_edge = self.edge_from_cell(cell, &vd.to_direction());
        let v_edge = self.edge_from_cell(cell, &hd.to_direction());
        let other_h_edge = self.edge_from_cell(cell, &vd.opposite().to_direction());
        let other_v_edge = self.edge_from_cell(cell, &hd.opposite().to_direction());

        // If corner is NOT an entry corner for the cell, then h_edge and v_edge match.
        if h_edge.is_on { self.set(&v_edge, true); }
        if h_edge.is_off { self.set(&v_edge, false); }
        if v_edge.is_on { self.set(&h_edge, true); }
        if v_edge.is_off { self.set(&h_edge, false); }

        // If the cell has a hint, we might be able to make more inferences.
        let hint = self.puzzle.grid[cell.0][cell.1];
        match hint {
            Some(1) => {
                self.set(&v_edge, false);
                self.set(&h_edge, false);
                self.enter_node(&opp_corner, &hd.opposite(), &vd.opposite());
            },
            Some(3) => {
                self.set(&v_edge, true);
                self.set(&h_edge, true);
                self.enter_node(&opp_corner, &hd.opposite(), &vd.opposite());
            },
            Some(2) => {
                if other_h_edge.is_on { self.set(&other_v_edge, true); }
                if other_h_edge.is_off { self.set(&other_v_edge, false); }
                if other_v_edge.is_on { self.set(&other_h_edge, true); }
                if other_v_edge.is_off { self.set(&other_h_edge, false); }

                // TODO: This argument could be generelized more. Basicly, if we know that we have
                // a no entry at a corner, then we know that we have at most one edge one for the
                // in the other diagonal.
                let h_neighbour = self.cell_from_cell(cell, &hd.opposite().to_direction());
                match h_neighbour {
                    Some(neighbour) => {
                        let hint = self.puzzle.grid[neighbour.0][neighbour.1];
                        if hint.is_some() && hint.unwrap() == 3 {
                            self.set(&self.edge_from_cell(&neighbour, &hd.opposite().to_direction()), true);
                            self.set(&self.edge_from_cell(&neighbour, &vd.to_direction()), true);
                        }
                    },
                    None => {},
                }

                let v_neighbour = self.cell_from_cell(cell, &vd.opposite().to_direction());
                match v_neighbour {
                    Some(neighbour) => {
                        let hint = self.puzzle.grid[neighbour.0][neighbour.1];
                        if hint.is_some() && hint.unwrap() == 3 {
                            self.set(&self.edge_from_cell(&neighbour, &vd.opposite().to_direction()), true);
                            self.set(&self.edge_from_cell(&neighbour, &hd.to_direction()), true);
                        }
                    },
                    None => {},
                }

                self.remove_entry_at_node(&opp_corner, &hd.opposite(), &vd.opposite());
                self.enter_node(&self.node_from_cell(cell, &hd.opposite(), vd), &hd.opposite(), vd);
                self.enter_node(&self.node_from_cell(cell, hd, &vd.opposite()), hd, &vd.opposite());
            }
            _ => {},
        }

        // The following inferences only work if there is at least one on edge outside this cell.
        // TODO: Could probably find a more general condition, but this case is really rare.
        if self.paths.num_paths() >= 2 {
            let mut potential_exits: Vec<(HDirection, VDirection)> = Vec::new();
            for h in [HDirection::RIGHT, HDirection::LEFT] {
                for v in [VDirection::UP, VDirection::DOWN] {
                    if !hd.eq(&h) || !vd.eq(&v) {
                        if !self.get_entry_status(cell, &h, &v).eq(&EntryStatus::NotEntryForSure) {
                            potential_exits.push((h.clone(), v.clone()));
                        }
                    }
                }
            }
            if potential_exits.len() <= 1 {
                // If there is just one potential exit, then all the edges must be off.
                self.set(&h_edge, false);
                self.set(&v_edge, false);
                self.set(&other_h_edge, false);
                self.set(&other_v_edge, false);
            }

            if potential_exits.len() == 2 {
                // If there is at least one on edge, then these two exits must be exits.
                // OR, it could be the
                if h_edge.is_on || v_edge.is_on || other_h_edge.is_on || other_v_edge.is_on {
                    let exit_h_1 = potential_exits[0].0;
                    let exit_v_1 = potential_exits[0].1;
                    let exit_corner_1 = self.node_from_cell(cell, &exit_h_1, &exit_v_1);
                    self.enter_node(&exit_corner_1, &exit_h_1, &exit_v_1);

                    let exit_h_2 = potential_exits[1].0;
                    let exit_v_2 = potential_exits[1].1;
                    let exit_corner_2 = self.node_from_cell(cell, &exit_h_2, &exit_v_2);
                    self.enter_node(&exit_corner_2, &exit_h_2, &exit_v_2);
                }
            }
        }
    }

}

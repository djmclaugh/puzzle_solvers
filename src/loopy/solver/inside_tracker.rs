use super::coordinate::Coordinate;
use std::collections::HashMap;
use super::edge::Edge;
use super::edge::EdgeType;

use std::collections::HashSet;

#[derive(Clone, Debug)]
pub struct InsideInfo {
    c: Coordinate,
    is_inside: bool,
    is_outside: bool,
    matching_cells: HashSet<Coordinate>,
    opposite_cells: HashSet<Coordinate>,
}


#[derive(Clone, Debug)]
pub struct InsideTracker {
    size: usize,
    data: Vec<Vec<InsideInfo>>,
    newly_set_cells: Vec<Coordinate>,
    pub found_contradiction: bool,
}

impl InsideTracker {
    pub fn new(size:usize) -> InsideTracker {
        let mut data: Vec<Vec<InsideInfo>> = Vec::new();
        for i in 0..size {
            let mut row = Vec::new();
            for j in 0..size {
                let mut matching_cells = HashSet::new();
                matching_cells.insert(Coordinate(i,j));
                row.push(InsideInfo {
                    c: Coordinate(i, j),
                    is_inside: false,
                    is_outside: false,
                    matching_cells,
                    opposite_cells: HashSet::new(),
                });
            }
            data.push(row);
        }
        return InsideTracker{
            size,
            data,
            newly_set_cells: Vec::new(),
            found_contradiction: false};
    }

    pub fn get_neighbours_to_check(& self) -> HashSet<Coordinate> {
        let mut result: HashSet<Coordinate> = HashSet::new();
        for c in self.newly_set_cells.clone() {
            if c.0 > 0 {
                result.insert(Coordinate(c.0 - 1, c.1));
            }
            if c.0 < self.size - 1 {
                result.insert(Coordinate(c.0 + 1, c.1));
            }
            if c.1 > 0 {
                result.insert(Coordinate(c.0, c.1 - 1));
            }
            if c.1 < self.size - 1 {
                result.insert(Coordinate(c.0, c.1 + 1));
            }
        }
        return result;
    }

    pub fn get_inferences(&mut self) -> Vec<Edge> {
        let mut result: Vec<Edge> = Vec::new();
        for c in self.newly_set_cells.clone() {
            let top = Coordinate(c.0.wrapping_sub(1), c.1);
            let top_edge = Edge {
                edge_type: EdgeType::HORIZONTAL,
                row: c.0,
                col: c.1,
                is_on: self.are_different(&c, &top),
                is_off: self.are_same(&c, &top),
            };
            if top_edge.is_on || top_edge.is_off {
                result.push(top_edge);
            }

            let bottom = Coordinate(c.0 + 1, c.1);
            let bottom_edge = Edge {
                edge_type: EdgeType::HORIZONTAL,
                row: c.0 + 1,
                col: c.1,
                is_on: self.are_different(&c, &bottom),
                is_off: self.are_same(&c, &bottom),
            };
            if bottom_edge.is_on || bottom_edge.is_off {
                result.push(bottom_edge);
            }

            let left = Coordinate(c.0, c.1.wrapping_sub(1));
            let left_edge = Edge {
                edge_type: EdgeType::VERTICAL,
                row: c.0,
                col: c.1,
                is_on: self.are_different(&c, &left),
                is_off: self.are_same(&c, &left),
            };
            if left_edge.is_on || left_edge.is_off {
                result.push(left_edge);
            }

            let right = Coordinate(c.0, c.1 + 1);
            let right_edge = Edge {
                edge_type: EdgeType::VERTICAL,
                row: c.0,
                col: c.1 + 1,
                is_on: self.are_different(&c, &right),
                is_off: self.are_same(&c, &right),
            };
            if right_edge.is_on || right_edge.is_off {
                result.push(right_edge);
            }
        }
        self.newly_set_cells.clear();
        return result;
    }

    fn are_same(&self, c1: &Coordinate, c2: &Coordinate) -> bool {
        return (self.is_inside(c1) && self.is_inside(c2)) || (self.is_outside(c1) && self.is_outside(c2));
    }

    fn are_different(&self, c1: &Coordinate, c2: &Coordinate) -> bool {
        return (self.is_inside(c1) && self.is_outside(c2)) || (self.is_inside(c1) && self.is_outside(c2));
    }

    fn is_inside(&self, c: &Coordinate) -> bool {
        return self.is_within_bounds(c) && self.data[c.0][c.1].is_inside;
    }

    fn is_outside(&self, c: &Coordinate) -> bool {
        return !self.is_within_bounds(c) || self.data[c.0][c.1].is_outside;
    }

    fn is_within_bounds(&self, c: &Coordinate) -> bool {
        return c.0 < self.size && c.1 < self.size;
    }

    fn mark_cell(&mut self, c: &Coordinate, inside: bool) {
        if !self.is_within_bounds(c) {
            if inside {
                self.found_contradiction = true;
            }
            return;
        }
        let mut is_new_information = false;
        if inside && !self.data[c.0][c.1].is_inside {
            self.data[c.0][c.1].is_inside = true;
            is_new_information = true;
        } else if !inside && !self.data[c.0][c.1].is_outside {
            self.data[c.0][c.1].is_outside = true;
            is_new_information = true;
        }
        if self.data[c.0][c.1].is_inside && self.data[c.0][c.1].is_outside {
            self.found_contradiction = true;
        } else if is_new_information {
            self.newly_set_cells.push(c.clone());
            for other in self.data[c.0][c.1].matching_cells.clone() {
                self.make_cells_same(c, &other);
            }
            for other in self.data[c.0][c.1].opposite_cells.clone() {
                self.make_cells_different(c, &other);
            }
        }
    }


    fn make_cells_same(&mut self, c1: &Coordinate, c2: &Coordinate) {
        if self.is_within_bounds(c1) && self.is_within_bounds(c2) {
            self.data[c1.0][c1.1].matching_cells.insert(c2.clone());
            self.data[c2.0][c2.1].matching_cells.insert(c1.clone());
        }

        if self.is_within_bounds(c1) && self.data[c1.0][c1.1].opposite_cells.contains(c1) {
            self.found_contradiction = true;
            return;
        }
        if self.is_within_bounds(c2) && self.data[c2.0][c2.1].opposite_cells.contains(c2) {
            self.found_contradiction = true;
            return;
        }

        if self.is_inside(c1) {
            self.mark_cell(c2, true);
        }
        if self.is_outside(c1) {
            self.mark_cell(c2, false);
        }
        if self.is_inside(c2) {
            self.mark_cell(c1, true);
        }
        if self.is_outside(c2) {
            self.mark_cell(c1, false);
        }
    }

    fn make_cells_different(&mut self, c1: &Coordinate, c2: &Coordinate) {
        if self.is_within_bounds(c1) && self.is_within_bounds(c2) {
            self.data[c1.0][c1.1].opposite_cells.insert(c2.clone());
            self.data[c2.0][c2.1].opposite_cells.insert(c1.clone());
        }

        if self.is_within_bounds(c1) && self.data[c1.0][c1.1].opposite_cells.contains(c1) {
            self.found_contradiction = true;
            return;
        }
        if self.is_within_bounds(c2) && self.data[c2.0][c2.1].opposite_cells.contains(c2) {
            self.found_contradiction = true;
            return;
        }


        if self.is_inside(c1) {
            self.mark_cell(c2, false);
        }
        if self.is_outside(c1) {
            self.mark_cell(c2, true);
        }
        if self.is_inside(c2) {
            self.mark_cell(c1, false);
        }
        if self.is_outside(c2) {
            self.mark_cell(c1, true);
        }
    }

    pub fn add_edge_info(&mut self, e: &Edge) {
        match e.edge_type {
            EdgeType::HORIZONTAL => {
                if e.is_on {
                    self.make_cells_different(&Coordinate(e.row.wrapping_sub(1), e.col), &Coordinate(e.row, e.col));
                }
                if e.is_off {
                    self.make_cells_same(&Coordinate(e.row.wrapping_sub(1), e.col), &Coordinate(e.row, e.col));
                }
            },
            EdgeType::VERTICAL => {
                if e.is_on {
                    self.make_cells_different(&Coordinate(e.row, e.col.wrapping_sub(1)), &Coordinate(e.row, e.col));
                }
                if e.is_off {
                    self.make_cells_same(&Coordinate(e.row, e.col.wrapping_sub(1)), &Coordinate(e.row, e.col));
                }
            }
        }
    }

    pub fn if_stuck_between_similar_make_similar(&mut self, c: &Coordinate) {
        let top = Coordinate(c.0.wrapping_sub(1), c.1);
        let bottom = Coordinate(c.0 + 1, c.1);
        let left = Coordinate(c.0, c.1.wrapping_sub(1));
        let right = Coordinate(c.0, c.1 + 1);
        if self.are_same(&top, &bottom) {
            self.make_cells_same(c, &top);
            self.make_cells_same(c, &bottom);
        }
        if self.are_same(&top, &left) {
            self.make_cells_same(c, &top);
            self.make_cells_same(c, &left);
        }
        if self.are_same(&top, &right) {
            self.make_cells_same(c, &top);
            self.make_cells_same(c, &right);
        }
        if self.are_same(&bottom, &left) {
            self.make_cells_same(c, &bottom);
            self.make_cells_same(c, &left);
        }
        if self.are_same(&bottom, &right) {
            self.make_cells_same(c, &bottom);
            self.make_cells_same(c, &right);
        }
        if self.are_same(&left, &right) {
            self.make_cells_same(c, &left);
            self.make_cells_same(c, &right);
        }
    }

    pub fn if_stuck_between_similar_make_different(&mut self, c: &Coordinate) {
        let top = Coordinate(c.0.wrapping_sub(1), c.1);
        let bottom = Coordinate(c.0 + 1, c.1);
        let left = Coordinate(c.0, c.1.wrapping_sub(1));
        let right = Coordinate(c.0, c.1 + 1);
        if self.are_same(&top, &bottom) {
            self.make_cells_different(c, &top);
            self.make_cells_different(c, &bottom);
        }
        if self.are_same(&top, &left) {
            self.make_cells_different(c, &top);
            self.make_cells_different(c, &left);
        }
        if self.are_same(&top, &right) {
            self.make_cells_different(c, &top);
            self.make_cells_different(c, &right);
        }
        if self.are_same(&bottom, &left) {
            self.make_cells_different(c, &bottom);
            self.make_cells_different(c, &left);
        }
        if self.are_same(&bottom, &right) {
            self.make_cells_different(c, &bottom);
            self.make_cells_different(c, &right);
        }
        if self.are_same(&left, &right) {
            self.make_cells_different(c, &left);
            self.make_cells_different(c, &right);
        }
    }
}

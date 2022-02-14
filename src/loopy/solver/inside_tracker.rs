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
    pub found_contradiction: bool,
}

impl InsideTracker {
    pub fn new(size:usize) -> InsideTracker {
        let mut data: Vec<Vec<InsideInfo>> = Vec::new();
        for i in 0..size {
            let mut row = Vec::new();
            for j in 0..size {
                row.push(InsideInfo {
                    c: Coordinate(i, j),
                    is_inside: false,
                    is_outside: false,
                    matching_cells: HashSet::new(),
                    opposite_cells: HashSet::new(),
                });
            }
            data.push(row);
        }
        return InsideTracker{size, data, found_contradiction: false};
    }

    fn is_inside(&self, c: &Coordinate) -> bool {
        return c.0 < self.size && c.1 < self.size && self.data[c.0][c.1].is_inside;
    }

    fn is_outside(&self, c: &Coordinate) -> bool {
        return c.0 > self.size || c.1 > self.size || self.data[c.0][c.1].is_outside;
    }

    fn mark_cell(&mut self, c: &Coordinate, inside: bool) {
        if c.0 > self.size || c.1 > self.size {
            if inside {
                self.found_contradiction = true;
            }
            return;
        }
        if inside {
            self.data[c.0][c.1].is_inside = true;
        } else {
            self.data[c.0][c.1].is_outside = true;
        }
        if self.data[c.0][c.1].is_inside && self.data[c.0][c.1].is_outside {
            self.found_contradiction = true;
        }
    }


    fn make_cells_same(&mut self, c1: &Coordinate, c2: &Coordinate) {
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
                    self.make_cells_different(&Coordinate(e.row - 1, e.col), &Coordinate(e.row, e.col));
                }
                if e.is_off {
                    self.make_cells_same(&Coordinate(e.row - 1, e.col), &Coordinate(e.row, e.col));
                }
            },
            EdgeType::VERTICAL => {
                if e.is_on {
                    self.make_cells_different(&Coordinate(e.row, e.col - 1), &Coordinate(e.row, e.col));
                }
                if e.is_off {
                    self.make_cells_same(&Coordinate(e.row, e.col - 1), &Coordinate(e.row, e.col));
                }
            }
        }
    }
}

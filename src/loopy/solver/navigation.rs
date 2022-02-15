use super::Solver;
use super::coordinate::Coordinate;
use super::direction::Direction;
use super::direction::HDirection;
use super::direction::VDirection;
use super::edge::Edge;
use super::edge::EdgeType;

use std::collections::HashSet;
use std::iter::FromIterator;

impl Solver {
    pub fn edge_from_cell(&self, c: &Coordinate, d: &Direction) -> Edge {
        return match d {
            Direction::UP => self.h_edges[c.0][c.1],
            Direction::DOWN => self.h_edges[c.0 + 1][c.1],
            Direction::LEFT => self.v_edges[c.0][c.1],
            Direction::RIGHT => self.v_edges[c.0][c.1 + 1],
        }
    }

    pub fn edges_from_cell(&self, c: &Coordinate) -> [Edge; 4] {
        return [
            self.edge_from_cell(c, &Direction::UP),
            self.edge_from_cell(c, &Direction::RIGHT),
            self.edge_from_cell(c, &Direction::DOWN),
            self.edge_from_cell(c, &Direction::LEFT),
        ]
    }

    pub fn edge_from_node(&self, c: &Coordinate, d: &Direction) -> Option<Edge> {
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

    pub fn edges_from_node(&self, c: &Coordinate) -> [Option<Edge>; 4] {
        return [
            self.edge_from_node(c, &Direction::UP),
            self.edge_from_node(c, &Direction::RIGHT),
            self.edge_from_node(c, &Direction::DOWN),
            self.edge_from_node(c, &Direction::LEFT),
        ]
    }

    pub fn nodes_from_edge(&self, e: &Edge) -> (Coordinate, Coordinate) {
        return match e.edge_type {
            EdgeType::HORIZONTAL => {
                (Coordinate(e.row, e.col), Coordinate(e.row, e.col + 1))
            },
            EdgeType::VERTICAL => {
                (Coordinate(e.row, e.col), Coordinate(e.row + 1, e.col))
            },
        }
    }

    pub fn node_from_edge(&self, e: &Edge, d: &Direction) -> Option<Coordinate> {
        return match e.edge_type {
            EdgeType::HORIZONTAL => {
                match d {
                    Direction::LEFT => Option::Some(Coordinate(e.row, e.col)),
                    Direction::RIGHT => Option::Some(Coordinate(e.row, e.col + 1)),
                    _ => Option::None,
                }
            },
            EdgeType::VERTICAL => {
                match d {
                    Direction::UP => Option::Some(Coordinate(e.row, e.col)),
                    Direction::DOWN => Option::Some(Coordinate(e.row + 1, e.col)),
                    _ => Option::None,
                }
            },
        }
    }

    pub fn nodes_from_cell(&self, c: & Coordinate) -> [Coordinate; 4] {
        return [
            Coordinate(c.0, c.1),
            Coordinate(c.0 + 1, c.1),
            Coordinate(c.0, c.1 + 1),
            Coordinate(c.0 + 1, c.1 + 1),
        ];
    }

    pub fn cells_from_edge(&self, e: &Edge) -> [Option<Coordinate>; 2] {
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

    pub fn node_from_cell(&self, cell: &Coordinate, hd: &HDirection, vd: &VDirection) -> Coordinate {
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

    pub fn cell_from_node(&self, n: &Coordinate, hd: &HDirection, vd: &VDirection) -> Option<Coordinate> {
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

    pub fn cells_from_node(&self, n: &Coordinate) -> [Option<Coordinate>; 4] {
        return [
            self.cell_from_node(n, &HDirection::RIGHT, &VDirection::UP),
            self.cell_from_node(n, &HDirection::RIGHT, &VDirection::DOWN),
            self.cell_from_node(n, &HDirection::LEFT, &VDirection::DOWN),
            self.cell_from_node(n, &HDirection::LEFT, &VDirection::UP),
        ]
    }

    pub fn node_from_node(&self, n: &Coordinate, hd: &HDirection, vd: &VDirection) -> Option<Coordinate> {
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

    pub fn cell_from_cell(&self, c: &Coordinate, d: &Direction) -> Option<Coordinate> {
        return match d {
            Direction::UP => if c.0 > 0 {
                Option::Some(Coordinate(c.0 - 1, c.1))
            } else {
                Option::None
            },
            Direction::DOWN => if c.0 < self.puzzle.size - 1 {
                Option::Some(Coordinate(c.0 + 1, c.1))
            } else {
                Option::None
            },
            Direction::LEFT => if c.1 > 0 {
                Option::Some(Coordinate(c.0, c.1 - 1))
            } else {
                Option::None
            },
            Direction::RIGHT => if c.1 < self.puzzle.size - 1 {
                Option::Some(Coordinate(c.0, c.1 + 1))
            } else {
                Option::None
            },
        };
    }

    pub fn outer_inner_border_argument(&mut self) {
        // This function only works if there are no dead ends.
        // If there are dead ends, return for now and let other inferences remove them first.
        if self.has_dead_end() {
            return;
        }
        // Find any top most horizontal edge that isn't off yet.
        let mut e = self.h_edges[0][0];
        for row in 0..(self.puzzle.size + 1) {
            for col in 0..self.puzzle.size {
                e = self.h_edges[row][col];
                if !e.is_off {
                    break;
                }
            }
            if !e.is_off {
                break;
            }
        }
        // If all edges are off, then just return.
        if e.is_off {
            return;
        }
        // Start by going right and whenever there is a fork, chose a clockwise turn over going
        // straight over a counter clocwise turn.
        let is_available = |o: Option<Edge>| o.is_some() && !o.unwrap().is_off;
        let mut border:Vec<Edge> = Vec::new();
        let mut directions:Vec<Direction> = Vec::new();
        border.push(e);
        let mut d = Direction::RIGHT;
        directions.push(d);
        let mut n = self.node_from_edge(&e, &d).unwrap();
        let mut next_e;
        if is_available(self.edge_from_node(&n, &d.clockwise())) {
            next_e = self.edge_from_node(&n, &d.clockwise()).unwrap();
            d = d.clockwise();
        } else if is_available(self.edge_from_node(&n, &d)) {
            next_e = self.edge_from_node(&n, &d).unwrap();
        } else if is_available(self.edge_from_node(&n, &d.counter_clockwise())) {
            next_e = self.edge_from_node(&n, &d.counter_clockwise()).unwrap();
            d = d.counter_clockwise();
        } else {
            // This should never happen.
            panic!("Edge with dead end found.");
        }
        while !next_e.eq(&border[0]) {
            border.push(next_e);
            directions.push(d);
            e = next_e;
            n = self.node_from_edge(&e, &d).unwrap();
            if is_available(self.edge_from_node(&n, &d.clockwise())) {
                next_e = self.edge_from_node(&n, &d.clockwise()).unwrap();
                d = d.clockwise();
            } else if is_available(self.edge_from_node(&n, &d)) {
                next_e = self.edge_from_node(&n, &d).unwrap();
            } else if is_available(self.edge_from_node(&n, &d.counter_clockwise())) {
                next_e = self.edge_from_node(&n, &d.counter_clockwise()).unwrap();
                d = d.counter_clockwise();
            } else {
                // This should never happen.
                panic!("Edge with dead end found.");
            }
        }

        let border_edges: HashSet<Edge> = HashSet::from_iter(border.iter().cloned());

        for i in 0..border.len() {
            let mut intersection: Vec<Edge> = Vec::new();
            let mut inner_path: Vec<Edge> = Vec::new();
            let mut has_at_least_one_non_set_intersection = false;
            let mut has_been_out = false;
            let mut has_been_back_in = false;
            let mut has_been_out_again = false;
            let mut inner_e = border[i];
            intersection.push(inner_e);
            inner_path.push(inner_e);
            let mut inner_d = directions[i];

            let potential_degree = |c: &Coordinate| {
                let mut count = 0;
                for e in self.edges_from_node(c) {
                    if e.is_some() && !e.unwrap().is_off {
                        count += 1;
                    }
                }
                return count;
            };

            let mut first_run = true;
            while first_run || !inner_e.eq(&intersection[0]) {
                first_run = false;
                let inner_n = self.node_from_edge(&inner_e, &inner_d).unwrap();
                if has_been_out == false {
                    if potential_degree(&inner_n) > 2 {
                        has_been_out = true;
                    }
                } else if has_been_back_in == false {
                    if border_edges.contains(&inner_e) {
                        has_been_back_in = true;
                    }
                } else {
                    if potential_degree(&inner_n) > 2 {
                        has_been_out_again = true;
                    }
                }
                if border_edges.contains(&inner_e) {
                    intersection.push(inner_e);
                    if !inner_e.is_on {
                        has_at_least_one_non_set_intersection = true;
                    }
                }
                if is_available(self.edge_from_node(&inner_n, &inner_d.counter_clockwise())) {
                    inner_e = self.edge_from_node(&inner_n, &inner_d.counter_clockwise()).unwrap();
                    inner_d = inner_d.counter_clockwise();
                } else if is_available(self.edge_from_node(&inner_n, &inner_d)) {
                    inner_e = self.edge_from_node(&inner_n, &inner_d).unwrap();
                } else if is_available(self.edge_from_node(&inner_n, &inner_d.clockwise())) {
                    inner_e = self.edge_from_node(&inner_n, &inner_d.clockwise()).unwrap();
                    inner_d = inner_d.clockwise();
                } else {
                    // This should never happen.
                    panic!("Edge with dead end found.");
                }
            }
            if has_been_out_again && has_at_least_one_non_set_intersection {
                // If the inner loop touched the outer loop into two non-consecutive sections, then
                // that means that the inner section divides the grid into at least two sections.
                // If there is at leas one edge in both sections, then the two sections must be
                // connected and they only way to do that is to turn on the intersection.
                let mut count = 0;
                let intersection_set: HashSet<Edge> = HashSet::from_iter(intersection.iter().cloned());
                for e in intersection.iter() {
                    if self.connected_component_has_on(&e.nodes()[0], &intersection_set) {
                        count += 1;
                    }
                    if self.connected_component_has_on(&e.nodes()[1], &intersection_set) {
                        count += 1;
                    }
                    if count > 2 {
                        break;
                    }
                }
                if count > 2 {
                    for e in intersection.iter() {
                        self.set(&e, true);
                    }
                    return;
                }
            }
        }
    }

    pub fn connected_component_has_on(&self, start: &Coordinate, edges_to_avoid: &HashSet<Edge>) -> bool {
        let mut already_added: HashSet<Coordinate> = HashSet::new();
        let mut to_visit: Vec<Coordinate> = Vec::new();
        to_visit.push(start.clone());
        already_added.insert(start.clone());
        while !to_visit.is_empty() {
            let n = to_visit.pop().unwrap();
            for edge in self.edges_from_node(&n) {
                match edge {
                    Some(e) => {
                        if !e.is_off && !edges_to_avoid.contains(&e) {
                            if e.is_on {
                                return true;
                            }
                            let other = e.other_node(&n);
                            if !already_added.contains(&other) {
                                to_visit.push(other);
                                already_added.insert(other);
                            }
                        }
                    },
                    _ => {},
                }
            }
        }
        return false;
    }
}

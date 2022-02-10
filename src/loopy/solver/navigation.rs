use super::Solver;
use super::coordinate::Coordinate;
use super::direction::Direction;
use super::direction::HDirection;
use super::direction::VDirection;
use super::edge::Edge;
use super::edge::EdgeType;

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
}

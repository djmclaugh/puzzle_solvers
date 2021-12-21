use super::coordinate::Coordinate;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EdgeType {
    HORIZONTAL,
    VERTICAL
}

#[derive(Clone, Debug, Copy)]
pub struct Edge {
    pub is_on: bool,
    pub is_off: bool,
    pub row: usize,
    pub col: usize,
    pub edge_type: EdgeType,
}

impl Edge {
    pub fn to_string(&self) -> String {
        match self.edge_type {
            EdgeType::HORIZONTAL => {
                if self.is_on {
                    if self.is_off {
                        return String::from("═");
                    } else {
                        return String::from("─");
                    }
                } else {
                    if self.is_off {
                        return String::from(" ");
                    } else {
                        return String::from("┄");
                    }
                }
            },
            EdgeType::VERTICAL => {
                if self.is_on {
                    if self.is_off {
                        return String::from("║");
                    } else {
                        return String::from("│");
                    }
                } else {
                    if self.is_off {
                        return String::from(" ");
                    } else {
                        return String::from("┆");
                    }
                }
            },
        }
    }

    pub fn nodes(&self) -> [Coordinate; 2] {
        match self.edge_type {
            EdgeType::HORIZONTAL => {
                return [Coordinate(self.row, self.col), Coordinate(self.row, self.col + 1)];
            },
            EdgeType::VERTICAL => {
                return [Coordinate(self.row, self.col), Coordinate(self.row + 1, self.col)];
            },
        }
    }

    pub fn touches_node(&self, n: &Coordinate) -> bool {
        let nodes = self.nodes();
        return n.eq(&nodes[0]) || n.eq(&nodes[1]);
    }

    pub fn common_node(&self, other: &Edge) -> Option<Coordinate> {
        let self_nodes = self.nodes();
        if other.touches_node(&self_nodes[0]) {
            return Option::Some(self_nodes[0]);
        } else if other.touches_node(&self_nodes[1]) {
            return Option::Some(self_nodes[1]);
        }
        return Option::None;
    }
}

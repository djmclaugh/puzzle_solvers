use std::slice::Iter;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Direction {
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
pub enum HDirection {
    RIGHT,
    LEFT,
}
impl HDirection {
    pub fn to_direction(&self) -> Direction {
        match self {
            Self::RIGHT => Direction::RIGHT,
            Self::LEFT => Direction::LEFT,
        }
    }
    pub fn opposite(&self) -> HDirection {
        match self {
            Self::RIGHT => Self::LEFT,
            Self::LEFT => Self::RIGHT,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VDirection {
    UP,
    DOWN,
}
impl VDirection {
    pub fn to_direction(&self) -> Direction {
        match self {
            Self::UP => Direction::UP,
            Self::DOWN => Direction::DOWN,
        }
    }
    pub fn opposite(&self) -> VDirection {
        match self {
            Self::UP => Self::DOWN,
            Self::DOWN => Self::UP,
        }
    }
}

use std::fmt;

#[derive(Clone, Debug, Copy, PartialEq, Eq, Hash)]
pub struct Triple {
    pub row: u8,
    pub col: u8,
    pub val: u8,
}

#[derive(Clone, Debug, Copy, PartialEq, Eq, Hash)]
pub struct RowCol {
    pub row: u8,
    pub col: u8,
}

#[derive(Clone, Debug, Copy, PartialEq, Eq, Hash)]
pub struct RowVal {
    pub row: u8,
    pub val: u8,
}

#[derive(Clone, Debug, Copy, PartialEq, Eq, Hash)]
pub struct ColVal {
    pub col: u8,
    pub val: u8,
}

impl Triple {
    pub fn get_row_col(&self) -> RowCol {
        return RowCol {
            row: self.row,
            col: self.col,
        };
    }

    pub fn get_row_val(&self) -> RowVal {
        return RowVal {
            row: self.row,
            val: self.val,
        };
    }

    pub fn get_col_val(&self) -> ColVal {
        return ColVal {
            col: self.col,
            val: self.val,
        };
    }

    pub fn with_row(&self, row: u8) -> Triple {
        return Triple { row, col: self.col, val: self.val };
    }

    pub fn with_col(&self, col: u8) -> Triple {
        return Triple { row: self.row, col, val: self.val };
    }

    pub fn with_val(&self, val: u8) -> Triple {
        return Triple { row: self.row, col: self.col, val };
    }
}

impl fmt::Display for Triple {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{}):{}", self.row, self.col, self.val)
    }
}

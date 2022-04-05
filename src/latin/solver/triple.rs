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
}

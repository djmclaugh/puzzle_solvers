fn char_to_cell(c: char) -> Option<u8> {
    if c == '.' || c == '·' {
      return None;
    } else {
      return Some(c.to_digit(10).unwrap() as u8);
    }
}

pub fn row<T>(grid: &Vec<Vec<T>>, index: usize) -> Vec<&T> {
    return grid[index].iter().map(|x| x).collect();
}

pub fn column<T>(grid: &Vec<Vec<T>>, index: usize) -> Vec<&T> {
    return grid.iter().map(|x| &x[index]).collect();
}

#[derive(Clone)]
#[derive(Debug)]
pub struct Puzzle {
    pub size: usize,
    pub difficulty: u8,
    pub grid: Vec<Vec<Option<u8>>>,
}

impl Puzzle {
    pub fn to_string(&self) -> String {
      let n = self.size;
      let mut rows: Vec<String> = Vec::new();

      for i in 0..n {
          let mut row = Vec::new();
          let content: Vec<String> = self.grid[i].iter().map(|hint| {
              match hint {
                Some(x) => x.to_string(),
                None => String::from("."),
            }
          }).collect();
          row.extend(content);
          rows.push(row.join(""));
      }

      // Join and return rows.
      return rows.join("\n");
    }

    pub fn is_value(&self, value:u8, i: usize, j:usize) -> bool {
        if !(i < self.size && j < self.size) {
            return false;
        }
        let hint = self.grid[i][j];
        return hint.is_some() && hint.unwrap() == value;
    }

    pub fn clone(&self) -> Puzzle {
        return Puzzle {
            size: self.grid.len(),
            difficulty: self.difficulty,
            grid: self.grid.clone(),
        }
    }

    pub fn from_grid(g: &Vec<Vec<Option<u8>>>, difficulty: u8) -> Puzzle {
        return Puzzle {
            size: g.len(), difficulty, grid: g.clone(),
        };
    }

    pub fn from_string(s: &str) -> Puzzle {
        let mut grid = Vec::new();

        let mut iter = s.trim().split("\n").peekable();

        let mut row: Option<&str> = iter.next();
        while row.is_some() {
            let row_str = row.unwrap().trim();
            let mut grid_row = Vec::new();
            for cell in row_str.chars() {
                grid_row.push(char_to_cell(cell));
            }
            grid.push(grid_row);
            row = iter.next();
        }

        return Puzzle {
            size: grid.len(),
            difficulty: 0,
            grid: grid,
        };
    }

    pub fn number_of_hints(&self) -> usize {
        let mut total = 0;

        for row in &self.grid {
            for cell in row {
                if cell.is_some() {
                    total += 1;
                }
            }
        }

        return total;
    }

    pub fn with_hints_removed(&self, hints_to_remove: &Vec<bool>, difficulty: u8) -> Puzzle {
        let n = self.grid.len();
        let mut total = 0;

        let mut grid = self.grid.clone();
        for i in 0..n {
            for j in 0..n {
                if grid[i][j].is_some() {
                    if hints_to_remove[total] {
                        grid[i][j] = None;
                    }
                    total += 1;
                }
            }
        }

        return Puzzle {
            size: grid.len(), difficulty, grid,
        };
    }
}

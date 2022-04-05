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
    pub grid: Vec<Vec<Option<u8>>>,
}

impl Puzzle {
    pub fn to_human_string(&self) -> String {
      let n = self.size;
      let mut rows: Vec<String> = Vec::new();

      for i in 0..n {
          let mut row = Vec::new();
          let content: Vec<String> = self.grid[i].iter().map(|hint| {
              match hint {
                Some(x) => x.to_string(),
                None => String::from("·"),
            }
          }).collect();
          row.extend(content);
          rows.push(row.join(""));
      }

      // Join and return rows.
      return rows.join("\n");
    }

    pub fn column(&self, i: usize) -> Vec<&Option<u8>> {
        return column(&self.grid, i);
    }

    pub fn row(&self, i: usize) -> Vec<&Option<u8>> {
        return row(&self.grid, i);
    }

    pub fn to_tatham_string_without_size(&self) -> String {
      let n = self.size;
      let mut char_list: Vec<String> = Vec::new();
      let mut counter: u8 = 0;
      for i in 0..n {
          for j in 0..n {
              match self.grid[i][j] {
                  Some(x) => {
                      if counter == 0 && i+j != 0 {
                          char_list.push(String::from("_"));
                      }
                      while counter > 0 {
                          if counter >= 26 {
                              char_list.push(String::from("z"));
                              counter -= 26;
                          } else {
                              char_list.push(String::from(('`' as u8 + counter) as char));
                              counter = 0;
                          }
                      }
                      char_list.push(x.to_string());
                  },
                  None => {
                      counter += 1;
                  },
              };
          }
      }
      while counter > 0 {
          if counter >= 26 {
              char_list.push(String::from('z'));
              counter -= 26;
          } else {
              char_list.push(String::from(('`' as u8 + counter) as char));
              counter = 0;
          }
      }
      return char_list.join("");
    }

    pub fn to_tatham_string(&self) -> String {
      let n = self.size;
      return [n.to_string() + ":", self.to_tatham_string_without_size()].concat();
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
            grid: self.grid.clone(),
        }
    }

    pub fn from_grid(g: &Vec<Vec<Option<u8>>>) -> Puzzle {
        return Puzzle {
            size: g.len(), grid: g.clone(),
        };
    }

    pub fn from_human_string(s: &str) -> Puzzle {
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

        return Puzzle { size: grid.len(), grid };
    }

    pub fn from_tatham_string(s: &str) -> Puzzle {
        let mut iter = s.trim().split(":");
        let n = iter.next().unwrap().parse::<usize>().unwrap();
        return Puzzle::from_tatham_string_with_size(n, iter.next().unwrap());
    }

    pub fn from_tatham_string_with_size(n:usize, s: &str) -> Puzzle {
        let mut grid = Vec::new();

        let mut grid_string = s.trim().chars().peekable();
        let mut counter = 0;

        for i in 0..n {
            grid.push(Vec::new());
            for j in 0..n {
                if counter == 0 {
                    let mut c = '_';
                    while c == '_' {
                        c = grid_string.next().unwrap();
                    }
                    if c.is_ascii_lowercase() {
                        counter += (c as u8) - ('`' as u8);
                    } else {
                        let mut val = c.to_digit(10).unwrap();
                        let mut p = grid_string.peek();
                        while p.is_some() && p.unwrap().is_digit(10) {
                            val *= 10;
                            val += grid_string.next().unwrap().to_digit(10).unwrap();
                            p = grid_string.peek();
                        }
                        grid[i].push(Option::Some(val as u8));
                    }
                }
                if counter > 0 {
                    grid[i].push(Option::None);
                    counter -= 1;
                }
            }
        }
        return Puzzle { size: n, grid };
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
            size: grid.len(), grid,
        };
    }
}

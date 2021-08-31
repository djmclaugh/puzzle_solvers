use crate::latin::square::Square;

// fn string_to_view(view: &str) -> Option<u8> {
//     if view == "?" {
//       return None;
//     } else {
//       return Some(view.parse::<u8>().unwrap());
//     }
// }
//
// fn string_to_cell(view: &str) -> Option<u8> {
//     if view == "?" {
//       return None;
//     } else {
//       return Some(view.parse::<u8>().unwrap() - 1);
//     }
// }

pub fn row<T>(grid: &Vec<Vec<T>>, index: usize) -> Vec<&T> {
    return grid[index].iter().map(|x| x).collect();
}

pub fn column<T>(grid: &Vec<Vec<T>>, index: usize) -> Vec<&T> {
    return grid.iter().map(|x| &x[index]).collect();
}

pub fn calculate_view(row: &Vec<&u8>) -> u8 {
    if row.len() == 0 {
        return 0;
    }
    let mut max_so_far = row[0];
    let mut seen_so_far = 1;
    for i in 1..row.len() {
        if row[i] > max_so_far {
            max_so_far = row[i];
            seen_so_far += 1;
        }
    }
    return seen_so_far;
}

// fn possibilities_to_string(p: &HashSet<u8>) -> String {
//     if p.len() == 1 {
//       return (p.iter().next().unwrap() + 1).to_string();
//     } else {
//       return String::from("?");
//     }
// }


// fn string_to_possibilities(p: &str, size: usize) -> HashSet<u8> {
//     let mut set: HashSet<u8> = HashSet::new();
//     if p == "?" {
//       for i in 0..size {
//           set.insert(i as u8);
//       }
//     } else {
//       set.insert(p.parse::<u8>().unwrap() - 1);
//     }
//     return set;
// }

pub struct Puzzle {
    pub size: usize,
    pub difficulty: u8,
    pub north: Vec<Option<u8>>,
    pub east: Vec<Option<u8>>,
    pub south: Vec<Option<u8>>,
    pub west: Vec<Option<u8>>,
    pub grid: Vec<Vec<Option<u8>>>,
}

impl Puzzle {
    pub fn to_string(&self) -> String {
      let n = self.size;
      let mut rows: Vec<String> = Vec::new();
      // North hints
      let mut row: Vec<String> = Vec::new();
      row.push(String::from(" "));
      let hints: Vec<String> = self.north.iter().map(|hint| {
          match hint {
              Some(x) => x.to_string(),
              None => String::from("?"),
          }
      }).collect();
      row.extend(hints);
      row.push(String::from(" "));

      rows.push(row.join(""));

      // Middle Rows
      for i in 0..n {
          row = Vec::new();
          row.push(match self.west[i] {
              Some(x) => x.to_string(),
              None => String::from("?"),
          });
          let content: Vec<String> = self.grid[i].iter().map(|hint| {
              match hint {
                Some(x) => x.to_string(),
                None => String::from("?"),
            }
          }).collect();
          row.extend(content);
          row.push(match self.east[i] {
              Some(x) => x.to_string(),
              None => String::from("?"),
          });
          rows.push(row.join(""));
      }

      // South hints
      row = Vec::new();
      row.push(String::from(" "));
      let hints: Vec<String> = self.south.iter().map(|hint| {
          match hint {
              Some(x) => x.to_string(),
              None => String::from("?"),
          }
      }).collect();
      row.extend(hints);
      row.push(String::from(" "));

      rows.push(row.join(""));

      // Join and return rows.
      return rows.join("\n");
    }

    pub fn clone(&self) -> Puzzle {
        return Puzzle {
            size: self.north.len(),
            difficulty: self.difficulty,
            north: self.north.clone(),
            east: self.east.clone(),
            south: self.south.clone(),
            west: self.west.clone(),
            grid: self.grid.clone(),
        }
    }

    pub fn from_square(s: &Square, difficulty: u8) -> Puzzle {
        return Puzzle::from_grid(&s.grid, difficulty);
    }

    pub fn from_grid(g: &Vec<Vec<u8>>, difficulty: u8) -> Puzzle {
        let n = g.len();
        let mut grid = Vec::new();
        for column in g {
            grid.push(column.iter().map(|cell| Some(*cell)).collect());
        }

        let mut north = Vec::new();
        let mut east = Vec::new();
        let mut south = Vec::new();
        let mut west = Vec::new();
        for i in 0..n {
            north.push(Some(calculate_view(&column(g, i))));
            let mut col = column(g, i);
            col.reverse();
            south.push(Some(calculate_view(&col)));

            west.push(Some(calculate_view(&row(g, i))));
            let mut r = row(g, i);
            r.reverse();
            east.push(Some(calculate_view(&r)));
        }


        return Puzzle {
            size: grid.len(), difficulty, north, east, south, west, grid,
        };
    }

    // pub fn from_string(s: &str) -> Puzzle {
    //     let mut grid = Vec::new();
    //
    //     let mut iter = s.trim().split("\n").peekable();
    //
    //     // North hints
    //     let first_row: &str = iter.next().unwrap().trim();
    //     let north:Vec<Option<u8>> = first_row.trim().split(" ").map(string_to_view).collect();
    //
    //     // Middle rows
    //     let mut east = Vec::new();
    //     let mut west = Vec::new();
    //     let mut row: &str = iter.next().unwrap().trim();
    //     while iter.peek().is_some() {
    //       let mut grid_row = Vec::new();
    //       let mut row_iter = row.split(" ").peekable();
    //       west.push(string_to_view(row_iter.next().unwrap()));
    //       let mut value = row_iter.next().unwrap();
    //       while row_iter.peek().is_some() {
    //           grid_row.push(string_to_cell(value));
    //           value = row_iter.next().unwrap();
    //       }
    //       east.push(string_to_view(value));
    //       grid.push(grid_row);
    //       row = iter.next().unwrap().trim();
    //     }
    //
    //     // South hints
    //     // Since iter.peek() is none, row is the last row.
    //     let south:Vec<Option<u8>> = row.trim().split(" ").map(string_to_view).collect();
    //
    //     return Puzzle {
    //         size: north.len(), north, east, south, west, grid,
    //     };
    // }

    pub fn number_of_hints(&self) -> usize {
        let mut total = 0;

        for column in &self.grid {
            for cell in column {
                if cell.is_some() {
                    total += 1;
                }
            }
        }

        for hint in &self.north {
            if hint.is_some() {
                total += 1;
            }
        }
        for hint in &self.east {
            if hint.is_some() {
                total += 1;
            }
        }
        for hint in &self.south {
            if hint.is_some() {
                total += 1;
            }
        }
        for hint in &self.west {
            if hint.is_some() {
                total += 1;
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

        let mut north = self.north.clone();
        for i in 0..n {
            if north[i].is_some() {
                if hints_to_remove[total] {
                    north[i] = None;
                }
                total += 1;
            }
        }
        let mut east = self.east.clone();
        for i in 0..n {
            if east[i].is_some() {
                if hints_to_remove[total] {
                    east[i] = None;
                }
                total += 1;
            }
        }
        let mut south = self.south.clone();
        for i in 0..n {
            if south[i].is_some() {
                if hints_to_remove[total] {
                    south[i] = None;
                }
                total += 1;
            }
        }
        let mut west = self.west.clone();
        for i in 0..n {
            if west[i].is_some() {
                if hints_to_remove[total] {
                    west[i] = None;
                }
                total += 1;
            }
        }

        return Puzzle {
            size: north.len(), difficulty, north, east, south, west, grid,
        };
    }










}

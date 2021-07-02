use std::collections::HashSet;

fn view_to_string(view: &u8) -> String {
    if *view == 0 {
      return String::from("?");
    } else {
      return view.to_string();
    }
}

fn string_to_view(view: &str) -> u8 {
    if view == "?" {
      return 0;
    } else {
      return view.parse::<u8>().unwrap();
    }
}

// fn possibilities_to_string(p: &HashSet<u8>) -> String {
//     if p.len() == 1 {
//       return (p.iter().next().unwrap() + 1).to_string();
//     } else {
//       return String::from("?");
//     }
// }

fn possibilities_to_detailed_string(p: &HashSet<u8>, size: usize) -> String {
    let mut result = String::with_capacity(size);
    for i in 0..size {
        if p.contains(&(i as u8)) {
            result.push_str(&(i+1).to_string());
        } else {
            result.push_str("_");
        }
    }
    return result;
}

fn string_to_possibilities(p: &str, size: usize) -> HashSet<u8> {
    let mut set: HashSet<u8> = HashSet::new();
    if p == "?" {
      for i in 0..size {
          set.insert(i as u8);
      }
    } else {
      set.insert(p.parse::<u8>().unwrap() - 1);
    }
    return set;
}

pub struct Puzzle {
    pub size: usize,
    pub north: Vec<u8>,
    pub east: Vec<u8>,
    pub south: Vec<u8>,
    pub west: Vec<u8>,
    pub grid: Vec<Vec<HashSet<u8>>>,
}

impl Puzzle {
    // pub fn to_string(&self) -> String {
    //   let n = self.size;
    //   let mut rows: Vec<String> = Vec::new();
    //   // North hints
    //   let mut row: Vec<String> = Vec::new();
    //   row.push(String::from(" "));
    //   let hints: Vec<String> = self.north.iter().map(view_to_string).collect();
    //   row.extend(hints);
    //   row.push(String::from(" "));
    //
    //   rows.push(row.join(" "));
    //
    //   // Middle Rows
    //   for i in 0..n {
    //       row = Vec::new();
    //       row.push(view_to_string(&self.west[i]));
    //       let content: Vec<String> = self.grid[i].iter().map(possibilities_to_string).collect();
    //       row.extend(content);
    //       row.push(view_to_string(&self.east[i]));
    //       rows.push(row.join(" "));
    //   }
    //
    //   // South hints
    //   row = Vec::new();
    //   row.push(String::from(" "));
    //   let hints: Vec<String> = self.south.iter().map(view_to_string).collect();
    //   row.extend(hints);
    //   row.push(String::from(" "));
    //
    //   rows.push(row.join(" "));
    //
    //   // Join and return rows.
    //   return rows.join("\n");
    // }

    pub fn to_detailed_string(&self) -> String {
      let n = self.size;
      let mut rows: Vec<String> = Vec::new();
      // North hints
      let mut row: Vec<String> = Vec::new();
      row.push(String::from(" "));
      let hints: Vec<String> = self.north.iter().map(view_to_string).collect();
      row.extend(hints);
      row.push(String::from(" "));

      rows.push(row.join(&(" ".repeat(n))));

      // Middle Rows
      for i in 0..n {
          row = Vec::new();
          row.push(view_to_string(&self.west[i]));
          let content: Vec<String> = self.grid[i].iter().map(|x| possibilities_to_detailed_string(x, n)).collect();
          row.extend(content);
          row.push(view_to_string(&self.east[i]));
          rows.push(row.join(" "));
      }

      // South hints
      row = Vec::new();
      row.push(String::from(" "));
      let hints: Vec<String> = self.south.iter().map(view_to_string).collect();
      row.extend(hints);
      row.push(String::from(" "));

      rows.push(row.join(&(" ".repeat(n))));

      // Join and return rows.
      return rows.join("\n");
    }
}

pub fn from_string(s: &str) -> Puzzle {
    let mut grid = Vec::new();

    let mut iter = s.trim().split("\n").peekable();

    // North hints
    let first_row: &str = iter.next().unwrap().trim();
    let north:Vec<u8> = first_row.trim().split(" ").map(string_to_view).collect();

    // Middle rows
    let mut east = Vec::new();
    let mut west = Vec::new();
    let mut row: &str = iter.next().unwrap().trim();
    while iter.peek().is_some() {
      let mut grid_row = Vec::new();
      let mut row_iter = row.split(" ").peekable();
      west.push(string_to_view(row_iter.next().unwrap()));
      let mut value = row_iter.next().unwrap();
      while row_iter.peek().is_some() {
        grid_row.push(string_to_possibilities(value, north.len()));
        value = row_iter.next().unwrap();
      }
      east.push(string_to_view(value));
      grid.push(grid_row);
      row = iter.next().unwrap().trim();
    }

    // South hints
    // Since iter.peek() is none, row is the last row.
    let south:Vec<u8> = row.trim().split(" ").map(string_to_view).collect();

    return Puzzle {
        size: north.len(),
        north: north,
        east: east,
        south: south,
        west: west,
        grid: grid,
    };
}

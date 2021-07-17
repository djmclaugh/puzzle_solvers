fn string_to_view(view: &str) -> Option<u8> {
    if view == "?" {
      return None;
    } else {
      return Some(view.parse::<u8>().unwrap());
    }
}

fn string_to_cell(view: &str) -> Option<u8> {
    if view == "?" {
      return None;
    } else {
      return Some(view.parse::<u8>().unwrap() - 1);
    }
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
    pub north: Vec<Option<u8>>,
    pub east: Vec<Option<u8>>,
    pub south: Vec<Option<u8>>,
    pub west: Vec<Option<u8>>,
    pub grid: Vec<Vec<Option<u8>>>,
}

impl Puzzle {
    pub fn from_string(s: &str) -> Puzzle {
        let mut grid = Vec::new();

        let mut iter = s.trim().split("\n").peekable();

        // North hints
        let first_row: &str = iter.next().unwrap().trim();
        let north:Vec<Option<u8>> = first_row.trim().split(" ").map(string_to_view).collect();

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
              grid_row.push(string_to_cell(value));
              value = row_iter.next().unwrap();
          }
          east.push(string_to_view(value));
          grid.push(grid_row);
          row = iter.next().unwrap().trim();
        }

        // South hints
        // Since iter.peek() is none, row is the last row.
        let south:Vec<Option<u8>> = row.trim().split(" ").map(string_to_view).collect();

        return Puzzle {
            size: north.len(), north, east, south, west, grid,
        };
    }
}

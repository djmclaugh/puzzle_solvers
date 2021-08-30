use crate::perm::permutation::PermIter;

pub fn row<T>(grid: &Vec<Vec<T>>, index: usize) -> Vec<&T> {
    return grid[index].iter().map(|x| x).collect();
}

pub fn column<T>(grid: &Vec<Vec<T>>, index: usize) -> Vec<&T> {
    return grid.iter().map(|x| &x[index]).collect();
}

pub struct Square {
    pub size: usize,
    pub grid: Vec<Vec<u8>>,
}

impl Square {
    pub fn clone(&self) -> Square {
        return Square {
            size: self.size,
            grid: self.grid.clone(),
        }
    }

    pub fn from_grid(g: &Vec<Vec<u8>>) -> Square {
        let mut grid = Vec::new();
        for column in g {
            grid.push(column.iter().map(|cell| *cell).collect());
        }

        return Square {
            size: grid.len(), grid,
        };
    }

    pub fn from_string(s: &str) -> Square {
        let mut grid = Vec::new();

        let iter = s.trim().split("\n");

        for row in iter {
          let mut grid_row = Vec::new();
          let row_iter = row.trim().split(" ");
          for value in row_iter {
              grid_row.push(value.parse::<u8>().unwrap() - 1);
          }
          grid.push(grid_row);
        }

        return Square {
            size: grid.len(), grid,
        };
    }

    pub fn to_string(&self) -> String {
        let n = self.size;
        let mut rows: Vec<String> = Vec::new();

        for i in 0..n {
            let row: Vec<String> = self.grid[i].iter().map(|x| x.to_string()).collect();
            rows.push(row.join(" "));
        }

        return rows.join("\n");
    }
}

pub struct SquareIter {
    n: u8,
    row_iters: Vec<PermIter>,
    current_rows: Vec<Vec<u8>>,
    done: bool,
}

impl SquareIter {
    pub fn new(n: u8) -> SquareIter {
        let mut iter = SquareIter{
            n,
            row_iters: vec![PermIter::new(n, &Vec::new()); n as usize],
            current_rows: vec![vec![0; n as usize]; n as usize],
            done: false
        };
        iter.find_first();

        return iter;
    }

    fn find_first(&mut self) {
        let n = self.n as usize;
        let mut restrictions: Vec<Vec<u8>> = Vec::new();

        for i in 0..n {
            self.row_iters[i] = PermIter::new(self.n, &restrictions);
            self.current_rows[i] = self.row_iters[i].next().unwrap();
            restrictions.push(self.current_rows[i].clone());
        }
    }

    fn increment(&mut self) {
        let n = self.n as usize;

        let mut depth = n - 1;
        let mut restrictions: Vec<Vec<u8>> = Vec::new();

        for i in 0..depth {
            restrictions.push(self.current_rows[i].clone());
        }

        while depth < n {
            match self.row_iters[depth].next() {
                Some(x) => {
                    self.current_rows[depth] = x;
                    restrictions.push(self.current_rows[depth].clone());
                    depth += 1;
                    if depth < n {
                        self.row_iters[depth] = PermIter::new(self.n, &restrictions);
                    }
                },
                None => {
                    if depth == 0 {
                        self.done = true;
                        return;
                    }
                    depth -= 1;
                    restrictions.pop();
                }
            }
        }
    }

}

impl Iterator for SquareIter {
    type Item = Square;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }
        let result = Square::from_grid(&self.current_rows);
        self.increment();
        return Some(result);
    }
}

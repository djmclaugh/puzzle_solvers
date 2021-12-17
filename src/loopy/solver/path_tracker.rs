use super::coordinate::Coordinate;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct PathTracker {
    endpoints: HashMap<Coordinate, (Coordinate, Coordinate)>,
    num_loops: usize,
}

impl PathTracker {
    pub fn new() -> PathTracker {
        return PathTracker{
            endpoints: HashMap::new(),
            num_loops: 0,
        }
    }

    pub fn num_paths(&self) -> usize {
        return self.num_loops + (self.endpoints.len() / 2);
    }

    pub fn has_loop(&self) -> bool {
        return self.num_loops > 0;
    }

    pub fn would_create_loop(&mut self, c1: &Coordinate, c2: &Coordinate) -> bool {
        match self.endpoints.get(c1) {
            Some(x) => {
                return (x.0.eq(c1) && x.1.eq(c2)) || (x.0.eq(c2) && x.1.eq(c1));
            },
            None => {
                return false;
            },
        }
    }

    pub fn add_edge(&mut self, c1: &Coordinate, c2: &Coordinate) {
        let p1 = self.endpoints.get(c1);
        let p2 = self.endpoints.get(c2);
        if p1.is_none() && p2.is_none() {
            // Start a new path
            self.endpoints.insert(c1.clone(), (c1.clone(), c2.clone()));
            self.endpoints.insert(c2.clone(), (c1.clone(), c2.clone()));
        } else if p1.is_some() && p2.is_none() {
            // Update p1 to include c2
            let old_path = p1.unwrap();
            let other;
            let new_path;
            match old_path.0.eq(c1) {
                    true => {
                        other = old_path.1;
                        new_path = (c2.clone(), other.clone());
                    },
                    false => {
                        other = old_path.0;
                        new_path = (other.clone(), c2.clone());
                    },
            };
            self.endpoints.remove(c1);
            self.endpoints.insert(c2.clone(), new_path);
            self.endpoints.insert(other.clone(), new_path);
        } else if p1.is_none() && p2.is_some() {
            // Update p2 to include c1
            let old_path = p2.unwrap();
            let other;
            let new_path;
            match old_path.0.eq(c2) {
                    true => {
                        other = old_path.1;
                        new_path = (c1.clone(), other.clone());
                    },
                    false => {
                        other = old_path.0;
                        new_path = (other.clone(), c1.clone());
                    },
            };
            self.endpoints.remove(c2);
            self.endpoints.insert(c1.clone(), new_path);
            self.endpoints.insert(other.clone(), new_path);
        } else {
            // Joining two paths together
            let other1 = match p1.unwrap().0.eq(c1) {
                true => p1.unwrap().1,
                false => p1.unwrap().0,
            };
            let other2 = match p2.unwrap().0.eq(c2) {
                true => p2.unwrap().1,
                false => p2.unwrap().0,
            };
            self.endpoints.remove(c1);
            self.endpoints.remove(c2);
            if (other1.eq(c2) && other2.eq(c1)) || (other1.eq(c1) && other2.eq(c2)) {
                self.num_loops += 1;
            } else {
                self.endpoints.insert(other1, (other1, other2));
                self.endpoints.insert(other2, (other1, other2));
            }
        }
    }
}

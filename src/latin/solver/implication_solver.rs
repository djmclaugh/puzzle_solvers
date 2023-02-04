use super::triple::*;

use std::fmt;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Clone, Debug, Copy, PartialEq, Eq, Hash)]
pub struct BinaryTriple {
    pub t: Triple,
    pub negated: bool,
}

impl BinaryTriple {
    pub fn opposite(&self) -> BinaryTriple {
        return BinaryTriple {
            t: self.t.clone(),
            negated: !self.negated,
        };
    }
}

impl fmt::Display for BinaryTriple {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.negated {
            true => write!(f, "¬{}", self.t),
            false => write!(f, "{}", self.t),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ImplicationsTracker {
    pub implications: HashMap<BinaryTriple, HashSet<BinaryTriple>>,
    pub row_col_map: HashMap<RowCol, HashSet<u8>>,
    pub row_val_map: HashMap<RowVal, HashSet<u8>>,
    pub col_val_map: HashMap<ColVal, HashSet<u8>>,
}

impl ImplicationsTracker {
    pub fn new(n: u8) -> Self {
        let mut row_col_map: HashMap<RowCol, HashSet<u8>> = HashMap::new();
        let mut row_val_map: HashMap<RowVal, HashSet<u8>> = HashMap::new();
        let mut col_val_map: HashMap<ColVal, HashSet<u8>> = HashMap::new();

        for i in 0..n as u8 {
            for j in 0..n as u8 {
                row_col_map.insert(RowCol{row: i, col: j}, HashSet::new());
                row_val_map.insert(RowVal{row: i, val: j}, HashSet::new());
                col_val_map.insert(ColVal{col: i, val: j}, HashSet::new());
            }
        }

        for i in 0..n {
            for j in 0..n {
                for k in 0..n {
                    let t = Triple{row: i as u8, col: j as u8, val: k as u8};
                    row_col_map.get_mut(&t.get_row_col()).unwrap().insert(t.val);
                    row_val_map.get_mut(&t.get_row_val()).unwrap().insert(t.col);
                    col_val_map.get_mut(&t.get_col_val()).unwrap().insert(t.row);
                }
            }
        }

        let mut s = ImplicationsTracker {
            implications: HashMap::new(),
            row_col_map,
            row_val_map,
            col_val_map
        };

        for i in 0..n {
            for j in 0..n {
                for k in 0..n {
                    let t = Triple{row: i as u8, col: j as u8, val: k as u8};
                    s.add_latin_square_implications(&t, n);
                }
            }
        }

        return s;
    }

    pub fn set_triple(&mut self, t: &Triple) -> HashSet<BinaryTriple> {
        return self.add_information(&BinaryTriple { t: t.clone(), negated: false });
    }

    pub fn remove_triple(&mut self, t: &Triple) -> HashSet<BinaryTriple> {
        return self.add_information(&BinaryTriple { t: t.clone(), negated: true });
    }

    fn remove(&mut self, t: &BinaryTriple) -> HashSet<BinaryTriple> {
        if !t.negated {
            self.row_col_map.get_mut(&t.t.get_row_col()).unwrap().remove(&t.t.val);
            self.row_val_map.get_mut(&t.t.get_row_val()).unwrap().remove(&t.t.col);
            self.col_val_map.get_mut(&t.t.get_col_val()).unwrap().remove(&t.t.row);
        }
        return match self.implications.remove(t) {
            Some(x) => x,
            None => HashSet::new(),
        };
    }

    fn add_information(&mut self, t: &BinaryTriple) -> HashSet<BinaryTriple> {
        // Fetch the implications and remove them from the hash map.
        let implicated_triples = self.remove(t);
        // Remove the contrapositive implications as they are redundant.
        for implicated_triple in implicated_triples.iter() {
            match self.implications.get_mut(&implicated_triple.opposite()) {
                Some(x) => { x.remove(&t.opposite()); },
                None => {},
            };
        }
        // Remove any implication that needed the oppositve of t as they won't ever be satisfied.
        let affected_triples = self.remove(&t.opposite());
        // Also remove the contrapositive implications.
        for affected_triple in affected_triples.iter() {
            match self.implications.get_mut(&affected_triple.opposite()) {
                Some(x) => { x.remove(t); }
                None => {},
            };
        }
        return implicated_triples;
    }

    fn add_implication(&mut self, a: &BinaryTriple, c: &BinaryTriple) {
        match self.implications.get_mut(a) {
            Some(x) => { x.insert(c.clone()); },
            None => {
                let mut s = HashSet::new();
                s.insert(a.clone());
                s.insert(c.clone());
                self.implications.insert(a.clone(), s);
            },
        };
        // Also add the contrapositive.
        match self.implications.get_mut(&c.opposite()) {
            Some(x) => { x.insert(a.opposite()); },
            None => {
                let mut s = HashSet::new();
                s.insert(c.opposite());
                s.insert(a.opposite());
                self.implications.insert(c.opposite(), s);
            },
        };
    }

    fn add_latin_square_implications(&mut self, t: &Triple, size: u8) {
        let bt = BinaryTriple{t: t.clone(), negated: false};
        for i in 0..size {
            if i != t.row {
                let row_t = Triple{row: i, col: t.col, val: t.val};
                self.add_implication(&bt, &BinaryTriple { t: row_t, negated: true });
            }
            if i != t.col {
                let col_t = Triple{row: t.row, col: i, val: t.val};
                self.add_implication(&bt, &BinaryTriple { t: col_t, negated: true });
            }
            if i != t.val {
                let val_t = Triple{row: t.row, col: t.col, val: i};
                self.add_implication(&bt, &BinaryTriple { t: val_t, negated: true });
            }
        }
    }

    // Collapses chains of implications.
    // For example, if A -> B and B -> C, then add A -> C to the implications.
    pub fn hypothetical_syllogism(&mut self) {
        let copy = self.implications.clone();
        for a in copy.keys() {
            let all_implications = self.implications.get_mut(&a).unwrap();
            let mut new_implications = all_implications.clone();
            while !new_implications.is_empty() {
                let drained: HashSet<BinaryTriple> = new_implications.drain().collect();
                for c in drained {
                    let c_implications = match copy.get(&c) {
                        Some(x) => x,
                        None => continue,
                    };
                    for new in c_implications {
                        if all_implications.insert(new.clone()) {
                            new_implications.insert(new.clone());
                        }
                    }
                }
            }
        }
    }

    // If T implies the negation of all but one value in a cell, then T implies that value.
    // Symetrically for row/val and col/val.
    pub fn disjunctive_syllogism(&mut self) -> bool {
        let mut did_add_something = false;
        let copy = self.implications.clone();
        for a in copy.keys() {
            let implications = self.implications.get_mut(&a).unwrap();
            let negations: HashSet<Triple> = implications.iter().filter(|x| x.negated).map(|x| x.t.clone()).collect();
            for (row_col, mut vals) in self.row_col_map.clone() {
                for n in negations.iter().filter(|x| x.get_row_col().eq(&row_col)) {
                    vals.remove(&n.val);
                }
                if vals.len() == 1 {
                    let val = vals.drain().next().unwrap();
                    let t = Triple { row: row_col.row, col: row_col.col, val}; 
                    did_add_something |= implications.insert(BinaryTriple{t, negated: false});
                }
            }

            for (row_val, mut cols) in self.row_val_map.clone() {
                for n in negations.iter().filter(|x| x.get_row_val().eq(&row_val)) {
                    cols.remove(&n.col);
                }
                if cols.len() == 1 {
                    let col = cols.drain().next().unwrap();
                    let t = Triple { row: row_val.row, col, val: row_val.val}; 
                    did_add_something |= implications.insert(BinaryTriple{t, negated: false});
                }
            }

            for (col_val, mut rows) in self.col_val_map.clone() {
                for n in negations.iter().filter(|x| x.get_col_val().eq(&col_val)) {
                    rows.remove(&n.row);
                }
                if rows.len() == 1 {
                    let row = rows.drain().next().unwrap();
                    let t = Triple { row, col: col_val.col, val: col_val.val}; 
                    did_add_something |= implications.insert(BinaryTriple{t, negated: false});
                }
            }
        }
        return did_add_something;
    }

    pub fn get_contradictions(&self) -> HashSet<BinaryTriple> {
        let mut contradictions = HashSet::new();
        for (key, val) in self.implications.iter() {
            if val.contains(&key.opposite()) {
                contradictions.insert(key.clone());
            }
        }
        return contradictions;
    }

    pub fn get_disjunction_elimination_inferences(&self, size: u8) -> HashSet<BinaryTriple> {
        let mut forced = HashSet::new();
        for i in 0..size {
            for j in 0..size {
                let mut common_in_row_col: Option<HashSet<BinaryTriple>> = None;
                let mut common_in_row_val: Option<HashSet<BinaryTriple>> = None;
                let mut common_in_col_val: Option<HashSet<BinaryTriple>> = None;
                for k in 0..size {
                    let row_col_t = BinaryTriple { t: Triple{row: i, col: j, val: k }, negated: false };
                    match self.implications.get(&row_col_t) {
                        Some(x) => {
                            common_in_row_col = match common_in_row_col {
                                Some(y) => Some(&y & &x),
                                None => Some(x.clone()),
                            };
                        },
                        None => {},
                    };

                    let row_val_t = BinaryTriple { t: Triple{row: i, col: k, val: j }, negated: false };
                    match self.implications.get(&row_val_t) {
                        Some(x) => {
                            common_in_row_val = match common_in_row_val {
                                Some(y) => Some(&y & &x),
                                None => Some(x.clone()),
                            };
                        },
                        None => {},
                    };

                    let col_val_t = BinaryTriple { t: Triple{row: k, col: i, val: j }, negated: false };
                    match self.implications.get(&col_val_t) {
                        Some(x) => {
                            common_in_col_val = match common_in_col_val {
                                Some(y) => Some(&y & &x),
                                None => Some(x.clone()),
                            };
                        },
                        None => {},
                    };
                }
                match common_in_row_col {
                    Some(x) => { forced = &forced | &x; },
                    None => {},
                }
                match common_in_row_val {
                    Some(x) => { forced = &forced | &x; },
                    None => {},
                }
                match common_in_col_val {
                    Some(x) => { forced = &forced | &x; },
                    None => {},
                }
            }
        }
        return forced;
    }

}

impl fmt::Display for ImplicationsTracker {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (key, value) in self.implications.iter() {
            write!(f, "{} => :\n\t", key)?;
            for v in value {
                write!(f, "{} ", v)?;
            }
            write!(f, "\n\n")?;
        }
        Ok(())
    }
}

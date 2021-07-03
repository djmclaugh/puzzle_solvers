use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Clone)]
#[derive(Debug)]
#[derive(Copy)]
#[derive(Hash)]
#[derive(PartialEq)]
#[derive(Eq)]
pub struct Possibility (pub u8, pub u8, pub u8);

#[derive(Clone)]
#[derive(Hash)]
#[derive(PartialEq)]
#[derive(Eq)]
// Represents the first two positions of a possibility triple.
pub struct PosPair (u8, u8);

#[derive(Clone)]
#[derive(Hash)]
#[derive(PartialEq)]
#[derive(Eq)]
// Represents the first nad last positions of a possibility triple.
pub struct RowPair (u8, u8);

#[derive(Clone)]
#[derive(Hash)]
#[derive(PartialEq)]
#[derive(Eq)]
// Represents the last two positions of a possibility triple.
pub struct ColPair (u8, u8);

impl Possibility {
    pub fn pos(&self) -> PosPair {
        return PosPair(self.0, self.1);
    }

    pub fn row(&self) -> RowPair {
        return RowPair(self.0, self.2);
    }

    pub fn col(&self) -> ColPair {
        return ColPair(self.1, self.2);
    }
}

pub struct Graph {
    n: u8,
    nodes: Vec<Possibility>,
    forced_choices: HashMap<Possibility, HashSet<Possibility>>,
    choices_that_force: HashMap<Possibility, HashSet<Possibility>>,
    // forced_removals: HashMap<Possibility, HashSet<Possibility>>,
    // choices_that_remove: HashMap<Possibility, HashSet<Possibility>>,
    possibilities_by_pos_pair: HashMap<PosPair, HashSet<Possibility>>,
    possibilities_by_row_pair: HashMap<RowPair, HashSet<Possibility>>,
    possibilities_by_col_pair: HashMap<ColPair, HashSet<Possibility>>,
    //possibilities_by_value: HashMap<u8, HashSet<Possibility>>,
    //possibilities_by_row: HashMap<u8, HashSet<Possibility>>,
    //possibilities_by_col: HashMap<u8, HashSet<Possibility>>,
}

impl Graph {
    pub fn new(grid: &Vec<Vec<HashSet<u8>>>) -> Graph {
        let n = grid.len();

        let mut nodes: Vec<Possibility> = Vec::new();
        let mut forced_choices: HashMap<Possibility, HashSet<Possibility>> = HashMap::new();
        let mut choices_that_force: HashMap<Possibility, HashSet<Possibility>> = HashMap::new();
        let mut possibilities_by_pos_pair: HashMap<PosPair, HashSet<Possibility>> = HashMap::new();
        let mut possibilities_by_row_pair: HashMap<RowPair, HashSet<Possibility>> = HashMap::new();
        let mut possibilities_by_col_pair: HashMap<ColPair, HashSet<Possibility>> = HashMap::new();
        //let mut possibilities_by_value: HashMap<u8, HashSet<Possibility>> = HashMap::new();
        //let mut possibilities_by_row: HashMap<u8, HashSet<Possibility>> = HashMap::new();
        //let mut possibilities_by_col: HashMap<u8, HashSet<Possibility>> = HashMap::new();

        for row in 0..n {
            for col in 0..n {
                for value in &grid[row][col] {
                    let p = Possibility(row as u8, col as u8, *value as u8);
                    nodes.push(p);
                    let mut implications = HashSet::new();
                    implications.insert(p);
                    forced_choices.insert(p, implications);
                    let mut implied_by = HashSet::new();
                    implied_by.insert(p);
                    choices_that_force.insert(p, implied_by);
                    possibilities_by_pos_pair.entry(p.pos()).or_insert(HashSet::new()).insert(p);
                    possibilities_by_row_pair.entry(p.row()).or_insert(HashSet::new()).insert(p);
                    possibilities_by_col_pair.entry(p.col()).or_insert(HashSet::new()).insert(p);
                    //possibilities_by_value.entry(p.2).or_insert(HashSet::new()).insert(p);
                    //possibilities_by_row.entry(p.0).or_insert(HashSet::new()).insert(p);
                    //possibilities_by_col.entry(p.1).or_insert(HashSet::new()).insert(p);
                }
            }
        }
        return Graph {
            n: (n as u8),
            nodes,
            forced_choices,
            choices_that_force,
            possibilities_by_pos_pair,
            possibilities_by_row_pair,
            possibilities_by_col_pair,
            //possibilities_by_value,
            //possibilities_by_row,
            //possibilities_by_col,
        };
    }

    // p1 implies p2.
    pub fn add_implication(&mut self, p1: &Possibility, p2: &Possibility) -> bool {
        if self.forced_choices.get(p1).unwrap().contains(p2) && self.choices_that_force.get(p2).unwrap().contains(p1) {
            // We already know that p1 implies p2.
            return false;
        }
        self.forced_choices.get_mut(p1).unwrap().insert(p2.clone());
        self.choices_that_force.get_mut(p2).unwrap().insert(p1.clone());

        // Any node that implies p1 must therefore also imply p2.
        for implier in self.choices_that_force.get(p1).unwrap().clone() {
            self.add_implication(&implier, p2);
        }

        // Any node that implied p2 must therefore also be implied by p1.
        for implied in self.forced_choices.get(p2).unwrap().clone() {
            self.add_implication(p1, &implied);
        }

        return true;
    }

    // A node is maximal if no other node implies all of the same nodes and at least one move.
    // A node is maximal if none of its impliers have a larger implied set.
    // Since a node's implied set is a subset of its impliers inmplied set, it suffices to check
    // that none of its impliers implied sets are larger than the nodes implied set.
    // This function returns one of the potentially multiple maximal nodes that imply p.
    pub fn find_maximal_implier(&self, p: &Possibility) -> Possibility {
        let mut current = p;
        let mut found_larger = true;
        while found_larger {
            found_larger = false;
            let num_implied = self.forced_choices.get(current).unwrap().len();
            for implier in self.choices_that_force.get(current).unwrap() {
                let implier_num_implied = self.forced_choices.get(implier).unwrap().len();
                if implier_num_implied > num_implied {
                    current = implier;
                    found_larger = true;
                    break;
                }
            }
        }
        return current.clone();
    }

    // Returns all nodes that are maximal in terms of the set of nodes they imply.
    // A node is maximal if no other node implies all of the same nodes and at least one move.
    // A node is maximal if all of its impliers has a larger implied set.
    pub fn maximal_impliers(&self) -> Vec<HashSet<Possibility>> {
        let mut nodes_remaining: HashSet<Possibility> = self.nodes.iter().map(|x| *x).collect();
        let mut maximal_classes: Vec<HashSet<Possibility>> = Vec::new();
        while !nodes_remaining.is_empty() {
            let next = nodes_remaining.iter().next().unwrap();
            let maximal = self.find_maximal_implier(next);
            maximal_classes.push(self.choices_that_force.get(&maximal).unwrap().clone());
            nodes_remaining = nodes_remaining.difference(self.forced_choices.get(&maximal).unwrap()).map(|x| *x).collect();
        }
        return maximal_classes;
    }

    pub fn find_direct_implications(&mut self, p: &Possibility) {
        // Assume that p = (x, y, z) is a chosen possibility, what other possibilities must be
        // chosen?
        let (x, y, z) = (p.0, p.1, p.2);
        let mut to_add: Vec<Possibility> = Vec::new();
        // If there are only 2 triples of the form (x, i, *) and one of them is (x, i, z), then
        // the other triple must be chosen.
        // Same argument can be repeated for each choice of fixed position and each choice of
        // iterated position for a total of 3 * 2  = 6 times.
        for i in 0..self.n {
            let mut set = self.possibilities_by_pos_pair.get(&PosPair(x, i)).unwrap();
            if i != y && set.len() == 2 && set.contains(&Possibility(x, i, z)) {
                to_add.push(set.iter().find(|other| other.2 != z).unwrap().clone());
            }
            set = self.possibilities_by_row_pair.get(&RowPair(x, i)).unwrap();
            if i != z && set.len() == 2 && set.contains(&Possibility(x, y, i)) {
                to_add.push(set.iter().find(|other| other.1 != y).unwrap().clone());
            }

            set = self.possibilities_by_pos_pair.get(&PosPair(i, y)).unwrap();
            if i != x && set.len() == 2 && set.contains(&Possibility(i, y, z)) {
                to_add.push(set.iter().find(|other| other.2 != z).unwrap().clone());
            }
            set = self.possibilities_by_col_pair.get(&ColPair(y, i)).unwrap();
            if i != z && set.len() == 2 && set.contains(&Possibility(x, y, i)) {
                to_add.push(set.iter().find(|other| other.0 != x).unwrap().clone());
            }

            set = self.possibilities_by_row_pair.get(&RowPair(i, z)).unwrap();
            if i != x && set.len() == 2 && set.contains(&Possibility(i, y, z)) {
                to_add.push(set.iter().find(|other| other.1 != y).unwrap().clone());
            }
            set = self.possibilities_by_col_pair.get(&ColPair(y, z)).unwrap();
            if i != y && set.len() == 2 && set.contains(&Possibility(x, i, z)) {
                to_add.push(set.iter().find(|other| other.0 != x).unwrap().clone());
            }
        }

        for i in to_add {
            self.add_implication(p, &i);
        }
    }

    pub fn find_indirect_implications(&mut self, p: &Possibility) -> bool {
        // Indead of looking at the effects of just this node. We'll look at the effects of this
        // node and all of its implied nodes.
        let implications: &HashSet<Possibility> = self.forced_choices.get(p).unwrap();

        let mut remaining_by_pos_pair = self.possibilities_by_pos_pair.clone();
        let mut remaining_by_row_pair = self.possibilities_by_row_pair.clone();
        let mut remaining_by_col_pair = self.possibilities_by_col_pair.clone();

        // For each implied node, remove all possibilities with two matching fields.
        for implied in implications {
            for to_remove in remaining_by_pos_pair.get(&implied.pos()).unwrap() {
                remaining_by_row_pair.get_mut(&to_remove.row()).unwrap().remove(to_remove);
                remaining_by_col_pair.get_mut(&to_remove.col()).unwrap().remove(to_remove);
            }
            remaining_by_pos_pair.get_mut(&implied.pos()).unwrap().clear();

            for to_remove in remaining_by_row_pair.get(&implied.row()).unwrap() {
                remaining_by_pos_pair.get_mut(&to_remove.pos()).unwrap().remove(to_remove);
                remaining_by_col_pair.get_mut(&to_remove.col()).unwrap().remove(to_remove);
            }
            remaining_by_row_pair.get_mut(&implied.row()).unwrap().clear();

            for to_remove in remaining_by_col_pair.get(&implied.col()).unwrap() {
                remaining_by_pos_pair.get_mut(&to_remove.pos()).unwrap().remove(to_remove);
                remaining_by_row_pair.get_mut(&to_remove.row()).unwrap().remove(to_remove);
            }
            remaining_by_col_pair.get_mut(&implied.col()).unwrap().clear();
        }

        let mut to_add: Vec<Possibility> = Vec::new();

        // Look at what remaining by pos/row/col pair.
        // If there's only one choice left, then it must be chosen.
        for remaining in remaining_by_pos_pair.values() {
            if remaining.len() == 1 {
                to_add.push(*remaining.iter().next().unwrap());
            }
        }
        for remaining in remaining_by_row_pair.values() {
            if remaining.len() == 1 {
                to_add.push(*remaining.iter().next().unwrap());
            }
        }
        for remaining in remaining_by_col_pair.values() {
            if remaining.len() == 1 {
                to_add.push(*remaining.iter().next().unwrap());
            }
        }

        //println!("More implications found: {:?}", to_add);
        let mut found_implication = false;
        for extra in to_add {
            found_implication = self.add_implication(p, &extra) || found_implication;
        }
        return found_implication;
    }

    pub fn find_all_direct_implications(&mut self) {
        let nodes = self.nodes.clone();
        for p in nodes {
            self.find_direct_implications(&p);
        }
    }

    pub fn find_impossibilities(&mut self) -> Vec<Possibility> {
        let mut to_remove = Vec::new();
        self.find_all_direct_implications();
        // Just need to look at one element form each maximal class.
        let mut maximals: Vec<Possibility> = self.maximal_impliers().iter().map(|x| *x.iter().next().unwrap()).collect();
        // let mut found_implication = true;
        // while found_implication {
        //     found_implication = false;
        //     //println!("going deeper");
            for p in maximals.iter() {
                // found_implication = self.find_indirect_implications(p) || found_implication;
                self.find_indirect_implications(p);
            }
            maximals = self.maximal_impliers().iter().map(|x| *x.iter().next().unwrap()).collect();
        // }
        //println!("Number of maximal classes: {}", maximals.len());
        for p in maximals.iter() {
            let set = self.forced_choices.get(p).unwrap();
            if !is_valid_possibility_set(set) {
                let mut class: Vec<Possibility> = self.choices_that_force.get(p).unwrap().iter().map(|x| *x).collect();
                to_remove.append(&mut class);
            }
        }
        return to_remove;
    }

    fn to_implied_grid(&self, assumption: Possibility) -> Vec<Vec<HashSet<u8>>> {
        let mut grid: Vec<Vec<HashSet<u8>>> = Vec::new();
        for i in 0..(self.n as usize) {
            grid.push(Vec::new());
            for _j in 0..self.n {
                grid[i].push(HashSet::new());
            }
        }
        for p in &self.nodes {
            grid[p.0 as usize][p.1 as usize].insert(p.2);
        }
        for p in self.forced_choices.get(&assumption).unwrap() {
            grid[p.0 as usize][p.1 as usize].clear();
            grid[p.0 as usize][p.1 as usize].insert(p.2);
        }
        return grid;
    }

    pub fn maximal_implied_grids(&self) -> Vec<(HashSet<Possibility>, Vec<Vec<HashSet<u8>>>)> {
        let mut result: Vec<(HashSet<Possibility>, Vec<Vec<HashSet<u8>>>)> = Vec::new();
        for c in self.maximal_impliers() {
            let representative = *c.iter().next().unwrap();
            result.push((c, self.to_implied_grid(representative)));
        }
        return result;
    }
}

// O(n) (where n is the size of the set)
fn is_valid_possibility_set(set: &HashSet<Possibility>) -> bool {
    let mut possibility_with_pos_pair: HashMap<PosPair, &Possibility> = HashMap::new();
    let mut possibility_with_row_pair: HashMap<RowPair, &Possibility> = HashMap::new();
    let mut possibility_with_col_pair: HashMap<ColPair, &Possibility> = HashMap::new();
    for p in set {
        match possibility_with_pos_pair.get(&p.pos()) {
            Some(_x) => {
                return false;
            },
            None => { possibility_with_pos_pair.insert(p.pos(), &p); }
        };
        match possibility_with_row_pair.get(&p.row()) {
            Some(_x) => {
                return false;
            },
            None => { possibility_with_row_pair.insert(p.row(), &p); }
        };
        match possibility_with_col_pair.get(&p.col()) {
            Some(_x) => {
                return false;
            },
            None => { possibility_with_col_pair.insert(p.col(), &p); }
        };
    }
    return true;
}

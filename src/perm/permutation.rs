use std::collections::HashSet;
use rand::seq::SliceRandom;
use rand::thread_rng;

// Note this doesn't generate the permutations unifiormaly randomly
pub fn random_perm(n: u8, restrictions: &Vec<Vec<u8>>) -> Vec<u8> {
    let mut positions: Vec<usize> = vec![0; n as usize];
    let mut all: HashSet<u8> = HashSet::new();
    let mut r: Vec<HashSet<usize>> = vec![HashSet::new(); n as usize];
    let mut temp_r:Vec<HashSet<usize>> = vec![HashSet::new(); n as usize];
    let mut taken:HashSet<usize> = HashSet::new();

    for i in 0..n {
        all.insert(i);
    }
    for restriction in restrictions {
        for i in 0..n as usize {
            r[restriction[i] as usize].insert(i);
        }
    }

    let mut value: usize = 0;
    while value < (n as usize) {
        let available_choices: Vec<usize> = (0..n as usize).filter(|x| !taken.contains(x) && !r[value].contains(x) && !temp_r[value].contains(x)).collect();
        if available_choices.is_empty() {
            // Back track
            temp_r[value].clear();
            value -= 1;
            taken.remove(&(positions[value] as usize));
            temp_r[value].insert(positions[value] as usize);
        } else {
            let mut rng = thread_rng();
            let choice = *available_choices.choose(&mut rng).unwrap();
            positions[value] = choice;
            taken.insert(choice);
            value += 1 ;
        }
    }

    let mut perm: Vec<u8> = vec![0; n as usize];
    for i in 0..(n as usize) {
        perm[positions[i]] = i as u8;
    }
    return perm;
}

// #[derive(Clone, Debug)]
// pub struct PermIter {
//     pub n: u8,
//     pub positions: Vec<usize>,
//     pub restrictions: Vec<HashSet<usize>>,
//     pub done: bool,
// }
//
// impl PermIter {
//     pub fn new(n: u8, restrictions: &Vec<Vec<u8>>) -> PermIter {
//         let mut r: Vec<HashSet<usize>> = vec![HashSet::new(); n as usize];
//         for restriction in restrictions {
//             for i in 0..n as usize {
//                 r[restriction[i] as usize].insert(i);
//             }
//         }
//
//         let mut iter = PermIter{
//             n,
//             positions: vec![0; n as usize],
//             restrictions: r,
//             done: false
//         };
//         iter.find_first();
//
//         return iter;
//     }
//
//     fn find_first(&mut self) {
//         let n = self.n as usize;
//
//         let mut taken:HashSet<usize> = HashSet::new();
//         let mut depth = 0;
//         let mut current = 0;
//
//         while depth < n {
//             if current < n && !taken.contains(&current) && !self.restrictions[depth].contains(&current){
//                 self.positions[depth] = current;
//                 taken.insert(current);
//                 depth += 1;
//                 current = 0;
//             } else if current == n {
//                 if depth == 0 {
//                     self.done = true;
//                     return;
//                 }
//                 depth -= 1;
//                 current = self.positions[depth];
//                 taken.remove(&current);
//                 current = current + 1;
//             } else {
//                 current += 1;
//             }
//         }
//     }
//
//     fn increment(&mut self) {
//         let n = self.n as usize;
//
//         let mut taken:HashSet<usize> = HashSet::new();
//         let mut depth = n - 1;
//         let mut current = self.positions[depth] + 1;
//
//         for i in 0..depth {
//             taken.insert(self.positions[i]);
//         }
//
//         while depth < n {
//             if current < n && !taken.contains(&current) && !self.restrictions[depth].contains(&current){
//                 self.positions[depth] = current;
//                 taken.insert(current);
//                 depth += 1;
//                 current = 0;
//             } else if current == n {
//                 if depth == 0 {
//                     self.done = true;
//                     return;
//                 }
//                 depth -= 1;
//                 current = self.positions[depth];
//                 taken.remove(&current);
//                 current = current + 1;
//             } else {
//                 current += 1;
//             }
//         }
//     }
//
//     fn build_perm(&self) -> Vec<u8> {
//         let mut perm: Vec<u8> = vec![0; self.n as usize];
//         for i in 0..self.n {
//             perm[self.positions[i as usize]] = i;
//         }
//
//         return perm;
//     }
// }
//
// impl Iterator for PermIter {
//     type Item = Vec<u8>;
//
//     fn next(&mut self) -> Option<Self::Item> {
//         if self.done {
//             return None;
//         }
//         let result = self.build_perm();
//         self.increment();
//         return Some(result);
//     }
// }

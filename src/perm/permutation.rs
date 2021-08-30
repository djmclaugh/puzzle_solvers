use std::collections::HashSet;

#[derive(Clone, Debug)]
pub struct PermIter {
    pub n: u8,
    pub positions: Vec<usize>,
    pub restrictions: Vec<HashSet<usize>>,
    pub done: bool,
}

impl PermIter {
    pub fn new(n: u8, restrictions: &Vec<Vec<u8>>) -> PermIter {
        let mut r: Vec<HashSet<usize>> = vec![HashSet::new(); n as usize];
        for restriction in restrictions {
            for i in 0..n as usize {
                r[restriction[i] as usize].insert(i);
            }
        }

        let mut iter = PermIter{
            n,
            positions: vec![0; n as usize],
            restrictions: r,
            done: false
        };
        iter.find_first();

        return iter;
    }

    fn find_first(&mut self) {
        let n = self.n as usize;

        let mut taken:HashSet<usize> = HashSet::new();
        let mut depth = 0;
        let mut current = 0;

        while depth < n {
            if current < n && !taken.contains(&current) && !self.restrictions[depth].contains(&current){
                self.positions[depth] = current;
                taken.insert(current);
                depth += 1;
                current = 0;
            } else if current == n {
                if depth == 0 {
                    self.done = true;
                    return;
                }
                depth -= 1;
                current = self.positions[depth];
                taken.remove(&current);
                current = current + 1;
            } else {
                current += 1;
            }
        }
    }

    fn increment(&mut self) {
        let n = self.n as usize;

        let mut taken:HashSet<usize> = HashSet::new();
        let mut depth = n - 1;
        let mut current = self.positions[depth] + 1;

        for i in 0..depth {
            taken.insert(self.positions[i]);
        }

        while depth < n {
            if current < n && !taken.contains(&current) && !self.restrictions[depth].contains(&current){
                self.positions[depth] = current;
                taken.insert(current);
                depth += 1;
                current = 0;
            } else if current == n {
                if depth == 0 {
                    self.done = true;
                    return;
                }
                depth -= 1;
                current = self.positions[depth];
                taken.remove(&current);
                current = current + 1;
            } else {
                current += 1;
            }
        }
    }

    fn build_perm(&self) -> Vec<u8> {
        let mut perm: Vec<u8> = vec![0; self.n as usize];
        for i in 0..self.n {
            perm[self.positions[i as usize]] = i;
        }

        return perm;
    }
}

impl Iterator for PermIter {
    type Item = Vec<u8>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }
        let result = self.build_perm();
        self.increment();
        return Some(result);
    }
}

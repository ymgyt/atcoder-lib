pub struct UnionFind {
    parent: Vec<Option<usize>>,
    size: Vec<usize>,
}

#[derive(PartialEq, Eq, Debug)]
pub enum UnionResult {
    Unified,
    AlreadyUnified,
}

impl UnionFind {
    /// Create new `UnionFind` of `n` disjoint sets.
    pub fn new(n: usize) -> Self {
        Self {
            parent: vec![None; n],
            size: vec![1; n],
        }
    }

    pub fn union(&mut self, x: usize, y: usize) -> UnionResult {
        if x == y {
            return UnionResult::AlreadyUnified;
        }

        let (large, small) = {
            let (x, y) = (self.root_with_optimize(x), self.root_with_optimize(y));
            if x == y {
                return UnionResult::AlreadyUnified;
            }

            if self.size(x) >= self.size(y) {
                (x, y)
            } else {
                (y, x)
            }
        };

        self.parent[small] = Some(large);
        self.size[large] += self.size[small];
        UnionResult::Unified
    }

    /// Returns `true` if the given elements belong to the same set
    /// and returns `false` otherwise.
    pub fn equiv(&self, x: usize, y: usize) -> bool {
        self.root(x) == self.root(y)
    }

    /// Return the representative for `x`.
    pub fn root(&self, x: usize) -> usize {
        let mut curr = x;
        loop {
            match self.parent[curr] {
                Some(parent) => {
                    curr = parent;
                }
                None => return curr,
            }
        }
    }

    fn root_with_optimize(&mut self, x: usize) -> usize {
        let mut curr = x;
        loop {
            match self.parent[curr] {
                Some(parent) => {
                    self.parent[x] = Some(parent);
                    curr = parent;
                }
                None => return curr,
            }
        }
    }

    pub fn size(&self, x: usize) -> usize {
        self.size[x]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let mut uf = UnionFind::new(3);

        assert_eq!(uf.root(0), 0);
        assert_eq!(uf.root(1), 1);
        assert_eq!(uf.root(2), 2);

        assert!(!uf.equiv(0, 1));
        assert!(!uf.equiv(1, 2));
        assert!(!uf.equiv(2, 0));

        assert_eq!(uf.size(0), 1);
        assert_eq!(uf.size(1), 1);
        assert_eq!(uf.size(2), 1);

        assert_eq!(uf.union(0, 1), UnionResult::Unified);
        assert!(uf.equiv(0, 1));
        assert_eq!(uf.size(0), 2);
    }
}

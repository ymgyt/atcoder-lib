pub struct UnionFind {
    parent: Vec<usize>,
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
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }

    pub fn union(&mut self, x: usize, y: usize) -> UnionResult {
        if x == y {
            return UnionResult::AlreadyUnified;
        }

        let (large, small) = {
            let (x, y) = (self.root(x), self.root(y));
            if x == y {
                return UnionResult::AlreadyUnified;
            }

            if self.size(x) >= self.size(y) {
                (x, y)
            } else {
                (y, x)
            }
        };

        self.parent[small] = large;
        self.size[large] += self.size[small];
        UnionResult::Unified
    }

    /// Returns `true` if the given elements belong to the same set
    /// and returns `false` otherwise.
    pub fn equiv(&mut self, x: usize, y: usize) -> bool {
        self.root(x) == self.root(y)
    }

    fn root(&mut self, x: usize) -> usize {
        let mut curr = x;
        loop {
            let parent = self.parent[curr];
            if curr == parent {
                self.parent[x] = curr;
                return curr;
            }
            curr = parent;
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

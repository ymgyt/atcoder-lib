use std::ops::Index;

const INF: i64 = i64::MAX;

/// Directed asyclic graph
pub struct Dag {
    edges: Vec<Vec<Edge>>,
}

#[derive(Clone, Copy, Debug)]
struct Edge {
    from: usize,
    to: usize,
    cost: i64,
}

impl Dag {
    pub fn new(size: usize) -> Self {
        Self {
            edges: vec![vec![]; size],
        }
    }

    pub fn add_edge(&mut self, from: usize, to: usize, cost: i64) {
        let edge = Edge { from, to, cost };
        self.edges[from].push(edge);
    }

    pub fn remove_edge(&mut self, from: usize, to: usize) {
        let adj = &mut self.edges[from];
        let Some(pos) = adj.iter().position(|edge| edge.to == to) else { return };
        adj.swap_remove(pos);
    }

    fn size(&self) -> usize {
        self.edges.len()
    }

    fn edges(&self) -> impl Iterator<Item = Edge> + '_ {
        self.edges.iter().flatten().cloned()
    }
}

pub struct ShortestPaths(Vec<Vec<i64>>);

impl Index<usize> for ShortestPaths {
    type Output = [i64];

    fn index(&self, index: usize) -> &Self::Output {
        self.0[index].as_slice()
    }
}

impl From<Vec<Vec<i64>>> for ShortestPaths {
    fn from(v: Vec<Vec<i64>>) -> Self {
        ShortestPaths(v)
    }
}

impl Dag {
    pub fn floyd_warshall(&self) -> ShortestPaths {
        let n = self.size();
        let mut dp = vec![vec![INF; n]; n];

        // init self edge to zero
        for (i, adj) in dp.iter_mut().enumerate() {
            adj[i] = 0
        }

        // write eges
        self.edges().for_each(|e| {
            dp[e.from][e.to] = e.cost;
        });

        let chmin = |a: &mut i64, b: i64| {
            if b < *a {
                *a = b
            }
        };

        for k in 0..n {
            for i in 0..n {
                for j in 0..n {
                    let cost = dp[i][k].saturating_add(dp[k][j]);
                    chmin(&mut dp[i][j], cost);
                }
            }
        }

        dp.into()
    }
}

impl Dag {
    //
    pub fn djkstra(&self, from: usize, to: usize) -> Option<i64> {
        //  https://doc.rust-lang.org/std/collections/binary_heap/index.html#examples
        use std::{cmp::Ordering, collections::BinaryHeap};

        #[derive(Copy, Clone, Eq, PartialEq)]
        struct State {
            position: usize,
            cost: i64,
        }

        // BinaryHeap::pop require item to impl Ord
        impl Ord for State {
            fn cmp(&self, other: &Self) -> Ordering {
                // I'm not sure why we should compare position
                other
                    .cost
                    .cmp(&self.cost)
                    .then_with(|| self.position.cmp(&other.position))
            }
        }

        impl PartialOrd for State {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.cmp(other))
            }
        }

        let mut dist = (0..self.size()).map(|_| INF).collect::<Vec<_>>();
        dist[from] = 0;

        let mut queue = BinaryHeap::new();
        queue.push(State {
            position: from,
            cost: 0,
        });

        while let Some(State { position, cost }) = queue.pop() {
            if position == to {
                return Some(cost);
            }

            if cost > dist[position] {
                continue;
            }

            for edge in &self.edges[position] {
                let next = State {
                    position: edge.to,
                    cost: edge.cost + cost,
                };
                if next.cost < dist[next.position] {
                    queue.push(next);
                    dist[next.position] = next.cost;
                }
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn floyd_warshall() {
        let mut g = Dag::new(4);

        g.add_edge(0, 1, 5);
        g.add_edge(0, 2, 2);
        g.add_edge(1, 3, 4);
        g.add_edge(2, 1, 2);
        g.add_edge(2, 3, 3);

        let sp = g.floyd_warshall();

        assert_eq!(sp[0][3], 5);
        assert_eq!(sp[0][1], 4);
    }

    #[test]
    fn djkstra() {
        // stole from std
        //
        //                  7
        //          +-----------------+
        //          |                 |
        //          v   1        2    |  2
        //          0 -----> 1 -----> 3 ---> 4
        //          |        ^        ^      ^
        //          |        | 1      |      |
        //          |        |        | 3    | 1
        //          +------> 2 -------+      |
        //           10      |               |
        //                   +---------------+

        let mut g = Dag::new(5);

        g.add_edge(0, 1, 1);
        g.add_edge(0, 2, 10);
        g.add_edge(1, 3, 2);
        g.add_edge(2, 1, 1);
        g.add_edge(2, 3, 3);
        g.add_edge(2, 4, 1);
        g.add_edge(3, 0, 7);
        g.add_edge(3, 4, 2);

        assert_eq!(g.djkstra(0, 4), Some(5));
        assert_eq!(g.djkstra(0, 3), Some(3));
    }
}

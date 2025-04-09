use std::collections::BinaryHeap;
use ordered_float::NotNan;
use std::cmp::Reverse;
type DistWrapper = NotNan<f64>;

#[derive(Clone)]
struct Edge {
    to: usize,
    weight: f64,
}

pub struct Graph {
    edges: Vec<Vec<Edge>>,
    distances: Vec<f64>,
    changed: Vec<usize>,
    visited: Vec<bool>,
}

impl Graph {
    pub fn new(size: usize) -> Self {
        Graph {
            edges: vec![vec![]; size],
            distances: vec![f64::INFINITY; size],
            changed: vec![],
            visited: vec![false; size],
        }
    }

    pub fn add_edge(&mut self, from: usize, to: usize, weight: f64) {
        self.edges[from].push(Edge { to, weight });
    }

    pub fn dijkstra(&mut self, start: usize, end: usize) -> f64 {
        while let Some(v) = self.changed.pop() {
            self.distances[v] = f64::INFINITY;
            self.visited[v] = false;
        }

        self.distances[start] = 0.0;

        let mut heap = BinaryHeap::new();
        heap.push((Reverse(DistWrapper::default()), start));
        self.visited[start] = true;
        self.changed.push(start);

        while let Some((dist_wrapper, u)) = heap.pop() {
            let dist = dist_wrapper.0.into();
            if u == end {
                return dist;
            }

            if dist > self.distances[u] {
                continue;
            }
            self.changed.push(u);

            for &Edge {to: v, weight} in &self.edges[u] {
                let next_dist = dist + weight;
                if next_dist < self.distances[v] {
                    self.distances[v] = next_dist;
                    heap.push((Reverse(DistWrapper::new(next_dist).unwrap()), v));
                    if !self.visited[v] {
                        self.visited[v] = true;
                        self.changed.push(v);
                    }
                }
            }
        }
        f64::MAX
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dijkstra() {
        let mut g = Graph::new(6);
            g.add_edge(0, 1, 7.0);
            g.add_edge(1, 2, 10.0);
            g.add_edge(0, 2, 9.0);
            g.add_edge(0, 5, 14.0);
            g.add_edge(1, 3, 15.0);
            g.add_edge(2, 5, 2.0);
            g.add_edge(2, 3, 11.0);
            g.add_edge(4, 5, 9.0);
            g.add_edge(3, 4, 6.0);
            g.add_edge(2, 2, 1.0);
            assert_eq!(20.0, g.dijkstra(0, 3));
            assert_eq!(26.0, g.dijkstra(0, 4));
    }
}
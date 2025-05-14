use std::collections::BinaryHeap;
use ordered_float::NotNan;
use std::cmp::Reverse;

// use crate::dary_heap::DaryHeap;

type DistWrapper = NotNan<f64>;

#[derive(Clone)]
struct Edge {
    to: usize,
    weight: f64,
}

pub struct Graph<const CALCPATH: bool, const HEURISTIC: bool, const EARLYSTOP: bool> {
    edges: Vec<Vec<Edge>>,
    coord: Vec<(i32, i32)>,
    distances: Vec<f64>,
    changed: Vec<usize>,
    visited: Vec<bool>,
    calced: Vec<bool>,
    heap: BinaryHeap<(Reverse<DistWrapper>, usize)>,
    // heap: DaryHeap<(Reverse<DistWrapper>, usize), 3>,
    path: Vec<usize>,
    num_edges: usize,
}

impl<const CALCPATH: bool, const HEURISTIC: bool, const EARLYSTOP: bool> Graph<CALCPATH, HEURISTIC, EARLYSTOP> {
    pub fn new(size: usize, coord: &[(i32, i32)]) -> Self {
        println!("HEURISTIC: {}, EARLYSTOP: {}, CALCPATH: {}", HEURISTIC, EARLYSTOP, CALCPATH);
        Graph {
            edges: vec![vec![]; size],
            coord: coord.to_vec(),
            distances: vec![f64::INFINITY; size],
            changed: Vec::with_capacity(size),
            visited: vec![false; size],
            calced: vec![false; size],
            heap: BinaryHeap::with_capacity(2 * size),
            // heap: DaryHeap::with_capacity(2 * size),
            path: vec![0; size],
            num_edges: 0,
        }
    }

    pub fn add_edge(&mut self, from: usize, to: usize) {
        self.num_edges += 1;
        self.edges[from].push(
            Edge {
                to,
                weight: Self::euclidean_dist(self.coord[from], self.coord[to])
            }
        );
    }

    pub fn dijkstra(&mut self, start: usize, end: usize) -> f64 {

        if EARLYSTOP {
            while let Some(v) = self.changed.pop() {
                self.distances[v] = f64::INFINITY;
                self.visited[v] = false;
                self.calced[v] = false;
            }
        } else {
            self.distances.fill(f64::INFINITY);
            self.calced.fill(false);
        }

        if CALCPATH {
            self.path[start] = start;
            self.path[end] = end;
        }


        if HEURISTIC {
            self.distances[start] = Self::euclidean_dist(self.coord[start], self.coord[end]);
        } else {
            self.distances[start] = 0.0;
        }

        if EARLYSTOP {
            self.visited[start] = true;
            self.changed.push(start);
        }

        let heap = &mut self.heap;
        heap.clear();
        heap.push((Reverse(DistWrapper::new(self.distances[start]).unwrap()), start));

        while let Some((dist_wrapper, u)) = heap.pop() {
            let dist: f64 = dist_wrapper.0.into();
            
            if self.calced[u] {
                continue;
            }
            self.calced[u] = true;

            if EARLYSTOP {
                if u == end {
                    return dist;
                }
            }

            for &Edge {to: v, weight} in &self.edges[u] {
                if self.calced[v] {
                    continue;
                }

                let next_dist;

                if HEURISTIC {
                    next_dist = dist + weight + Self::euclidean_dist(self.coord[v], self.coord[end]) - Self::euclidean_dist(self.coord[u], self.coord[end]);
                } else {
                    next_dist = dist + weight;
                }

                if next_dist < self.distances[v] {
                    self.distances[v] = next_dist;
                    heap.push((Reverse(DistWrapper::new(next_dist).unwrap()), v));

                    if CALCPATH {
                        self.path[v] = u;
                    }

                    if EARLYSTOP {
                        if !self.visited[v] {
                            self.visited[v] = true;
                            self.changed.push(v);
                        }
                    }
                }
            }
        }
        self.distances[end]
    }

    #[inline]
    pub fn euclidean_dist(u: (i32, i32), v: (i32, i32)) -> f64 {
        (((u.0 - v.0).pow(2) + (u.1 - v.1).pow(2)) as f64).sqrt()
    }

    #[inline]
    pub fn chebyshev_dist(u: (i32, i32), v: (i32, i32)) -> f64 {
        (u.0 - v.0).abs().max((u.1 - v.1).abs()) as f64
    }

    pub fn get_path(&self, start: usize, end: usize) -> Vec<usize> {
        let mut path = Vec::new();
        let mut current = end;
        while current != self.path[current] {
            path.push(current);
            current = self.path[current];   
        }
        path.push(start);
        path.reverse();
        path
    }
    
}
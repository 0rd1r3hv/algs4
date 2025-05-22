use crate::dary_heap::DaryHeap;
use ordered_float::NotNan;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
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
    // heap: BinaryHeap<(Reverse<DistWrapper>, usize)>,
    heap: DaryHeap<(Reverse<DistWrapper>, usize), 2>,
    path: Vec<usize>,
    num_edges: usize,
}

impl<const CALCPATH: bool, const HEURISTIC: bool, const EARLYSTOP: bool>
    Graph<CALCPATH, HEURISTIC, EARLYSTOP>
{
    #[inline]
    pub fn new(size: usize, coord: &[(i32, i32)]) -> Self {
        println!(
            "HEURISTIC: {}, EARLYSTOP: {}, CALCPATH: {}",
            HEURISTIC, EARLYSTOP, CALCPATH
        );
        Graph {
            edges: vec![vec![]; size],
            coord: coord.to_vec(),
            distances: vec![f64::INFINITY; size],
            changed: Vec::with_capacity(size),
            visited: vec![false; size],
            calced: vec![false; size],
            // heap: BinaryHeap::with_capacity(2 * size),
            heap: DaryHeap::with_capacity(2 * size),
            path: vec![0; size],
            num_edges: 0,
        }
    }

    #[inline]
    pub fn add_edge(&mut self, from: usize, to: usize) {
        self.num_edges += 1;
        self.edges[from].push(Edge {
            to,
            weight: Self::euclidean_dist(&self.coord[from], &self.coord[to]),
        });
    }

    #[inline]
    pub fn dijkstra(&mut self, start: usize, end: usize) -> f64 {
        if EARLYSTOP {
            while let Some(v) = self.changed.pop() {
                *self.get_dist_mut(v) = f64::INFINITY;
                *self.get_visited_mut(v) = false;
                *self.get_calced_mut(v) = false;
            }
        } else {
            self.distances.fill(f64::INFINITY);
            self.calced.fill(false);
        }

        if CALCPATH {
            *self.get_path_mut(start) = start;
            *self.get_path_mut(end) = end;
        }

        if HEURISTIC {
            *self.get_dist_mut(start) =
                Self::euclidean_dist(self.get_coord(start), self.get_coord(end));
        } else {
            *self.get_dist_mut(start) = 0.0;
        }

        if EARLYSTOP {
            *self.get_visited_mut(start) = true;
            self.changed.push(start);
        }

        self.heap.clear();
        self.heap.push((
            Reverse(DistWrapper::new(*self.get_dist(start)).unwrap()),
            start,
        ));

        while let Some((dist_wrapper, u)) = self.heap.pop() {
            let dist: f64 = dist_wrapper.0.into();

            if *self.get_calced(u) {
                continue;
            }
            *self.get_calced_mut(u) = true;

            if EARLYSTOP {
                if u == end {
                    return dist;
                };
            }

            for &Edge { to: v, weight } in unsafe { self.edges.get_unchecked(u) } {
                if *self.get_calced(v) {
                    continue;
                }

                let next_dist;

                if HEURISTIC {
                    next_dist = dist
                        + weight
                        + Self::euclidean_dist(self.get_coord(v), self.get_coord(end))
                        - Self::euclidean_dist(self.get_coord(u), self.get_coord(end));
                } else {
                    next_dist = dist + weight;
                }

                if next_dist < *self.get_dist(v) {
                    unsafe {
                        *self.distances.get_unchecked_mut(v) = next_dist;
                    }
                    self.heap
                        .push((Reverse(DistWrapper::new(next_dist).unwrap()), v));

                    if CALCPATH {
                        unsafe {
                            *self.path.get_unchecked_mut(v) = u;
                        }
                    }

                    if EARLYSTOP {
                        if !*self.get_visited(v) {
                            unsafe {
                                *self.visited.get_unchecked_mut(v) = true;
                            }
                            self.changed.push(v);
                        }
                    }
                }
            }
        }
        *self.get_dist(end)
    }

    #[inline]
    pub fn euclidean_dist(u: &(i32, i32), v: &(i32, i32)) -> f64 {
        (((u.0 - v.0).pow(2) + (u.1 - v.1).pow(2)) as f64).sqrt()
    }

    #[inline]
    pub fn chebyshev_dist(u: &(i32, i32), v: &(i32, i32)) -> f64 {
        (u.0 - v.0).abs().max((u.1 - v.1).abs()) as f64
    }

    #[inline]
    pub fn get_path(&self, start: usize, end: usize) -> (usize, impl Iterator<Item = usize>) {
        let mut path = Vec::new();
        let mut current = end;
        while current != self.path[current] {
            path.push(current);
            current = self.path[current];
        }
        path.push(start);
        (path.len(), path.into_iter().rev())
    }

    #[inline]
    fn get_dist(&self, u: usize) -> &f64 {
        unsafe { self.distances.get_unchecked(u) }
    }

    #[inline]
    fn get_dist_mut(&mut self, u: usize) -> &mut f64 {
        unsafe { self.distances.get_unchecked_mut(u) }
    }

    #[inline]
    fn get_visited(&self, u: usize) -> &bool {
        unsafe { self.visited.get_unchecked(u) }
    }

    #[inline]
    fn get_visited_mut(&mut self, u: usize) -> &mut bool {
        unsafe { self.visited.get_unchecked_mut(u) }
    }

    #[inline]
    fn get_calced(&self, u: usize) -> &bool {
        unsafe { self.calced.get_unchecked(u) }
    }

    #[inline]
    fn get_calced_mut(&mut self, u: usize) -> &mut bool {
        unsafe { self.calced.get_unchecked_mut(u) }
    }

    #[inline]
    fn get_coord(&self, u: usize) -> &(i32, i32) {
        unsafe { self.coord.get_unchecked(u) }
    }

    #[inline]
    fn get_coord_mut(&mut self, u: usize) -> &mut (i32, i32) {
        unsafe { self.coord.get_unchecked_mut(u) }
    }

    #[inline]
    fn get_path_mut(&mut self, u: usize) -> &mut usize {
        unsafe { self.path.get_unchecked_mut(u) }
    }

    #[inline]
    fn get_edges(&self, u: usize) -> &Vec<Edge> {
        unsafe { self.edges.get_unchecked(u) }
    }

    #[inline]
    fn get_edge(&self, u: usize, v: usize) -> &Edge {
        unsafe { self.edges.get_unchecked(u).get_unchecked(v) }
    }
}

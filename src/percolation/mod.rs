use crate::union_find::UnionFind;
use std::{
    fmt::Display,
    marker::PhantomData,
    time::{Duration, Instant},
};

struct Percolation<U: UnionFind> {
    grid: Vec<Vec<bool>>,
    size: usize,
    top: usize,
    bottom: usize,
    uf: U,
}

pub struct PercolationStats<U: UnionFind> {
    size: usize,
    mean: f64,
    stddev: f64,
    confidence_low: f64,
    confidence_high: f64,
    trials: usize,
    time: Duration,
    _phantom: PhantomData<U>,
}

impl<U: UnionFind> Percolation<U> {
    pub fn new(size: usize) -> Self {
        let grid = vec![vec![false; size]; size];
        let top = size * size;
        let bottom = top + 1;
        let uf = U::new(bottom + 1);
        Self {
            grid,
            size,
            top,
            bottom,
            uf,
        }
    }

    pub fn is_open(&self, row: usize, col: usize) -> bool {
        self.grid[row][col]
    }

    pub fn percolates(&mut self) -> bool {
        self.uf.connected(self.top, self.bottom)
    }

    pub fn index(&self, row: usize, col: usize) -> usize {
        row * self.size + col
    }

    pub fn open(&mut self, row: usize, col: usize) {
        self.grid[row][col] = true;
        let idx = self.index(row, col);

        if row == 0 {
            self.uf.union(idx, self.top);
        }
        if row == self.size - 1 {
            self.uf.union(idx, self.bottom);
        }
        if row != 0 && self.is_open(row - 1, col) {
            self.uf.union(idx, idx - self.size);
        }
        if row != self.size - 1 && self.is_open(row + 1, col) {
            self.uf.union(idx, idx + self.size);
        }
        if col != 0 && self.is_open(row, col - 1) {
            self.uf.union(idx, idx - 1);
        }
        if col != self.size - 1 && self.is_open(row, col + 1) {
            self.uf.union(idx, idx + 1);
        }
    }
}

impl<U: UnionFind> PercolationStats<U> {
    pub fn new(size: usize, trials: usize) -> Self {
        use rand::Rng;
        let mut results = vec![0.0; trials];
        let sq_size = size * size;
        let mut rng = rand::rng();

        let start_time = Instant::now();
        for result in &mut results {
            let mut percolation = Percolation::<U>::new(size);
            while !percolation.percolates() {
                let mut row = rng.random_range(0..size);
                let mut col = rng.random_range(0..size);
                while percolation.is_open(row, col) {
                    row = rng.random_range(0..size);
                    col = rng.random_range(0..size);
                }
                percolation.open(row, col);
                *result += 1.0;
            }
            *result /= sq_size as f64;
        }
        let end_time = Instant::now();

        let time = end_time.duration_since(start_time);
        let mean = results.iter().sum::<f64>() / trials as f64;
        let stddev = results.iter().map(|&x| (x - mean).powi(2)).sum::<f64>() / (trials - 1) as f64;
        let confidence_precomputed = 1.96 * stddev / (trials as f64).sqrt();
        let confidence_low = mean - confidence_precomputed;
        let confidence_high = mean + confidence_precomputed;
        Self {
            size,
            mean,
            stddev,
            confidence_low,
            confidence_high,
            time,
            trials,
            _phantom: PhantomData,
        }
    }
}

impl<U: UnionFind> Display for PercolationStats<U> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "n                       = {}", self.size)?;
        writeln!(f, "trials                  = {}", self.trials)?;
        writeln!(f, "time taken              = {}", self.time.as_secs_f64())?;
        writeln!(f, "mean                    = {}", self.mean)?;
        writeln!(f, "stddev                  = {}", self.stddev)?;
        writeln!(
            f,
            "95% confidence interval = [{}, {}]",
            self.confidence_low, self.confidence_high
        )?;
        Ok(())
    }
}

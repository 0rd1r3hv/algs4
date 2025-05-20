use crate::union_find::UnionFind;
use bitvec::{
    prelude::*,
    ptr::{Const, Mut},
};
use std::{
    fmt::Display,
    marker::PhantomData,
    time::{Duration, Instant},
};

struct Percolation<U: UnionFind> {
    grid: BitVec,
    size: usize,
    top: usize,
    bottom: usize,
    last_row: usize,
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
    #[inline]
    pub fn new(size: usize) -> Self {
        let top = size * size;
        let bottom = top + 1;
        let grid = bitvec![0; top];
        let uf = U::new(bottom + 1);
        Self {
            grid,
            size,
            top,
            bottom,
            last_row: size * (size - 1),
            uf,
        }
    }

    #[inline]
    fn get_grid(&self, pos: usize) -> BitRef<Const, usize> {
        unsafe { self.grid.get_unchecked(pos) }
    }

    #[inline]
    fn get_grid_mut(&mut self, pos: usize) -> BitRef<Mut, usize> {
        unsafe { self.grid.get_unchecked_mut(pos) }
    }

    #[inline]
    pub fn is_open(&self, pos: usize) -> bool {
        *self.get_grid(pos)
    }

    #[inline]
    pub fn percolates(&mut self) -> bool {
        self.uf.connected(self.top, self.bottom)
    }

    #[inline]
    pub fn open(&mut self, pos: usize) {
        *self.get_grid_mut(pos) = true;

        // Case size = 1 is not intended to be considered
        if pos < self.size {
            self.uf.union(pos, self.top);
            if self.is_open(pos + self.size) {
                self.uf.union(pos, pos + self.size);
            }
        } else {
            if self.is_open(pos - self.size) {
                self.uf.union(pos, pos - self.size);
            }
            if pos >= self.last_row {
                self.uf.union(pos, self.bottom);
            } else if self.is_open(pos + self.size) {
                self.uf.union(pos, pos + self.size);
            }
        }

        let col = pos % self.size;
        if col == 0 {
            if self.is_open(pos + 1) {
                self.uf.union(pos, pos + 1);
            }
        } else {
            if self.is_open(pos - 1) {
                self.uf.union(pos, pos - 1);
            }
            if col != self.size - 1 && self.is_open(pos + 1) {
                self.uf.union(pos, pos + 1);
            }
        }
    }

    #[inline]
    pub fn reset(&mut self) {
        self.grid.fill(false);
        self.uf.reset();
    }
}

impl<U: UnionFind> PercolationStats<U> {
    #[inline]
    pub fn new(size: usize, trials: usize) -> Self {
        use rand::Rng;
        let mut results = vec![0.0; trials];
        let mut rng = rand::rng();
        let mut percolation = Percolation::<U>::new(size);
        let sq_size = size * size;

        let start_time = Instant::now();
        results.iter_mut().for_each(|result| {
            while !percolation.percolates() {
                let mut pos = rng.random_range(0..sq_size);
                while percolation.is_open(pos) {
                    pos = rng.random_range(0..sq_size);
                }
                percolation.open(pos);
                *result += 1.0;
            }
            percolation.reset();
            *result /= sq_size as f64;
        });

        let time = start_time.elapsed();
        let mean = results.iter().sum::<f64>() / trials as f64;
        let stddev =
            (results.iter().map(|&x| (x - mean).powi(2)).sum::<f64>() / (trials - 1) as f64).sqrt();
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
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "Algorithm               = {}",
            std::any::type_name::<U>()
        )?;
        writeln!(f, "n                       = {}", self.size)?;
        writeln!(f, "trials                  = {}", self.trials)?;
        writeln!(f, "time taken              = {:?}", self.time)?;
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

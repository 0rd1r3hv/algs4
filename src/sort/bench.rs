use super::Sorter;
use crate::utils::Placeholder;
use crate::utils::{GLOBAL, measure};
use rand::{distr::StandardUniform, prelude::Distribution, random};
use std::mem;
use std::time::Instant;

pub fn generate_random_array<T>(size: usize) -> Vec<T>
where
    T: Copy + Ord,
    StandardUniform: Distribution<T>,
{
    (0..size).map(|_| random::<T>()).collect()
}

pub fn test_sort<T>(name: &str, sort_fn: Sorter<T>, array: &Vec<T>)
where
    T: Copy + Ord + std::fmt::Debug,
{
    let mut array = array.clone();
    let array_size = array.len();
    let array_size_bytes = mem::size_of::<T>() * array_size;

    let start = Instant::now();
    sort_fn(&mut array, |a, b| a.cmp(b));
    GLOBAL.reset();
    let (_, stack_usage) = measure(|| sort_fn(&mut array, |a, b| a.cmp(b)));
    let heap_usage = GLOBAL.get();
    let duration = start.elapsed();

    assert!(array.is_sorted(), "{} failed to sort correctly", name);
    println!(
        "{}: size = {} elements ({} bytes), time = {:.6} seconds",
        name,
        array_size,
        array_size_bytes,
        duration.as_secs_f64()
    );
    println!("Stack usage: {} bytes", stack_usage);
    println!("Heap usage: {} bytes", heap_usage);
    println!("Total usage: {} bytes", stack_usage + heap_usage);
}

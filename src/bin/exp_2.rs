use algs4::sort::*;
use algs4::utils::{Placeholder, Trallocator, measure};
use rand::{distr::StandardUniform, prelude::Distribution, random};
use std::alloc::System;
use std::mem;
use std::time::{Duration, Instant};

#[global_allocator]
// static GLOBAL: Trallocator<System> = Trallocator::new(System);
static GLOBAL: System = System;
const K: usize = 10;
const SIZES: [usize; 7] = [100, 200, 1000, 2000, 10000, 20000, 100000];

const TIMES: usize = 10;

fn generate_random_array<T>(size: usize) -> Vec<T>
where
    T: Copy + Ord,
    StandardUniform: Distribution<T>,
{
    (0..size).map(|_| random::<T>()).collect()
}

fn generate_sorted_array<T>(size: usize) -> Vec<T>
where
    T: Copy + Ord,
    StandardUniform: Distribution<T>,
{
    let mut array: Vec<T> = (0..size).map(|_| random::<T>()).collect();
    array.sort();
    array
}

fn generate_reverse_sorted_array<T>(size: usize) -> Vec<T>
where
    T: Copy + Ord,
    StandardUniform: Distribution<T>,
{
    let mut array: Vec<T> = (0..size).map(|_| random::<T>()).collect();
    array.sort();
    array.reverse();
    array
}

fn generate_nearly_sorted_array<T>(size: usize) -> Vec<T>
where
    T: Copy + Ord,
    StandardUniform: Distribution<T>,
{
    let mut array: Vec<T> = (0..size).map(|_| random::<T>()).collect();
    array.sort();
    let swaps = (mem::size_of::<usize>() * 8 - size.leading_zeros() as usize - 1).pow(2);
    for _ in 0..swaps {
        let i = random::<u64>() as usize % size;
        let j = random::<u64>() as usize % size;
        array.swap(i, j);
    }
    array
}

fn generate_array_of_equal_elements<T>(size: usize) -> Vec<T>
where
    T: Copy + Ord,
    StandardUniform: Distribution<T>,
{
    let elements: Vec<T> = (0..K).map(|_| random::<T>()).collect();
    (0..size)
        .map(|_| elements[random::<u64>() as usize % K])
        .collect()
}

fn test_sort<T>()
where
    T: Copy + Ord + std::fmt::Debug,
    StandardUniform: Distribution<T>,
{
    let generators: Vec<(fn(usize) -> Vec<T>, &str)> = vec![
        (generate_random_array, "Random"),
        (generate_sorted_array, "Sorted"),
        (generate_reverse_sorted_array, "Reverse sorted"),
        (generate_nearly_sorted_array, "Nearly sorted"),
        (generate_array_of_equal_elements, "Equal elements"),
    ];
    let sorters: Vec<(Sorter<T>, &str)> = vec![
        (insertion_sort, "Insertion Sort"),
        (top_down_merge_sort, "Top-down Mergesort"),
        (bottom_up_merge_sort, "Bottom-up Mergesort"),
        (quick_sort, "Random Quicksort"),
        (quick_sort_3way, "Quicksort with Dijkstra 3-way Partition)"),
    ];
    let mut durations: Vec<Vec<Vec<Vec<Duration>>>> =
        vec![
            vec![vec![vec![Duration::from_millis(0); TIMES]; sorters.len()]; SIZES.len()];
            generators.len()
        ];
    let mut memory_usage: Vec<Vec<Vec<Vec<usize>>>> =
        vec![vec![vec![vec![0; TIMES]; sorters.len()]; SIZES.len()]; generators.len()];
    for (i, (generator, name)) in generators.iter().enumerate() {
        for (j, &size) in SIZES.iter().enumerate() {
            for k in 0..TIMES {
                let mut array = generator(size);
                for (l, (sorter, _)) in sorters.iter().enumerate() {
                    let mut array_cloned = array.clone();
                    if BENCH_MEMORY {
                        GLOBAL.reset();
                        let (_, stack_usage) = measure(|| sorter(&mut array_cloned, T::cmp));
                        let heap_usage = GLOBAL.get();
                        memory_usage[i][j][l][k] = stack_usage + heap_usage;
                    } else {
                        let start = Instant::now();
                        sorter(&mut array_cloned, T::cmp);
                        let duration = start.elapsed();
                        durations[i][j][l][k] = duration;
                    }
                    array.sort();
                    assert!(array == array_cloned, "{} failed to sort correctly", name);
                }
            }
        }
    }
    println!("Array type: {}", std::any::type_name::<T>());
    for (i, (_, name)) in generators.iter().enumerate() {
        println!("Generated data type: {}", name);
        for (j, &size) in SIZES.iter().enumerate() {
            println!();
            println!("Array size: {}", size);
            for (k, (_, sorter_name)) in sorters.iter().enumerate() {
                println!("Sorter: {}", sorter_name);
                if BENCH_MEMORY {
                    for memory in memory_usage[i][j][k].iter() {
                        print!("{} bytes ", memory);
                    }
                    println!(
                        "Average memory usage: {} bytes",
                        memory_usage[i][j][k].iter().sum::<usize>() / TIMES
                    );
                } else {
                    for duration in durations[i][j][k].iter() {
                        print!("{:?} ", duration);
                    }
                    println!(
                        "Average duration: {:?} ",
                        durations[i][j][k].iter().sum::<Duration>() / TIMES as u32
                    );
                }
                println!();
            }
        }
        println!("----------------------------------------");
    }
}

fn main() {
    test_sort::<i32>();
}

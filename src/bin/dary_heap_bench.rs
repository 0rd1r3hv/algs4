use algs4::dary_heap::DaryHeap;
use ordered_float::NotNan;
use rand::{distr::StandardUniform, prelude::Distribution, random};
use std::collections::BinaryHeap;
use std::time::Instant;
use clap::{Arg, Command};

fn generate_random_array<T>(size: usize) -> Vec<T>
where
    T: Copy,
    StandardUniform: Distribution<T>,
{
    (0..size).map(|_| random::<T>()).collect()
}

const D: usize = 2;
const TEST_POP: bool = true;
type TestType = f64;
type WrappedType = NotNan<TestType>;

fn main() {
    let matches = Command::new("dary_heap_bench")
        .arg(
            Arg::new("size")
                .short('s')
                .long("size")
                .value_parser(clap::value_parser!(usize))
                .default_value("5000")
        )
        .get_matches();

    let size = *matches.get_one::<usize>("size").unwrap();

    let mut dary_heap = DaryHeap::<WrappedType, D>::new();
    let mut binary_heap = BinaryHeap::<WrappedType>::new();
    let mut dary_heap_crate = dary_heap::DaryHeap::<WrappedType, D>::new();

    println!(
        "Size: {}, Type: {}",
        size,
        std::any::type_name::<TestType>()
    );

    // let arr = generate_random_array::<TestType>(size);
    // let start = Instant::now();
    // arr.iter().for_each(|&x| binary_heap.push(NotNan::new(x).unwrap()));
    // let duration = start.elapsed();
    // println!("std BinaryHeap push time: {:?}", duration);

    // let arr = generate_random_array::<TestType>(size);
    // let start = Instant::now();
    // arr.iter().for_each(|&x| dary_heap.push(NotNan::new(x).unwrap()));
    // let duration = start.elapsed();
    // println!("My {}-aryHeap push time: {:?}", D, duration);

    // let start = Instant::now();
    // (0..size).for_each(|_| {binary_heap.pop();});
    // let duration = start.elapsed();
    // println!("std BinaryHeap pop time: {:?}", duration);

    // let start = Instant::now();
    // (0..size).for_each(|_| {dary_heap.pop();});
    // let duration = start.elapsed();
    // println!("My {}-aryHeap pop time: {:?}", D, duration);
        
    let arr = generate_random_array::<TestType>(size);
    let start = Instant::now();
    arr.iter()
        .for_each(|&x| dary_heap.push(NotNan::new(x).unwrap()));
    let duration = start.elapsed();
    println!("My {}-aryHeap push time: {:?}", D, duration);

    let arr = generate_random_array::<TestType>(size);
    let start = Instant::now();
    arr.iter()
        .for_each(|&x| binary_heap.push(NotNan::new(x).unwrap()));
    let duration = start.elapsed();
    println!("std BinaryHeap push time: {:?}", duration);

    let arr = generate_random_array::<TestType>(size);
    let start = Instant::now();
    arr.iter()
        .for_each(|&x| dary_heap_crate.push(NotNan::new(x).unwrap()));
    let duration = start.elapsed();
    println!("Crate {}-aryHeap push time: {:?}", D, duration);


    if TEST_POP {
        let start = Instant::now();
        (0..size).for_each(|_| {
            dary_heap.pop();
        });
        let duration = start.elapsed();
        println!("My {}-aryHeap pop time: {:?}", D, duration);

        let start = Instant::now();
        (0..size).for_each(|_| {
            binary_heap.pop();
        });
        let duration = start.elapsed();
        println!("std BinaryHeap pop time: {:?}", duration);

        let start = Instant::now();
        (0..size).for_each(|_| {
            dary_heap_crate.pop();
        });
        let duration = start.elapsed();
        println!("Crate {}-aryHeap pop time: {:?}", D, duration);
    }
}

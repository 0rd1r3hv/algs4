use std::fs::File;
use std::io::{BufRead, BufReader};
use algs4::graph ::*;
use std::time::{Duration, Instant};
use rand::Rng;

fn main() {
    let file = File::open("usa.txt").expect("Failed to open file");
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    let first_line = lines.next().expect("No first line").expect("Failed to read first line");
    let mut parts = first_line.split_whitespace();
    let n: usize = parts.next().expect("No n").parse().expect("Invalid n");
    let m: usize = parts.next().expect("No m").parse().expect("Invalid m");
    let mut coord = vec![(0, 0); n];

    for _ in 0..n {
        let line = lines.next().expect("Missing node line").expect("Failed to read node line");
        let mut parts = line.split_whitespace();
        let u: usize = parts.next().expect("No node id").parse().expect("Invalid node id");
        let x: i32 = parts.next().expect("No x coordinate").parse().expect("Invalid x coordinate");
        let y: i32 = parts.next().expect("No y coordinate").parse().expect("Invalid y coordinate");
        coord[u] = (x, y);
    }

    lines.next();

    let mut graph = Graph::new(n);

    for _ in 0..m {
        let line = lines.next().expect("Missing edge line").expect("Failed to read edge line");
        let mut parts = line.split_whitespace();
        let u: usize = parts.next().expect("No node id").parse().expect("Invalid node id");
        let v: usize = parts.next().expect("No node id").parse().expect("Invalid node id");
        let dist = (((coord[u].0 - coord[v].0).pow(2) + (coord[u].1 - coord[v].1).pow(2)) as f64).sqrt();
        graph.add_edge(u, v, dist);
        graph.add_edge(v, u, dist);
    }

    let mut rng = rand::rng();
    let mut total_time = Duration::new(0, 0);
    let test_times = 100;

    for _ in 0..test_times {
        let start = rng.random_range(0..n);
        let end = rng.random_range(0..n);
        
        let start_time = Instant::now();
        let result = graph.dijkstra(start, end);
        println!("Shortest path from {} to {}: {}", start, end, result);
        let elapsed = start_time.elapsed();
        
        total_time += elapsed;
    }

    let average_time = total_time / test_times as u32;
    println!("Average time for {} Dijkstra executions: {:?}", test_times, average_time);

}
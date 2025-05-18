use algs4::graph::*;
use clap::{Arg, Command};
use std::fs::File;
use std::io::Write;
use std::io::{BufRead, BufReader};
use std::time::{Duration, Instant};

const EARLYSTOP: bool = false;
const HEURISTIC: bool = true;
const CALCPATH: bool = true;

fn main() {
    let matches = Command::new("exp_3")
        .arg(
            Arg::new("input")
                .short('i')
                .long("input")
                .value_parser(clap::value_parser!(String))
                .default_value("usa.txt"),
        )
        .arg(
            Arg::new("tests")
                .short('t')
                .long("tests")
                .value_parser(clap::value_parser!(i32))
                .default_value("100"),
        )
        .get_matches();

    let input_file = matches.get_one::<String>("input").unwrap();
    let tests = matches.get_one::<i32>("tests").unwrap();

    // 读取图的基本信息
    let (n, mut graph) = {
        let file = File::open(input_file).expect("Failed to open file");
        let reader = BufReader::new(file);
        let mut lines = reader.lines();

        // 读取节点数和边数
        let first_line = lines
            .next()
            .expect("No first line")
            .expect("Failed to read first line");
        let mut parts = first_line.split_whitespace();
        let n: usize = parts.next().expect("No n").parse().expect("Invalid n");
        let m: usize = parts.next().expect("No m").parse().expect("Invalid m");

        // 读取节点坐标
        let mut coord = vec![(0, 0); n];
        for _ in 0..n {
            let line = lines
                .next()
                .expect("Missing node line")
                .expect("Failed to read node line");
            let mut parts = line.split_whitespace();
            let u: usize = parts
                .next()
                .expect("No node id")
                .parse()
                .expect("Invalid node id");
            let x: i32 = parts
                .next()
                .expect("No x coordinate")
                .parse()
                .expect("Invalid x coordinate");
            let y: i32 = parts
                .next()
                .expect("No y coordinate")
                .parse()
                .expect("Invalid y coordinate");
            coord[u] = (x, y);
        }

        lines.next(); // 跳过空行

        // 构建图
        let mut graph = Graph::<CALCPATH, HEURISTIC, EARLYSTOP>::new(n, &coord);
        for _ in 0..m {
            let line = lines
                .next()
                .expect("Missing edge line")
                .expect("Failed to read edge line");
            let mut parts = line.split_whitespace();
            let u: usize = parts
                .next()
                .expect("No node id")
                .parse()
                .expect("Invalid node id");
            let v: usize = parts
                .next()
                .expect("No node id")
                .parse()
                .expect("Invalid node id");
            graph.add_edge(u, v);
            graph.add_edge(v, u);
        }
        (n, graph)
    };

    // 测试文件列表
    let test_files = [
        "usa-1.txt",
        "usa-10.txt",
        "usa-100long.txt",
        "usa-100short.txt",
        "usa-1000long.txt",
        "usa-5000short.txt",
        "usa-50000short.txt",
    ];

    // 对每个测试文件进行测试
    for test_file in test_files {
        let file = File::open(test_file).expect("Failed to open test file");
        let reader = BufReader::new(file);
        let mut total_time = Duration::new(0, 0);
        let mut count = 0;
        let output_file = format!("output/{}", test_file);
        let mut file = File::create(output_file).expect("Failed to create output file");

        for line in reader.lines() {
            let line = line.expect("Failed to read line");
            let mut parts = line.split_whitespace();
            let start: usize = parts
                .next()
                .expect("No start node")
                .parse()
                .expect("Invalid start node");
            let end: usize = parts
                .next()
                .expect("No end node")
                .parse()
                .expect("Invalid end node");

            let start_time = Instant::now();
            let dist = graph.dijkstra(start, end);
            total_time += start_time.elapsed();
            if CALCPATH {
                let path = graph.get_path(start, end);
                for node in path {
                    file.write_all(format!("{} ", node).as_bytes())
                        .expect("Failed to write to file");
                }
                file.write_all(format!("{} ", dist).as_bytes())
                    .expect("Failed to write to file");
                file.write_all(b"\n").expect("Failed to write to file");
            }
            count += 1;
        }

        println!("File: {}", test_file);
        println!("Total time: {:?}", total_time);
        println!("Average time: {:?}", total_time / count);
        println!();
    }

    // 随机测试
    let mut rng = rand::rng();
    let mut total_time = Duration::new(0, 0);

    for _ in 0..*tests {
        let start = rand::Rng::random_range(&mut rng, 0..n);
        let end = rand::Rng::random_range(&mut rng, 0..n);

        let start_time = Instant::now();
        graph.dijkstra(start, end);
        total_time += start_time.elapsed();
    }

    println!("Random tests ({} pairs):", tests);
    println!("Total time: {:?}", total_time);
    println!("Average time: {:?}", total_time / *tests as u32);
}

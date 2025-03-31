use algs4::percolation::PercolationStats;
use algs4::union_find::{QuickUnion, WeightedQuickUnion, WeightedQuickUnionWithPathCompression};
use clap::{Arg, Command};
fn main() {
    let matches = Command::new("exp_1")
        .arg(
            Arg::new("n")
                .short('n')
                .long("n")
                .value_parser(clap::value_parser!(usize))
                .default_value("200"),
        )
        .arg(
            Arg::new("trials")
                .short('t')
                .long("trials")
                .value_parser(clap::value_parser!(usize))
                .default_value("100"),
        )
        .arg(
            Arg::new("algorithm")
                .short('a')
                .long("algorithm")
                .value_parser(clap::value_parser!(String))
                .default_value("WeightedQuickUnion"),
        )
        .get_matches();

    let n = *matches.get_one::<usize>("n").unwrap();
    let trials = *matches.get_one::<usize>("trials").unwrap();
    let algorithm = matches.get_one::<String>("algorithm").unwrap().as_str();
    if algorithm == "WeightedQuickUnion" {
        println!("{}", PercolationStats::<WeightedQuickUnion>::new(n, trials));
    } else if algorithm == "WeightedQuickUnionWithPathCompression" {
        println!(
            "{}",
            PercolationStats::<WeightedQuickUnionWithPathCompression>::new(n, trials)
        );
    } else if algorithm == "QuickUnion" {
        println!("{}", PercolationStats::<QuickUnion>::new(n, trials));
    }
}

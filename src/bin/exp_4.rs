use algs4::aho_corasick::*;
use std::io;

fn main() {
    let mut str_buf = String::new();
    io::stdin().read_line(&mut str_buf).unwrap();
    let num_patterns = str_buf.trim().parse::<usize>().unwrap();

    let mut aho_corasick = AhoCorasick::new();
    for _ in 0..num_patterns {
        str_buf.clear();
        io::stdin().read_line(&mut str_buf).unwrap();
        let pattern = str_buf.trim();
        aho_corasick.add_pattern(pattern);
    }

    aho_corasick.build();
    str_buf.clear();
    io::stdin().read_line(&mut str_buf).unwrap();
    let text = str_buf.trim();

    let counts = aho_corasick.get_pattern_counts(text);

    for count in counts {
        println!("{}", count);
    }

    // let matches = aho_corasick.find_matches(text);
    // for (start, end) in matches {
    //     println!("{} {}", start, end);
    // }
}
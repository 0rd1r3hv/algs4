use aho_corasick::automaton::OverlappingState;
// use algs4::aho_corasick;
// use algs4::aho_corasick_v1;
use aho_corasick::AhoCorasick;
use std::fs::{self, File};
use std::io::{self, BufRead, Write};
use std::path::Path;
use std::time::Instant;

fn main() -> io::Result<()> {
    // 获取所有词典文件
    let needle_dir = Path::new("ac_bench/needle");
    let haystack_dir = Path::new("ac_bench/haystack");

    for needle_entry in fs::read_dir(needle_dir)? {
        let needle_path = needle_entry?.path();
        let needle_name = needle_path.file_stem().unwrap().to_str().unwrap();

        // 读取词典文件
        let file = File::open(&needle_path)?;
        let patterns: Vec<String> = io::BufReader::new(file)
            .lines()
            .map(|l| l.unwrap())
            .collect();

        // 对每个文本文件进行测试
        for haystack_entry in fs::read_dir(haystack_dir)? {
            let haystack_path = haystack_entry?.path();
            let haystack_name = haystack_path.file_stem().unwrap().to_str().unwrap();
            
            // 读取文本文件
            let text = fs::read_to_string(&haystack_path)?;

            // 测试 v1 版本
            let start = Instant::now();
            let mut ac_v1 = algs4::aho_corasick_v1::AhoCorasick::new();
            for pattern in &patterns {
                ac_v1.add_pattern(pattern);
            }
            ac_v1.build();
            let counts_v1: Vec<usize> = ac_v1.get_pattern_counts(&text).collect();
            let duration_v1 = start.elapsed();

            // 测试 v2 版本
            let start = Instant::now();
            let mut ac_v2 = algs4::aho_corasick::AhoCorasick::new();
            for pattern in &patterns {
                ac_v2.add_pattern(pattern);
            }
            ac_v2.build();
            let counts_v2: Vec<usize> = ac_v2.get_pattern_counts(&text).collect();
            let duration_v2 = start.elapsed();

            let mut counts = vec![0; patterns.len()];
            let start = Instant::now();
            let ac = AhoCorasick::new(&patterns).unwrap();
            let mut state = OverlappingState::start();
            ac.find_overlapping(&text, &mut state);
            while let Some(mat) = state.get_match() {
                counts[mat.pattern().as_usize()] += 1;
                ac.find_overlapping(&text, &mut state);
            }
            let duration_v3 = start.elapsed();

            // 输出结果
            let output_path = format!("./output/{}_{}_v1.txt", needle_name, haystack_name);
            let mut output_file = File::create(output_path)?;
            writeln!(output_file, "Time: {:?}", duration_v1)?;
            for count in counts_v1 {
                writeln!(output_file, "{}", count)?;
            }

            let output_path = format!("./output/{}_{}_v2.txt", needle_name, haystack_name);
            let mut output_file = File::create(output_path)?;
            writeln!(output_file, "Time: {:?}", duration_v2)?;
            for count in counts_v2 {
                writeln!(output_file, "{}", count)?;
            }

            let output_path = format!("./output/{}_{}_crate.txt", needle_name, haystack_name);
            let mut output_file = File::create(output_path)?;
            writeln!(output_file, "Time: {:?}", duration_v3)?;
            for count in counts {
                writeln!(output_file, "{}", count)?;
            }
        }
    }

    Ok(())
}
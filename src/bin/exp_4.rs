use aho_corasick::AhoCorasick;
use aho_corasick::automaton::OverlappingState;
use algs4::aho_corasick_bitmap;
use algs4::aho_corasick_fixed_vector;
use algs4::aho_corasick_hashmap;
use algs4::utils::{Placeholder, Trallocator};
use std::alloc::System;
use std::fs::{self, File};
use std::io::{self, BufRead, Write};
use std::path::Path;
use std::time::Instant;

#[global_allocator]
static GLOBAL: Trallocator<System> = Trallocator::new(System);
// static GLOBAL: System = System;

fn main() -> io::Result<()> {
    // 获取所有词典文件
    let needle_dir = Path::new("ac_bench/needle");
    let haystack_dir = Path::new("ac_bench/haystack");
    let bench_path = Path::new("./output/exp_4/bench.txt");
    let mut bench_file = File::create(bench_path)?;

    for needle_entry in fs::read_dir(needle_dir)? {
        let needle_path = needle_entry?.path();
        let needle_name = needle_path.file_stem().unwrap().to_str().unwrap();

        // 读取词典文件
        let file = File::open(&needle_path)?;
        let patterns: Vec<_> = io::BufReader::new(file)
            .lines()
            .map(|l| l.unwrap())
            .collect();
        let mut patterns_to_sort = patterns
            .iter()
            .cloned()
            .enumerate()
            .map(|(i, p)| (p, i as u32))
            .collect::<Vec<_>>();

        let start = Instant::now();
        patterns_to_sort.sort_unstable();
        let sort_duration = start.elapsed();

        // 对每个文本文件进行测试
        for haystack_entry in fs::read_dir(haystack_dir)? {
            let haystack_path = haystack_entry?.path();
            let haystack_name = haystack_path.file_stem().unwrap().to_str().unwrap();

            // 读取文本文件
            let text = fs::read_to_string(&haystack_path)?;

            // // fixed vector
            // GLOBAL.reset();

            // let start_build = Instant::now();
            // let mut ac_fixed_vector =
            //     aho_corasick_fixed_vector::AhoCorasick::with_num_patterns(patterns.len());
            // for pattern in patterns.iter() {
            //     ac_fixed_vector.add_pattern(pattern);
            // }
            // ac_fixed_vector.build();
            // let build_duration_fixed_vector = start_build.elapsed();

            // let start_count = Instant::now();
            // let counts_fixed_vector: Vec<_> = ac_fixed_vector.get_pattern_counts(&text).collect();
            // let count_duration_fixed_vector = start_count.elapsed();

            // let heap_usage_fixed_vector = GLOBAL.get();

            // bitmap
            GLOBAL.reset();
            let start_build = Instant::now();
            let mut ac_bitmap = aho_corasick_bitmap::AhoCorasick::new(&patterns_to_sort);
            let build_duration_bitmap = start_build.elapsed();
            let start_count = Instant::now();
            let counts_bitmap: Vec<_> = ac_bitmap.get_pattern_counts(&text).collect();
            let count_duration_bitmap = start_count.elapsed();
            let start_first_matches = Instant::now();
            let first_matches_bitmap: Vec<_> = ac_bitmap.get_first_matches(&text).collect();
            let first_matches_duration_bitmap = start_first_matches.elapsed();

            let heap_usage_bitmap = GLOBAL.get();

            // // hashmap
            // GLOBAL.reset();
            // let start_build = Instant::now();
            // let mut ac_hashmap =
            //     aho_corasick_hashmap::AhoCorasick::with_num_patterns(patterns.len());
            // for pattern in patterns.iter() {
            //     ac_hashmap.add_pattern(pattern);
            // }
            // ac_hashmap.build();
            // let build_duration_hashmap = start_build.elapsed();
            // let start_count = Instant::now();
            // let counts_hashmap: Vec<_> = ac_hashmap.get_pattern_counts(&text).collect();
            // let count_duration_hashmap = start_count.elapsed();

            // let heap_usage_hashmap = GLOBAL.get();

            // // crate
            // GLOBAL.reset();
            // let mut counts_crate = vec![0; patterns.len()];
            // let start_build = Instant::now();
            // let ac = AhoCorasick::new(&patterns).unwrap();
            // let build_duration_crate = start_build.elapsed();

            // let start_count = Instant::now();
            // let mut state = OverlappingState::start();
            // ac.find_overlapping(&text, &mut state);
            // while let Some(mat) = state.get_match() {
            //     counts_crate[mat.pattern().as_usize()] += 1;
            //     ac.find_overlapping(&text, &mut state);
            // }
            // let count_duration_crate = start_count.elapsed();

            // let heap_usage_crate = GLOBAL.get();

            // // 输出结果
            // let output_path = format!(
            //     "./output/exp_4/fixed_vector/{}_{}.txt",
            //     needle_name, haystack_name
            // );
            // let mut output_file = File::create(output_path)?;
            // writeln!(
            //     output_file,
            //     "Build Time: {:?}, Count Time: {:?}, Total Time: {:?}, Heap Usage: {:?}",
            //     build_duration_fixed_vector,
            //     count_duration_fixed_vector,
            //     build_duration_fixed_vector + count_duration_fixed_vector,
            //     heap_usage_fixed_vector
            // )?;
            // for count in counts_fixed_vector {
            //     writeln!(output_file, "{}", count)?;
            // }

            let output_path = format!(
                "./output/exp_4/bitmap/{}_{}.txt",
                needle_name, haystack_name
            );
            let mut output_file = File::create(output_path)?;
            writeln!(
                output_file,
                "Sort Time: {:?}, Build Time: {:?}, First Matches Time: {:?}, Count Time: {:?}, Heap Usage: {:?}",
                sort_duration,
                build_duration_bitmap,
                first_matches_duration_bitmap,
                count_duration_bitmap,
                heap_usage_bitmap
            )?;
            for i in 0..patterns.len() {
                writeln!(
                    output_file,
                    "{}, {}, {}",
                    counts_bitmap[i], first_matches_bitmap[i], patterns[i]
                )?;
            }

            // let output_path = format!("./output/exp_4/crate/{}_{}.txt", needle_name, haystack_name);
            // let mut output_file = File::create(output_path)?;
            // writeln!(
            //     output_file,
            //     "Build Time: {:?}, Count Time: {:?}, Total Time: {:?}, Heap Usage: {:?}",
            //     build_duration_crate,
            //     count_duration_crate,
            //     build_duration_crate + count_duration_crate,
            //     heap_usage_crate
            // )?;
            // for count in counts_crate {
            //     writeln!(output_file, "{}", count)?;
            // }

            // let output_path = format!(
            //     "./output/exp_4/hashmap/{}_{}.txt",
            //     needle_name, haystack_name
            // );
            // let mut output_file = File::create(output_path)?;
            // writeln!(
            //     output_file,
            //     "Build Time: {:?}, Count Time: {:?}, Total Time: {:?}, Heap Usage: {:?}",
            //     build_duration_hashmap,
            //     count_duration_hashmap,
            //     build_duration_hashmap + count_duration_hashmap,
            //     heap_usage_hashmap
            // )?;
            // for count in counts_hashmap {
            //     writeln!(output_file, "{}", count)?;
            // }

            // writeln!(
            //     bench_file,
            //     "Dict: {}, Text: {}, Dict items: {}, Dict length: {}, Text length: {}",
            //     needle_name,
            //     haystack_name,
            //     patterns.len(),
            //     patterns.iter().map(|p| p.len()).sum::<usize>(),
            //     text.len()
            // )?;
            // writeln!(bench_file, "Fixed vector:")?;
            // writeln!(
            //     bench_file,
            //     "Build time: {:?}, Count time: {:?}, Total time: {:?}, Heap Usage: {:?}",
            //     build_duration_fixed_vector,
            //     count_duration_fixed_vector,
            //     build_duration_fixed_vector + count_duration_fixed_vector,
            //     heap_usage_fixed_vector
            // )?;
            writeln!(bench_file, "Bitmap:")?;
            writeln!(
                bench_file,
                "Sort time: {:?}, Build time: {:?}, Count time: {:?}, Total time: {:?}, Heap Usage: {:?}",
                sort_duration,
                build_duration_bitmap,
                count_duration_bitmap,
                sort_duration + build_duration_bitmap + count_duration_bitmap,
                heap_usage_bitmap
            )?;
            // writeln!(bench_file, "Hashmap:")?;
            // writeln!(
            //     bench_file,
            //     "Build time: {:?}, Count time: {:?}, Total time: {:?}, Heap Usage: {:?}",
            //     build_duration_hashmap,
            //     count_duration_hashmap,
            //     build_duration_hashmap + count_duration_hashmap,
            //     heap_usage_hashmap
            // )?;
            // writeln!(bench_file, "Crate:")?;
            // writeln!(
            //     bench_file,
            //     "Build time: {:?}, Count time: {:?}, Total time: {:?}, Heap Usage: {:?}",
            //     build_duration_crate,
            //     count_duration_crate,
            //     build_duration_crate + count_duration_crate,
            //     heap_usage_crate
            // )?;
            writeln!(bench_file, "--------------------------------")?;
        }
    }

    Ok(())
}

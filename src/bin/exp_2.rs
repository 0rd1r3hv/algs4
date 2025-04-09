use algs4::sort::{*, bench::*};

fn main() {
    let mut size = 1;
    for i in 1..=10 {
        size *= 5;
        println!("Scale: 5^{} = {}", i, size);

        let random_array: Vec<i32> = generate_random_array(size);

        if i <= 7 {
            test_sort("Insertion Sort", insertion_sort, &random_array);
        }

        test_sort("Top-down Mergesort", top_down_merge_sort, &random_array);
        test_sort("Bottom-up Mergesort", bottom_up_merge_sort, &random_array);
        test_sort("Random Quicksort", quick_sort, &random_array);
        test_sort("Quicksort with Dijkstra 3-way Partition)", quick_sort_3way, &random_array);
        println!("----------------------------------------");
    }
}
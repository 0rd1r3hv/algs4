use std::cmp::Ordering;

pub type Comparator<T> = fn(&T, &T) -> Ordering;
pub type Sorter<T> = fn(&mut [T], Comparator<T>);
use crate::tick;

pub const BENCH_MEMORY: bool = false;

pub fn insertion_sort<T>(a: &mut [T], is_less: Comparator<T>) {
    if BENCH_MEMORY {
        tick!();
    }
    for i in 1..a.len() {
        let mut j = i;
        while j > 0 && is_less(&a[j], &a[j - 1]) == Ordering::Less {
            a.swap(j, j - 1);
            j -= 1;
        }
    }
}

fn merge<T: Copy>(
    a: &mut [T],
    aux: &mut [T],
    lo: usize,
    mid: usize,
    hi: usize,
    is_less: Comparator<T>,
) {
    if BENCH_MEMORY {
        tick!();
    }
    let mut i = lo;
    let mut j = mid + 1;

    for k in lo..=hi {
        aux[k] = a[k];
    }
    for k in lo..=hi {
        if i > mid {
            a[k] = aux[j];
            j += 1;
        } else if j > hi {
            a[k] = aux[i];
            i += 1;
        } else if is_less(&aux[j], &aux[i]) == Ordering::Less {
            a[k] = aux[j];
            j += 1;
        } else {
            a[k] = aux[i];
            i += 1;
        }
    }
}

fn top_down_merge_sort_aux<T: Copy>(
    a: &mut [T],
    aux: &mut [T],
    lo: usize,
    hi: usize,
    is_less: Comparator<T>,
) {
    if BENCH_MEMORY {
        tick!();
    }
    if hi <= lo {
        return;
    }
    let mid = lo + (hi - lo) / 2;
    top_down_merge_sort_aux(a, aux, lo, mid, is_less);
    top_down_merge_sort_aux(a, aux, mid + 1, hi, is_less);
    merge(a, aux, lo, mid, hi, is_less);
}

pub fn top_down_merge_sort<T: Copy>(a: &mut [T], is_less: Comparator<T>) {
    if BENCH_MEMORY {
        tick!();
    }
    let aux = &mut a.to_vec();
    top_down_merge_sort_aux(a, aux, 0, a.len() - 1, is_less);
}

pub fn bottom_up_merge_sort<T: Copy>(a: &mut [T], is_less: Comparator<T>) {
    if BENCH_MEMORY {
        tick!();
    }
    use std::cmp::min;
    let aux = &mut a.to_vec();
    let n = a.len();
    let mut sz = 1;
    while sz < n {
        let mut lo = 0;
        while lo < n - sz {
            let hi = min(lo + (sz << 1) - 1, n - 1);
            merge(a, aux, lo, lo + sz - 1, hi, is_less);
            lo += sz << 1;
        }
        sz <<= 1;
    }
}

fn partition<T: Copy>(a: &mut [T], lo: usize, hi: usize, is_less: Comparator<T>) -> usize {
    if BENCH_MEMORY {
        tick!();
    }
    let mut i = lo;
    let mut j = hi + 1;
    let v = a[lo];
    loop {
        i += 1;
        while i < hi && is_less(&a[i], &v) == Ordering::Less {
            i += 1;
        }
        j -= 1;
        while j > lo && is_less(&v, &a[j]) == Ordering::Less {
            j -= 1;
        }
        if i >= j {
            break;
        }
        a.swap(i, j);
    }
    a.swap(lo, j);
    j
}

fn quick_sort_aux<T: Copy>(a: &mut [T], lo: usize, hi: usize, is_less: Comparator<T>) {
    if BENCH_MEMORY {
        tick!();
    }
    if hi <= lo {
        return;
    }
    let j = partition(a, lo, hi, is_less);
    if j >= 1 {
        quick_sort_aux(a, lo, j - 1, is_less);
    }
    quick_sort_aux(a, j + 1, hi, is_less);
}

pub fn quick_sort<T: Copy>(a: &mut [T], is_less: Comparator<T>) {
    if BENCH_MEMORY {
        tick!();
    }
    use rand::{prelude::SliceRandom, rng};
    a.shuffle(&mut rng());
    quick_sort_aux(a, 0, a.len() - 1, is_less);
}

fn quick_sort_3way_aux<T: Copy>(a: &mut [T], lo: usize, hi: usize, is_less: Comparator<T>) {
    if BENCH_MEMORY {
        tick!();
    }
    if hi <= lo {
        return;
    }
    let mut lt = lo;
    let mut gt = hi;
    let v = a[lo];
    let mut i = lo + 1;
    while i <= gt {
        match is_less(&a[i], &v) {
            Ordering::Less => {
                a.swap(lt, i);
                lt += 1;
                i += 1;
            }
            Ordering::Equal => {
                i += 1;
            }
            Ordering::Greater => {
                a.swap(i, gt);
                gt -= 1;
            }
        }
    }
    if lt >= 1 {
        quick_sort_3way_aux(a, lo, lt - 1, is_less);
    }
    quick_sort_3way_aux(a, gt + 1, hi, is_less);
}

pub fn quick_sort_3way<T: Copy>(a: &mut [T], is_less: Comparator<T>) {
    if BENCH_MEMORY {
        tick!();
    }
    use rand::{prelude::SliceRandom, rng};
    a.shuffle(&mut rng());
    quick_sort_3way_aux(a, 0, a.len() - 1, is_less);
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{distr::StandardUniform, prelude::Distribution, random};

    const N: usize = 5000;

    fn generate_random_vec_and_is_less<T>() -> (Vec<T>, Comparator<T>)
    where
        T: Copy + Ord,
        StandardUniform: Distribution<T>,
    {
        let a: Vec<T> = (0..N).map(|_| random::<T>()).collect();
        (a, |a, b| a.cmp(b))
    }

    #[test]
    fn test_insertion_sort() {
        let (mut a, is_less) = generate_random_vec_and_is_less::<i32>();
        insertion_sort(&mut a, is_less);
        assert!(a.is_sorted());
    }

    #[test]
    fn test_top_down_merge_sort() {
        let (mut a, is_less) = generate_random_vec_and_is_less::<i32>();
        top_down_merge_sort(&mut a, is_less);
        assert!(a.is_sorted());
    }

    #[test]
    fn test_bottom_up_merge_sort() {
        let (mut a, is_less) = generate_random_vec_and_is_less::<i32>();
        bottom_up_merge_sort(&mut a, is_less);
        assert!(a.is_sorted());
    }

    #[test]
    fn test_quick_sort() {
        let (mut a, is_less) = generate_random_vec_and_is_less::<i32>();
        quick_sort(&mut a, is_less);
        assert!(a.is_sorted());
    }

    #[test]
    fn test_quick_sort_3way() {
        let (mut a, is_less) = generate_random_vec_and_is_less::<i32>();
        quick_sort_3way(&mut a, is_less);
        assert!(a.is_sorted());
    }
}

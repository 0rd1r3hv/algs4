use likely_stable::{likely, unlikely};
use std::ptr;

#[derive(Debug)]
pub struct DaryHeap<T, const D: usize> {
    data: Vec<T>,
    _assertion: AssertD<D>,
}

#[derive(Debug)]
struct AssertD<const D: usize>;

impl<const D: usize> AssertD<D> {
    const _ASSERTION: () = assert!(D >= 2, "D must be at least 2");
}

impl<T: Ord, const D: usize> DaryHeap<T, D> {
    #[inline]
    pub fn new() -> Self {
        DaryHeap {
            data: Vec::new(),
            _assertion: AssertD::<D>,
        }
    }

    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        DaryHeap {
            data: Vec::with_capacity(capacity),
            _assertion: AssertD::<D>,
        }
    }

    #[inline]
    pub fn peek(&self) -> Option<&T> {
        self.data.first()
    }

    #[inline]
    pub fn push(&mut self, item: T) {
        let mut child = self.data.len();
        self.data.push(item);
        let item = unsafe { ptr::read(self.get(child)) };
        while child > 0 {
            let parent = (child - 1) / D;
            if item <= *self.get(parent) {
                break;
            }
            unsafe {
                ptr::copy_nonoverlapping(self.data.as_ptr().add(parent), self.data.as_mut_ptr().add(child), 1);
            }
            child = parent;
        }
        *self.get_mut(child) = item;
    }

    #[inline]
    pub fn pop(&mut self) -> Option<T> {
        match self.data.len() {
            0 => None,
            len => {
                let last_idx = len - 1;
                let ret = unsafe { ptr::read(self.data.as_ptr()) };
                if likely(last_idx != 0) {
                    let item = unsafe { ptr::read(self.get(last_idx)) };
                    let mut parent = 0;
                    let mut child = 1;
                    let mut max_idx;
                    while child + D <= last_idx {
                        max_idx = child;
                        for i in child..(child + D) {
                            if self.get(i) > self.get(max_idx) {
                                max_idx = i;
                            }
                        }

                        if *self.get(max_idx) <= item {
                            *self.get_mut(parent) = item;
                            unsafe {
                                self.data.set_len(last_idx);
                                return Some(ret);
                            }
                        }

                        unsafe {
                            ptr::copy_nonoverlapping(
                                self.data.as_ptr().add(max_idx),
                                self.data.as_mut_ptr().add(parent),
                                1,
                            );
                        }
                        parent = max_idx;
                        child = parent * D + 1;
                    }
                    if unlikely(child < last_idx) {
                        max_idx = child;
                        for i in child..last_idx {
                            if self.get(i) > self.get(max_idx) {
                                max_idx = i;
                            }
                        }
                        if *self.get(max_idx) > item {
                            unsafe {
                                ptr::copy_nonoverlapping(
                                    self.data.as_ptr().add(max_idx),
                                    self.data.as_mut_ptr().add(parent),
                                    1,
                                );
                            }
                            parent = max_idx;
                        }
                    }
                    *self.get_mut(parent) = item;
                }
                unsafe {
                    self.data.set_len(last_idx);
                    Some(ret)
                }
            }
        }
    }

    #[inline]
    pub fn clear(&mut self) {
        self.data.clear();
    }

    #[inline]
    fn get(&self, idx: usize) -> &T {
        unsafe { self.data.get_unchecked(idx) }
    }

    #[inline]
    fn get_mut(&mut self, idx: usize) -> &mut T {
        unsafe { self.data.get_unchecked_mut(idx) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    #[test]
    fn test_random_operations() {
        let mut rng = rand::rng();
        let mut heap: DaryHeap<i32, 4> = DaryHeap::new();
        let mut vec = Vec::new();

        // 随机插入10000个数字
        for _ in 0..10000 {
            let num = rng.random_range(i32::MIN..i32::MAX);
            heap.push(num);
            vec.push(num);
        }

        // 验证堆的性质
        vec.sort_unstable_by(|a, b| b.cmp(a));
        vec.iter().for_each(|&num| {
            assert_eq!(heap.pop(), Some(num));
        });
        assert_eq!(heap.pop(), None);
    }
}

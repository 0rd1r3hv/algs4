use likely_stable::{likely, unlikely};
use std::{fmt::Debug, ptr};
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
        if unlikely(self.data.len() == 0) {
            self.data.push(item);
            return;
        } else {
            let mut child = self.data.len();
            let mut parent = (child - 1) / D;
            if unsafe { item <= *self.get(parent) } {
                self.data.push(item);
                return;
            }
            self.data.push(unsafe { ptr::read(self.get(parent)) });
            child = parent;
            while child > 0 {
                parent = (child - 1) / D;
                if unsafe { item <= *self.get(parent) } {
                    unsafe {
                        *self.get_mut(child) = item;
                    }
                    return;
                }
                unsafe {
                    ptr::copy_nonoverlapping(
                        self.data.as_ptr().add(parent),
                        self.data.as_mut_ptr().add(child),
                        1,
                    );
                }
                child = parent;
            }
            unsafe {
                *self.get_mut(child) = item;
            }
        }
    }

    #[inline]
    pub fn pop(&mut self) -> Option<T> {
        if unlikely(self.data.len() == 0) {
            return None;
        } else {
            let last_idx = self.data.len() - 1;
            let ret = unsafe { ptr::read(self.data.as_ptr()) };
            if likely(last_idx != 0) {
                let item = unsafe { ptr::read(self.get(last_idx)) };
                let mut parent = 0;
                let mut child = 1;
                while child <= last_idx.saturating_sub(D) {
                    if D == 2 {
                        child += unsafe { self.get(child + 1) > self.get(child) } as usize;
                    } else {
                        for i in (child + 1)..(child + D) {
                            if unsafe { self.get(i) > self.get(child) } {
                                child = i;
                            }
                        }
                    }

                    unsafe {
                        ptr::copy_nonoverlapping(
                            self.data.as_ptr().add(child),
                            self.data.as_mut_ptr().add(parent),
                            1,
                        );
                    }
                    parent = child;
                    child = parent * D + 1;
                }
                if unlikely(child < last_idx) {
                    if D > 2 {
                        for i in (child + 1)..last_idx {
                            if unsafe { self.get(i) > self.get(child) } {
                                child = i;
                            }
                        }
                    }

                    unsafe {
                        ptr::copy_nonoverlapping(
                            self.data.as_ptr().add(child),
                            self.data.as_mut_ptr().add(parent),
                            1,
                        );
                    }
                    parent = child;
                }

                while parent > 0 {
                    let parent_parent = (parent - 1) / D;
                    if unsafe { item <= *self.get(parent_parent) } {
                        break;
                    }
                    unsafe {
                        ptr::copy_nonoverlapping(
                            self.data.as_ptr().add(parent_parent),
                            self.data.as_mut_ptr().add(parent),
                            1,
                        );
                    }
                    parent = parent_parent;
                }

                unsafe {
                    *self.get_mut(parent) = item;
                }
            }
            unsafe {
                self.data.set_len(last_idx);
            }
            Some(ret)
        }
    }

    #[inline]
    pub fn clear(&mut self) {
        self.data.clear();
    }

    #[inline]
    unsafe fn get(&self, idx: usize) -> &T {
        unsafe { self.data.get_unchecked(idx) }
    }

    #[inline]
    unsafe fn get_mut(&mut self, idx: usize) -> &mut T {
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
        let mut heap: DaryHeap<i32, 2> = DaryHeap::new();
        let mut vec = Vec::new();

        (0..1000000).for_each(|_| {
            let num = rng.random_range(i32::MIN..i32::MAX);
            heap.push(num);
            vec.push(num);
        });

        vec.sort_unstable_by(|a, b| b.cmp(a));
        vec.iter().for_each(|&num| {
            assert_eq!(heap.pop(), Some(num));
        });
        assert_eq!(heap.pop(), None);
    }
}

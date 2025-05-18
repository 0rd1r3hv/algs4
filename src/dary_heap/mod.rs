use std::iter::FromIterator;

#[derive(Debug)]
pub struct DaryHeap<T, const D: usize> {
    data: Vec<T>,
}

impl<T: Ord, const D: usize> DaryHeap<T, D> {
    pub fn new() -> Self {
        assert!(D >= 2, "D must be at least 2");
        DaryHeap { data: Vec::new() }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        assert!(D >= 2, "D must be at least 2");
        DaryHeap {
            data: Vec::with_capacity(capacity),
        }
    }

    pub fn push(&mut self, item: T) {
        self.data.push(item);
        self.sift_up(self.data.len() - 1);
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.data.is_empty() {
            return None;
        }
        let last_idx = self.data.len() - 1;
        self.data.swap(0, last_idx);
        let result = self.data.pop();
        if !self.data.is_empty() {
            self.sift_down(0);
        }
        result
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }

    pub fn peek(&self) -> Option<&T> {
        self.data.first()
    }

    fn sift_up(&mut self, mut idx: usize) {
        while idx > 0 {
            let parent = (idx - 1) / D;
            if self.data[idx] <= self.data[parent] {
                break;
            }
            self.data.swap(idx, parent);
            idx = parent;
        }
    }

    fn sift_down(&mut self, mut idx: usize) {
        let len = self.data.len();
        loop {
            let mut max_idx = idx;
            let start = idx * D + 1;
            let end = (start + D).min(len);

            for i in start..end {
                if self.data[i] > self.data[max_idx] {
                    max_idx = i;
                }
            }

            if max_idx == idx {
                break;
            }

            self.data.swap(idx, max_idx);
            idx = max_idx;
        }
    }
}

impl<T: Ord, const D: usize> Default for DaryHeap<T, D> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Ord, const D: usize> From<Vec<T>> for DaryHeap<T, D> {
    fn from(vec: Vec<T>) -> Self {
        let mut heap = DaryHeap { data: vec };
        for i in (0..heap.data.len() / D).rev() {
            heap.sift_down(i);
        }
        heap
    }
}

impl<T: Ord, const D: usize> FromIterator<T> for DaryHeap<T, D> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let vec: Vec<T> = iter.into_iter().collect();
        vec.into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    #[test]
    fn test_basic_operations() {
        let mut heap: DaryHeap<i32, 3> = DaryHeap::new();
        heap.push(1);
        heap.push(2);
        heap.push(3);
        assert_eq!(heap.peek(), Some(&3));
        assert_eq!(heap.pop(), Some(3));
        assert_eq!(heap.pop(), Some(2));
        assert_eq!(heap.pop(), Some(1));
        assert_eq!(heap.pop(), None);
    }

    #[test]
    fn test_random_operations() {
        let mut rng = rand::rng();
        let mut heap: DaryHeap<i32, 4> = DaryHeap::new();
        let mut vec = Vec::new();

        // 随机插入1000个数字
        for _ in 0..1000 {
            let num = rng.random_range(-1000..1000);
            heap.push(num);
            vec.push(num);
        }

        // 验证堆的性质
        vec.sort_unstable_by(|a, b| b.cmp(a));
        for &expected in &vec {
            assert_eq!(heap.pop(), Some(expected));
        }
        assert_eq!(heap.pop(), None);
    }

    #[test]
    fn test_from_iter() {
        let vec = vec![1, 2, 3, 4, 5];
        let heap: DaryHeap<i32, 3> = vec.into_iter().collect();
        assert_eq!(heap.peek(), Some(&5));
    }
}

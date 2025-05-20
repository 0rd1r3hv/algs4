pub trait UnionFind {
    fn new(size: usize) -> Self;
    fn find(&mut self, p: usize) -> usize;
    #[inline]
    fn connected(&mut self, p: usize, q: usize) -> bool {
        self.find(p) == self.find(q)
    }
    fn union(&mut self, p: usize, q: usize);
    fn reset(&mut self);
}

pub struct QuickUnion {
    id: Vec<usize>,
}

pub struct WeightedQuickUnion {
    id: Vec<usize>,
    sz: Vec<usize>,
}

pub struct WeightedQuickUnionWithPathCompression {
    id: Vec<usize>,
    sz: Vec<usize>,
}

impl QuickUnion {
    #[inline]
    fn get_id(&self, p: usize) -> &usize {
        unsafe { self.id.get_unchecked(p) }
    }
}

impl UnionFind for QuickUnion {
    #[inline]
    fn new(size: usize) -> Self {
        let id = (0..size).collect();
        Self { id }
    }

    #[inline]
    fn find(&mut self, p: usize) -> usize {
        *self.get_id(p)
    }

    #[inline]
    fn union(&mut self, p: usize, q: usize) {
        let rt_p = self.find(p);
        let rt_q = self.find(q);

        if rt_p == rt_q {
            return;
        }

        self.id
            .iter_mut()
            .filter(|x| **x == rt_p)
            .for_each(|x| *x = rt_q);
    }

    #[inline]
    fn reset(&mut self) {
        self.id.iter_mut().enumerate().for_each(|(i, x)| *x = i);
    }
}

impl WeightedQuickUnion {
    #[inline]
    fn get_id(&self, p: usize) -> &usize {
        unsafe { self.id.get_unchecked(p) }
    }

    #[inline]
    fn get_id_mut(&mut self, p: usize) -> &mut usize {
        unsafe { self.id.get_unchecked_mut(p) }
    }

    #[inline]
    fn get_sz(&self, p: usize) -> &usize {
        unsafe { self.sz.get_unchecked(p) }
    }

    #[inline]
    fn get_sz_mut(&mut self, p: usize) -> &mut usize {
        unsafe { self.sz.get_unchecked_mut(p) }
    }
}

impl UnionFind for WeightedQuickUnion {
    #[inline]
    fn new(size: usize) -> Self {
        let id = (0..size).collect();
        let sz = vec![1; size];
        Self { id, sz }
    }

    #[inline]
    fn find(&mut self, mut p: usize) -> usize {
        while p != *self.get_id(p) {
            p = *self.get_id(p);
        }
        p
    }

    #[inline]
    fn union(&mut self, p: usize, q: usize) {
        let rt_p = self.find(p);
        let rt_q = self.find(q);
        if rt_p == rt_q {
            return;
        }

        if self.get_sz(rt_p) < self.get_sz(rt_q) {
            *self.get_id_mut(rt_p) = rt_q;
            *self.get_sz_mut(rt_q) += *self.get_sz(rt_p);
        } else {
            *self.get_id_mut(rt_q) = rt_p;
            *self.get_sz_mut(rt_p) += *self.get_sz(rt_q);
        }
    }

    #[inline]
    fn reset(&mut self) {
        self.id.iter_mut().enumerate().for_each(|(i, x)| *x = i);
        self.sz.fill(1);
    }
}

impl WeightedQuickUnionWithPathCompression {
    #[inline]
    fn get_id(&self, p: usize) -> &usize {
        unsafe { self.id.get_unchecked(p) }
    }

    #[inline]
    fn get_id_mut(&mut self, p: usize) -> &mut usize {
        unsafe { self.id.get_unchecked_mut(p) }
    }

    #[inline]
    fn get_sz(&self, p: usize) -> &usize {
        unsafe { self.sz.get_unchecked(p) }
    }

    #[inline]
    fn get_sz_mut(&mut self, p: usize) -> &mut usize {
        unsafe { self.sz.get_unchecked_mut(p) }
    }
}

impl UnionFind for WeightedQuickUnionWithPathCompression {
    #[inline]
    fn new(size: usize) -> Self {
        let id = (0..size).collect();
        let sz = vec![1; size];
        Self { id, sz }
    }

    #[inline]
    fn find(&mut self, mut p: usize) -> usize {
        let mut root = p;
        while root != *self.get_id(root) {
            root = *self.get_id(root);
        }
        while p != root {
            let next = *self.get_id(p);
            *self.get_id_mut(p) = root;
            p = next;
        }
        root
    }

    #[inline]
    fn union(&mut self, p: usize, q: usize) {
        let rt_p = self.find(p);
        let rt_q = self.find(q);
        if rt_p == rt_q {
            return;
        }

        if self.get_sz(rt_p) < self.get_sz(rt_q) {
            *self.get_id_mut(rt_p) = rt_q;
            *self.get_sz_mut(rt_q) += *self.get_sz(rt_p);
        } else {
            *self.get_id_mut(rt_q) = rt_p;
            *self.get_sz_mut(rt_p) += *self.get_sz(rt_q);
        }
    }

    #[inline]
    fn reset(&mut self) {
        self.id.iter_mut().enumerate().for_each(|(i, x)| *x = i);
        self.sz.fill(1);
    }
}

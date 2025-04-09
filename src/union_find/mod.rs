pub trait UnionFind {
    fn new(size: usize) -> Self;
    fn count(&self) -> usize;
    fn find(&mut self, p: usize) -> usize;
    fn connected(&mut self, p: usize, q: usize) -> bool {
        self.find(p) == self.find(q)
    }
    fn union(&mut self, p: usize, q: usize);
}

pub struct QuickUnion {
    id: Vec<usize>,
    count: usize,
}

pub struct WeightedQuickUnion {
    id: Vec<usize>,
    sz: Vec<usize>,
    count: usize,
}

pub struct WeightedQuickUnionWithPathCompression {
    id: Vec<usize>,
    sz: Vec<usize>,
    count: usize,
}

impl UnionFind for QuickUnion {
    fn new(size: usize) -> Self {
        let id = (0..size).collect();
        Self { id, count: size }
    }

    fn count(&self) -> usize {
        self.count
    }

    fn find(&mut self, p: usize) -> usize {
        self.id[p]
    }

    fn union(&mut self, p: usize, q: usize) {
        let rt_p = self.find(p);
        let rt_q = self.find(q);
        if rt_p == rt_q {
            return;
        }

        for idx in &mut self.id {
            if *idx == rt_p {
                *idx = rt_q;
            }
        }
    }
}

impl UnionFind for WeightedQuickUnion {
    fn new(size: usize) -> Self {
        let id = (0..size).collect();
        let sz = vec![1; size];
        Self {
            id,
            sz,
            count: size,
        }
    }

    fn count(&self) -> usize {
        self.count
    }

    fn find(&mut self, mut p: usize) -> usize {
        while p != self.id[p] {
            p = self.id[p];
        }
        p
    }

    fn union(&mut self, p: usize, q: usize) {

        let rt_p = self.find(p);
        let rt_q = self.find(q);
        if rt_p == rt_q {
            return;
        }

        if self.sz[rt_p] < self.sz[rt_q] {
            self.id[rt_p] = rt_q;
            self.sz[rt_q] += self.sz[rt_p];
        } else {
            self.id[rt_q] = rt_p;
            self.sz[rt_p] += self.sz[rt_q];
        }

        self.count -= 1;
    }
}

impl UnionFind for WeightedQuickUnionWithPathCompression {
    fn new(size: usize) -> Self {
        let id = (0..size).collect();
        let sz = vec![1; size];
        Self {
            id,
            sz,
            count: size,
        }
    }

    fn count(&self) -> usize {
        self.count
    }

    fn find(&mut self, mut p: usize) -> usize {
        let mut root = p;
        while root != self.id[root] {
            root = self.id[root];
        }
        while p != root {
            let next = self.id[p];
            self.id[p] = root;
            p = next;
        }
        root
    }

    fn union(&mut self, p: usize, q: usize) {

        let rt_p = self.find(p);
        let rt_q = self.find(q);
        if rt_p == rt_q {
            return;
        }

        if self.sz[rt_p] < self.sz[rt_q] {
            self.id[rt_p] = rt_q;
            self.sz[rt_q] += self.sz[rt_p];
        } else {
            self.id[rt_q] = rt_p;
            self.sz[rt_p] += self.sz[rt_q];
        }

        self.count -= 1;
    }
}

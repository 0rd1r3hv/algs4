use ethnum::U256;
use std::cmp::Ordering;

type Map = U256;

trait BitMap {
    fn get_index(self, c: u8) -> Option<usize>;
    fn set(&mut self, c: u8);
    fn get_char(self) -> Option<u8>;
    fn next(&mut self);
    fn get_count(self) -> usize;
}

impl BitMap for Map {
    #[inline]
    fn get_index(self, c: u8) -> Option<usize> {
        if self & (1 << c) == U256::ZERO {
            None
        } else {
            Some((self & ((1 << c) - 1)).count_ones() as usize)
        }
    }

    #[inline]
    fn set(&mut self, c: u8) {
        *self |= 1 << c;
    }

    #[inline]
    fn get_char(self) -> Option<u8> {
        if self == U256::ZERO {
            None
        } else {
            Some(self.trailing_zeros() as u8)
        }
    }

    #[inline]
    fn next(&mut self) {
        *self &= *self - 1;
    }

    #[inline]
    fn get_count(self) -> usize {
        self.count_ones() as usize
    }
}

pub struct AhoCorasick {
    trie: Vec<Node>,
    pattern_indices: Vec<u32>,
    stack: Vec<u32>,
}

#[derive(Clone)]
struct Node {
    map: Map,
    children: Vec<u32>,
    fail: u32,
    ans: u32,
}

impl Node {
    #[inline]
    fn new() -> Self {
        Node {
            map: U256::ZERO,
            children: Vec::new(),
            fail: 0,
            ans: 0,
        }
    }
}

impl AhoCorasick {
    pub fn new(patterns: &[(String, u32)]) -> Self {
        let mut trie = vec![Node::new()];
        let mut len_trie = 0;

        let mut pattern_indices = Vec::with_capacity(patterns.len());

        unsafe { pattern_indices.set_len(patterns.len()) };

        for (pattern, index) in patterns {
            let mut current = 0;
            for &b in pattern.as_bytes() {
                current = if let Some(index) = trie[current].map.get_index(b) {
                    trie[current].children[index] as usize
                } else {
                    len_trie += 1;
                    trie.push(Node::new());
                    trie[current].map.set(b);
                    trie[current].children.push(len_trie as u32);
                    len_trie
                };
            }
            pattern_indices[*index as usize] = current as u32;
        }

        let mut stack: Vec<u32> = trie[0].children.iter().copied().collect();
        let mut start = 0;

        while start < stack.len() {
            let u = stack[start];
            start += 1;
            let u = u as usize;
            let fail = trie[u].fail as usize;
            let mut map = trie[u].map;
            for i in 0..trie[u].children.len() {
                let b = map.get_char().unwrap_or_default();
                let child = trie[u].children[i] as usize;
                if let Some(index) = trie[fail].map.get_index(b) {
                    trie[child].fail = trie[fail].children[index];
                }
                stack.push(child as u32);
                map.next();
            }
            let mut map_u = trie[u].map;
            let mut map_fail = trie[fail].map;
            let diff_count = (map_fail & (!map_u)).get_count();
            if diff_count > 0 {
                trie[u].map |= map_fail;
                let len = trie[u].children.len() + diff_count;
                let mut merged_children = Vec::with_capacity(len);
                let mut i = 0;
                let mut j = 0;
                for _ in 0..len {
                    if map_u == 0 {
                        merged_children.push(trie[fail].children[j]);
                        j += 1;
                    } else if map_fail == 0 {
                        merged_children.push(trie[u].children[i]);
                        i += 1;
                    } else {
                        match map_u
                            .get_char()
                            .unwrap_or_default()
                            .cmp(&map_fail.get_char().unwrap_or_default())
                        {
                            Ordering::Less => {
                                merged_children.push(trie[u].children[i]);
                                i += 1;
                                map_u.next();
                            }
                            Ordering::Greater => {
                                merged_children.push(trie[fail].children[j]);
                                j += 1;
                                map_fail.next();
                            }
                            Ordering::Equal => {
                                merged_children.push(trie[u].children[i]);
                                i += 1;
                                j += 1;
                                map_u.next();
                                map_fail.next();
                            }
                        }
                    }
                }
                trie[u].children = merged_children;
            }
        }

        AhoCorasick {
            trie,
            pattern_indices,
            stack,
        }
    }

    pub fn get_pattern_counts<'a>(&'a mut self, text: &'a str) -> impl Iterator<Item = u32> + 'a {
        let mut current = 0;

        for &b in text.as_bytes() {
            current = if let Some(index) = self.trie[current].map.get_index(b) {
                self.trie[current].children[index] as usize
            } else {
                0
            };
            self.trie[current].ans += 1;
        }

        for &u in self.stack.iter().rev() {
            let u = u as usize;
            let fail = self.trie[u].fail as usize;
            self.trie[fail].ans += self.trie[u].ans;
        }

        self.pattern_indices
            .iter()
            .map(|&i| self.trie[i as usize].ans)
    }
}

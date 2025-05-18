use fnv::FnvHashMap;

type Char = u8;

pub struct AhoCorasick {
    trie: Vec<Node>,
    pattern_indices: Vec<u32>,
    stack: Vec<u32>,
}

#[derive(Clone, Debug)]
struct Node {
    children: FnvHashMap<Char, u32>,
    fail: u32,
    ans: u32,
}

impl Node {
    #[inline]
    fn new() -> Self {
        Node {
            children: FnvHashMap::default(),
            fail: 0,
            ans: 0,
        }
    }
}

impl AhoCorasick {
    #[inline]
    pub fn new() -> Self {
        AhoCorasick {
            trie: vec![Node::new()],
            pattern_indices: vec![],
            stack: vec![],
        }
    }

    #[inline]
    pub fn with_num_patterns(num_patterns: usize) -> Self {
        AhoCorasick {
            trie: vec![Node::new()],
            pattern_indices: Vec::with_capacity(num_patterns),
            stack: vec![],
        }
    }

    #[inline]
    pub fn add_pattern(&mut self, pattern: &str) {
        self.pattern_indices
            .push(pattern.as_bytes().iter().fold(0, |current, b| {
                match self.trie[current].children.get(b) {
                    Some(child) => *child as usize,
                    None => {
                        let len = self.trie.len();
                        self.trie[current].children.insert(*b, len as u32);
                        self.trie.push(Node::new());
                        len
                    }
                }
            }) as u32);
    }

    #[inline]
    pub fn build(&mut self) {
        self.stack = self.trie[0].children.values().copied().collect();
        let mut start = 0;
        let trie = &mut *self.trie;

        while start < self.stack.len() {
            let u = self.stack[start] as usize;
            let fail = trie[u].fail as usize;
            for (b, child) in trie[u].children.iter() {
                trie[*child as usize].fail = trie[fail].children.get(b).copied().unwrap_or(0);
                self.stack.push(*child);
            }
            let temp = trie[fail]
                .children
                .iter()
                .filter(|(b, _)| !trie[u].children.contains_key(b))
                .map(|(b, child)| (*b, *child))
                .collect::<Vec<_>>();
            for (b, child) in temp {
                trie[u].children.insert(b, child);
            }
            start += 1;
        }
    }

    #[inline]
    pub fn get_pattern_counts<'a>(&'a mut self, text: &'a str) -> impl Iterator<Item = u32> + 'a {
        let mut current = 0;
        text.as_bytes().iter().for_each(|b| {
            current = self.trie[current].children.get(&b).copied().unwrap_or(0) as usize;
            self.trie[current].ans += 1;
        });

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

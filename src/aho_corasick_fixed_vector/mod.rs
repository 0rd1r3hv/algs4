const NUM_CHARS: usize = 0x100;

pub struct AhoCorasick {
    trie: Vec<Node>,
    pattern_indices: Vec<u32>,
    stack: Vec<u32>,
}

#[derive(Clone, Debug)]
struct Node {
    children: Vec<u32>,
    fail: u32,
    ans: u32,
}

impl Node {
    #[inline]
    fn new() -> Self {
        Node {
            children: vec![0; NUM_CHARS],
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
            .push(
                pattern
                    .as_bytes()
                    .iter()
                    .map(|&b| b as usize)
                    .fold(0, |current, b| match self.trie[current].children[b] {
                        0 => {
                            self.trie[current].children[b] = self.trie.len() as u32;
                            self.trie.push(Node::new());
                            self.trie.len() - 1
                        }
                        child => child as usize,
                    }) as u32,
            );
    }

    #[inline]
    pub fn build(&mut self) {
        self.stack = self.trie[0]
            .children
            .iter()
            .filter(|&&c| c != 0)
            .copied()
            .collect();
        let mut start = 0;

        while start < self.stack.len() {
            let u = self.stack[start] as usize;
            let fail = self.trie[u].fail as usize;
            for b in 0..NUM_CHARS {
                match self.trie[u].children[b] {
                    0 => self.trie[u].children[b] = self.trie[fail].children[b],
                    child => {
                        self.trie[child as usize].fail = self.trie[fail].children[b];
                        self.stack.push(child);
                    }
                }
            }
            start += 1;
        }
    }

    #[inline]
    pub fn get_pattern_counts<'a>(&'a mut self, text: &'a str) -> impl Iterator<Item = u32> + 'a {
        let mut current = 0;
        text.as_bytes().iter().map(|&b| b as usize).for_each(|b| {
            current = self.trie[current].children[b] as usize;
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

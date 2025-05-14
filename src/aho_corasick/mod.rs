use std::collections::VecDeque;

pub struct AhoCorasick {
    trie: Vec<Node>,
}

struct Node {
    children: [usize; 128],
    fail: usize,
    output: Vec<usize>,
}

impl AhoCorasick {
    pub fn new() -> Self {
        let mut trie = Vec::with_capacity(1);
        trie.push(Node {
            children: [0; 128],
            fail: 0,
            output: Vec::new(),
        });
        AhoCorasick { trie }
    }

    pub fn add_pattern(&mut self, pattern: &str) -> usize {
        let mut current = 0;
        for &b in pattern.as_bytes() {
            let next = self.trie[current].children[b as usize];
            if next == 0 {
                let new_node = Node {
                    children: [0; 128],
                    fail: 0,
                    output: Vec::new(),
                };
                self.trie.push(new_node);
                let next = self.trie.len() - 1;
                self.trie[current].children[b as usize] = next;
            }
            current = self.trie[current].children[b as usize];
        }
        self.trie[current].output.push(pattern.len());
        current
    }

    pub fn build(&mut self) {
        let mut queue = VecDeque::new();
        for i in 0..128 {
            let next = self.trie[0].children[i];
            if next != 0 {
                queue.push_back(next);
            }
        }

        while let Some(current) = queue.pop_front() {
            for i in 0..128 {
                let next = self.trie[current].children[i];
                if next != 0 {
                    let mut fail = self.trie[current].fail;
                    while fail != 0 && self.trie[fail].children[i] == 0 {
                        fail = self.trie[fail].fail;
                    }
                    self.trie[next].fail = self.trie[fail].children[i];
                    let fail_output = self.trie[self.trie[next].fail].output.clone();
                    self.trie[next].output.extend(&fail_output);
                    queue.push_back(next);
                }
            }
        }
    }

    pub fn find_matches(&self, text: &str) -> Vec<(usize, usize)> {
        let mut matches = Vec::new();
        let mut current = 0;
        
        for (i, &b) in text.as_bytes().iter().enumerate() {
            while current != 0 && self.trie[current].children[b as usize] == 0 {
                current = self.trie[current].fail;
            }
            current = self.trie[current].children[b as usize];
            
            for &len in &self.trie[current].output {
                matches.push((i + 1 - len, i + 1));
            }
        }
        matches
    }
}

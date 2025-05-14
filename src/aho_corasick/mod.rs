use std::collections::VecDeque;

pub struct AhoCorasick {
    trie: Vec<Node>,
    num_patterns: usize,
    pattern_indices: Vec<usize>,
    in_degree: Vec<usize>,
}

#[derive(Clone)]
struct Node {
    children: [usize; 128],
    fail: usize,
    flag: usize,
    ans: usize,
}

impl AhoCorasick {
    pub fn new() -> Self {
        let trie = vec![Node { children: [0; 128], fail: 0, flag: 0, ans: 0 }; 2];
        AhoCorasick { 
            trie, 
            num_patterns: 0,
            pattern_indices: Vec::new(),
            in_degree: Vec::new(),
        }
    }

    pub fn add_pattern(&mut self, pattern: &str) -> usize {
        let mut current = 1;
        for &b in pattern.as_bytes() {
            let v = b as usize;
            if self.trie[current].children[v] == 0 {
                let new_node = Node {
                    children: [0; 128],
                    fail: 0,
                    flag: 0,
                    ans: 0,
                };
                self.trie.push(new_node);
                self.trie[current].children[v] = self.trie.len() - 1;
            }
            current = self.trie[current].children[v];
        }
        if self.trie[current].flag == 0 {
            self.trie[current].flag = self.num_patterns + 1;
        }
        self.pattern_indices.push(self.trie[current].flag);
        self.num_patterns += 1;
        current
    }

    pub fn build(&mut self) {
        self.in_degree = vec![0; self.trie.len()];
        let mut queue = VecDeque::new();

        for i in 0..128 {
            self.trie[0].children[i] = 1;
        }
        queue.push_back(1);

        while let Some(u) = queue.pop_front() {
            let fail = self.trie[u].fail;
            for i in 0..128 {
                let v = self.trie[u].children[i];
                if v == 0 {
                    self.trie[u].children[i] = self.trie[fail].children[i];
                    continue;
                }
                self.trie[v].fail = self.trie[fail].children[i];
                self.in_degree[self.trie[v].fail] += 1;
                queue.push_back(v);
            }
        }
    }

    pub fn find_matches(&self, text: &str) -> Vec<(usize, usize)> {
        let mut matches = Vec::new();
        let mut current = 0;
        
        for (i, &b) in text.as_bytes().iter().enumerate() {
            let v = b as usize;
            current = self.trie[current].children[v];
            
            for &pattern_idx in &self.pattern_indices {
                if self.trie[pattern_idx].flag != 0 {
                    let pattern_len = self.pattern_indices[pattern_idx] - pattern_idx;
                    matches.push((i + 1 - pattern_len, i + 1));
                }
            }
        }
        matches
    }

    pub fn get_pattern_counts(&mut self, text: &str) -> Vec<usize> {
        let mut counts = vec![0; self.num_patterns + 1];
        let mut current = 1;

        for &b in text.as_bytes() {
            let v = b as usize;
            current = self.trie[current].children[v];
            self.trie[current].ans += 1;
        }

        let mut queue = VecDeque::new();
        for i in 1..self.trie.len() {
            if self.in_degree[i] == 0 {
                queue.push_back(i);
            }
        }

        while let Some(u) = queue.pop_front() {
            if self.trie[u].flag != 0 {
                counts[self.trie[u].flag] = self.trie[u].ans;
            }
            let v = self.trie[u].fail;
            self.trie[v].ans += self.trie[u].ans;
            self.in_degree[v] -= 1;
            if self.in_degree[v] == 0 {
                queue.push_back(v);
            }
        }

        counts[1..].to_vec()
    }
}

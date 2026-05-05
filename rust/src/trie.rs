use crate::trie_node::TrieNode;
pub struct Trie<TValue: Clone> {
    nodes: Vec<TrieNode<TValue>>,
}

impl<TValue: Clone> Trie<TValue> {
    /// Initializes a new, empty Trie.
    pub fn new() -> Self {
        let mut nodes = Vec::new();
        nodes.push(TrieNode::new());
        Self { nodes }
    }

    pub fn with_capacity(cap: usize) -> Self {
        let mut nodes = Vec::with_capacity(cap);
        nodes.push(TrieNode::new());
        Self { nodes }
    }

    fn alloc_node(&mut self) -> u32 {
        let idx = self.nodes.len() as u32;
        self.nodes.push(TrieNode::new());
        idx
    }

    /// Inserts a key-value pair into the Trie.
    /// value is used to mark the end of the string or can contain a value if using the trie as a /// key value pair.
    ///  If the key already exists,
    /// its value is updated.
    ///
    /// Example:
    /// ```Rust
    /// let mut trie = Trie::new();
    /// trie.insert("apple", 1);
    /// assert_eq!(trie.get("apple"), Some(&1));
    /// ```
    pub fn insert(&mut self, key: &str, value: &TValue) {
        let mut current = 0u32; // root
        for c in key.chars() {
            let next = match self.nodes[current as usize].get_child(c) {
                Some(idx) => idx,
                None => {
                    let idx = self.alloc_node();
                    self.nodes[current as usize].add_child(c, idx);
                    idx
                }
            };
            current = next;
        }
        self.nodes[current as usize].set_value(value.clone());
    }

    /// Searches for a key and returns a reference to its value if it exists.
    ///
    /// Example:
    /// ```Rust
    /// let mut trie = Trie::new();
    /// trie.insert("apple", 1);
    /// assert_eq!(trie.get("apple"), Some(&1));
    /// ```
    pub fn get(&self, key: &str) -> Option<&TValue> {
        let mut current = 0u32;
        for c in key.chars() {
            match self.nodes[current as usize].get_child(c) {
                Some(idx) => current = idx,
                None => return None,
            }
        }
        self.nodes[current as usize].get_value()
    }

    /// Deletes a key and its associated value from the Trie.
    /// Returns true if the key was found and deleted, false otherwise.
    ///
    /// Example:
    /// ```Rust
    /// let mut trie = Trie::new();
    /// trie.insert("apple", 1);
    /// assert_eq!(trie.delete("apple"), true);
    /// assert_eq!(trie.get("apple"), None);
    /// ```
    pub fn delete(&mut self, key: &str) -> bool {
        if key.is_empty() {
            return false;
        }

        // Walk down, recording the path as (node_index, char) pairs
        let mut path: Vec<(u32, char)> = Vec::new();
        let mut current = 0u32;

        for c in key.chars() {
            match self.nodes[current as usize].get_child(c) {
                Some(idx) => {
                    path.push((current, c));
                    current = idx;
                }
                None => return false, // key doesn't exist
            }
        }

        // current is now the terminal node — check it's actually a word
        if !self.nodes[current as usize].is_end_of_word() {
            return false;
        }

        self.nodes[current as usize].clear_value();

        // Walk back up pruning nodes that are now dead
        // (no value, no children)
        let mut child = current;
        for (parent, c) in path.into_iter().rev() {
            if self.nodes[child as usize].has_children()
                || self.nodes[child as usize].is_end_of_word()
            {
                break; // still needed, stop pruning
            }
            self.nodes[parent as usize].remove_child(c);
            child = parent;
        }

        true
    }
    /// Checks if there is any word in the trie that starts with the given prefix.
    /// Returns true if such a prefix exists, false otherwise.
    /// Example:
    /// ```Rust
    /// let mut trie = Trie::new();
    /// trie.insert("apple", 1);
    /// assert_eq!(trie.prefix_search("app"), true);
    /// assert_eq!(trie.prefix_search("apl"), false);
    /// ```
    pub fn prefix_search(&self, prefix: &str) -> bool {
        let mut current = 0u32;
        for c in prefix.chars() {
            match self.nodes[current as usize].get_child(c) {
                Some(idx) => current = idx,
                None => return false,
            }
        }
        true
    }

    /// Returns up to `max_results` words in the trie that start with the given prefix.
    /// If no words match the prefix, returns an empty vector.
    /// Example:
    /// ```Rust
    /// let mut trie = Trie::new();
    /// trie.insert("apple", 1);
    /// trie.insert("app", 2);
    /// trie.insert("apricot", 3);
    /// let results = trie.auto_complete("ap", 2);
    /// assert_eq!(results, vec!["app", "apple"]);
    /// ```
    pub fn auto_complete(&self, prefix: &str, max_results: usize) -> Vec<String> {
        let mut results = Vec::new();
        if max_results == 0 {
            return results;
        }

        let mut current = 0u32;
        for c in prefix.chars() {
            match self.nodes[current as usize].get_child(c) {
                Some(idx) => current = idx,
                None => return results,
            }
        }

        if self.nodes[current as usize].is_end_of_word() {
            results.push(prefix.to_string());
            if results.len() >= max_results {
                return results;
            }
        }

        Self::collect_words_recursive(
            &self.nodes,
            current,
            prefix.to_string(),
            &mut results,
            max_results,
        );
        results
    }

    fn collect_words_recursive(
        nodes: &[TrieNode<TValue>],
        current: u32,
        curr_prefix: String,
        results: &mut Vec<String>,
        max_results: usize,
    ) {
        if results.len() >= max_results {
            return;
        }
        for (c, idx) in nodes[current as usize].children_iter() {
            if results.len() >= max_results {
                return;
            }
            let new_prefix = format!("{curr_prefix}{c}");
            if nodes[idx as usize].is_end_of_word() {
                results.push(new_prefix.clone());
            }
            Self::collect_words_recursive(nodes, idx, new_prefix, results, max_results);
        }
    }

    /// Adds multiple words to the trie from a list, using a value generator function
    /// to determine the value associated with each word.
    /// This is useful for bulk insertion where the value might depend on the word itself.
    /// Example:
    /// ```Rust
    /// let mut trie = Trie::new();
    /// trie.add_word_list(&["cat", "car", "cart"], |word| word.len());
    /// assert_eq!(trie.get("cat"), Some(&3));
    /// assert_eq!(trie.get("car"), Some(&3));
    /// assert_eq!(trie.get("cart"), Some(&4));
    /// ```
    pub fn add_word_list<T, F>(&mut self, items: &[T], value_generator: F)
    where
        T: AsRef<str>,
        F: Fn(&T) -> TValue,
    {
        for item in items {
            self.insert(item.as_ref(), &value_generator(item));
        }
    }
}

/// Allows creating a new Trie with `Trie::default()`.
impl<TValue: Clone> Default for Trie<TValue> {
    fn default() -> Self {
        Self::new()
    }
}

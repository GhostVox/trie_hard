use crate::trie_node::TrieNode;
pub struct Trie<TValue: Clone> {
    root: TrieNode<TValue>,
}

impl<TValue: Clone> Trie<TValue> {
    /// Initializes a new, empty Trie.
    pub fn new() -> Self {
        Trie {
            root: TrieNode::new(),
        }
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
        let mut current_node = &mut self.root;
        for c in key.chars() {
            current_node = current_node.add_child(c);
        }
        current_node.set_value(value.clone());
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
        let mut current_node = &self.root;
        for c in key.chars() {
            if let Some(node) = current_node.get_child(c) {
                current_node = node;
            } else {
                return None;
            }
        }
        // Return a reference to the value if it exists
        current_node.get_value()
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
        // We collect the chars to easily pass slices during recursion.
        let chars: Vec<char> = key.chars().collect();
        Self::delete_recursively(&mut self.root, &chars)
    }

    /// Recursive helper to delete a key. Returns true if the calling node
    /// should remove the child node from its children map (i.e., prune the branch).
    fn delete_recursively(current_node: &mut TrieNode<TValue>, key_slice: &[char]) -> bool {
        if key_slice.is_empty() {
            // We have reached the node corresponding to the key.
            if current_node.is_end_of_word() {
                current_node.clear_value();
                // Return true if this node has no children, so the parent can remove it.
                return !current_node.has_children();
            }
            // Key doesn't actually exist as a word in the trie.
            return false;
        }

        let c = key_slice[0];
        let should_delete_child = if let Some(child_node) = current_node.get_child_mut(c) {
            // Recurse with the rest of the key
            Self::delete_recursively(child_node, &key_slice[1..])
        } else {
            // The path for the key doesn't exist.
            return false;
        };

        if should_delete_child {
            current_node.remove_child(c);
            // After removing the child, if this current node is not the end of another word
            // and has no other children, it should also be deleted by its parent.
            return !current_node.is_end_of_word() && !current_node.has_children();
        }

        false
    }

    pub fn prefix_search(&self, prefix: &str) -> Vec<String> {
        let mut results = Vec::new();
        let mut current_node = &self.root;

        for c in prefix.chars() {
            if let Some(child_node) = current_node.get_child(c) {
                current_node = child_node;
            } else {
                return results;
            }
        }
        if current_node.is_end_of_word() {
            return results;
        }

        Self::collect_words_recursive(current_node, prefix.to_string(), &mut results);
        results
    }

    fn collect_words_recursive(
        node: &TrieNode<TValue>,
        curr_prefix: String,
        results: &mut Vec<String>,
    ) {
        if node.is_end_of_word() {
            results.push(curr_prefix.clone());
        }
        for (char, child) in node.children_iter() {
            let new_prefix = format!("{}{}", curr_prefix, char);
            Self::collect_words_recursive(child, new_prefix, results);
        }
    }

    pub fn add_word_list(&mut self, words: &[&str], value: TValue) {
        for word in words {
            self.insert(word, &value);
        }
    }
}

/// Allows creating a new Trie with `Trie::default()`.
impl<TValue: Clone> Default for Trie<TValue> {
    fn default() -> Self {
        Self::new()
    }
}

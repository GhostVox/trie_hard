use std::collections::HashMap;

/// Represents a node within a Trie. It is generic over the value it stores.
pub struct TrieNode<TValue> {
    // Note: The `character` for this node is the *key* in the parent's HashMap.
    // We don't need to store it inside the node itself.
    pub children: Vec<(char, u32)>,

    /// The value associated with the full word ending at this node.
    /// Using Option is key, as intermediate nodes won't have a value.
    value: Option<TValue>,
}

impl<TValue> TrieNode<TValue> {
    /// Creates a new, "empty" TrieNode without an initial value.
    /// This is the correct constructor for a node that isn't the end of a word yet.
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
            value: None,
        }
    }

    /// Checks if the node has any children. A node with no children is a "leaf".
    pub fn has_children(&self) -> bool {
        !self.children.is_empty()
    }

    /// Gets an immutable reference to a child node corresponding to the character.
    pub fn get_child(&self, character: char) -> Option<u32> {
        self.children
            .iter()
            .find(|(c, _)| *c == character)
            .map(|(_, idx)| *idx)
    }

    /// Gets a mutable reference to a child node corresponding to the character.
    pub fn get_child_mut(&mut self, character: char) -> Option<&mut u32> {
        self.children
            .iter_mut()
            .find(|(c, _)| *c == character)
            .map(|(_, idx)| idx)
    }

    /// Removes a child node.
    pub fn remove_child(&mut self, character: char) {
        self.children.retain(|(c, _)| *c != character);
    }

    /// Checks if this node represents the end of a complete word.
    pub fn is_end_of_word(&self) -> bool {
        self.value.is_some()
    }

    /// Gets a reference to the value stored in this node.
    pub fn get_value(&self) -> Option<&TValue> {
        self.value.as_ref()
    }

    pub fn children_iter(&self) -> impl Iterator<Item = (char, u32)> + '_ {
        self.children.iter().map(|(c, idx)| (*c, *idx))
    }

    // It's useful for the Trie to be able to set and clear the value.
    // These methods should be part of the node's public API.

    /// Sets the value for this node, marking it as the end of a word.
    pub fn set_value(&mut self, value: TValue) {
        self.value = Some(value);
    }

    /// Clears the value from this node, un-marking it as the end of a word.
    /// Returns the old value if one existed.
    pub fn clear_value(&mut self) -> Option<TValue> {
        self.value.take()
    }
    pub fn add_child(&mut self, character: char, idx: u32) {
        self.children.push((character, idx));
    }
}

// It's also idiomatic to implement the Default trait.
impl<TValue> Default for TrieNode<TValue> {
    fn default() -> Self {
        Self::new()
    }
}

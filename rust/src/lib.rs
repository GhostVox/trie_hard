pub mod trie;
mod trie_node;

#[cfg(test)]
mod tests {
    use crate::trie::Trie;

    #[test]
    fn test_insert() {
        let mut trie = Trie::<i32>::new();
        trie.insert("Cat", &1);
        if let Some(value) = trie.get("Cat") {
            assert_eq!(value.to_owned(), 1);
        } else {
            panic!("Expected value to be Some(1)");
        }
    }

    #[test]
    fn test_get() {
        let mut trie = Trie::<&'static str>::new();
        trie.insert("Cat", &"Mittens");
        if let Some(value) = trie.get("Cat") {
            assert_eq!(value.to_owned(), "Mittens");
        } else {
            panic!("Expected value to be Some(1)");
        }
    }

    #[test]
    fn test_remove() {
        let mut trie = Trie::<&'static str>::new();
        trie.insert("Cat", &"Mittens");
        assert!(trie.delete("Cat"));
    }

    #[test]
    fn test_remove_nonexistent() {
        let mut trie = Trie::<String>::new();
        trie.delete("Cat");
        if let Some(_value) = trie.get("Cat") {
            panic!("Expected value to be None");
        }
    }

    #[test]
    fn test_prefix_search() {
        let mut trie = Trie::<&'static str>::new();
        trie.insert("Cat", &"Mittens");
        trie.insert("Cats", &"Whiskers");
        let results = trie.prefix_search("Ca");
        assert_eq!(results, vec!["Cat", "Cats"]);
    }

    #[test]
    fn test_prefix_search_empty() {
        let mut trie = Trie::<&'static str>::new();
        trie.insert("Cat", &"Mittens");
        trie.insert("Cats", &"Whiskers");
        let results = trie.prefix_search("Dog");
        assert_eq!(results, Vec::<String>::new());
    }

    #[test]
    fn test_add_word_list() {
        let mut trie = Trie::<&'static str>::new();
        trie.add_word_list(&["Cat", "Cats"], &"Mittens");
        assert_eq!(trie.get("Cat"), Some(&"Mittens"));
        assert_eq!(trie.get("Cats"), Some(&"Mittens"));
    }
}

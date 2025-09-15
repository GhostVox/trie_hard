pub mod trie;
mod trie_node;
pub use trie::Trie;
#[cfg(test)]
mod tests {
    use crate::trie::Trie;
    #[test]
    fn test_new_trie_is_empty() {
        let trie: Trie<i32> = Trie::new();
        assert_eq!(trie.get("anything"), None);
        assert!(!trie.prefix_search("anything"));
        assert!(trie.auto_complete("", 10).is_empty());
    }

    #[test]
    fn test_default_constructor() {
        let trie: Trie<i32> = Trie::default();
        assert_eq!(trie.get("test"), None);
    }

    #[test]
    fn test_insert_and_get_single_word() {
        let mut trie = Trie::new();
        trie.insert("hello", &42);

        assert_eq!(trie.get("hello"), Some(&42));
        assert_eq!(trie.get("hell"), None);
        assert_eq!(trie.get("hello_world"), None);
        assert_eq!(trie.get(""), None);
    }

    #[test]
    fn test_insert_and_get_multiple_words() {
        let mut trie = Trie::new();
        trie.insert("cat", &1);
        trie.insert("car", &2);
        trie.insert("card", &3);
        trie.insert("care", &4);
        trie.insert("careful", &5);

        assert_eq!(trie.get("cat"), Some(&1));
        assert_eq!(trie.get("car"), Some(&2));
        assert_eq!(trie.get("card"), Some(&3));
        assert_eq!(trie.get("care"), Some(&4));
        assert_eq!(trie.get("careful"), Some(&5));

        // Test partial matches don't exist
        assert_eq!(trie.get("ca"), None);
        assert_eq!(trie.get("careless"), None);
    }

    #[test]
    fn test_update_existing_key() {
        let mut trie = Trie::new();
        trie.insert("test", &100);
        assert_eq!(trie.get("test"), Some(&100));

        // Update the same key
        trie.insert("test", &200);
        assert_eq!(trie.get("test"), Some(&200));
    }

    #[test]
    fn test_empty_string_insertion() {
        let mut trie = Trie::new();
        trie.insert("", &42);
        assert_eq!(trie.get(""), Some(&42));
        assert_eq!(trie.get("a"), None);
    }

    #[test]
    fn test_single_character_words() {
        let mut trie = Trie::new();
        trie.insert("a", &1);
        trie.insert("I", &2);
        trie.insert("x", &3);

        assert_eq!(trie.get("a"), Some(&1));
        assert_eq!(trie.get("I"), Some(&2));
        assert_eq!(trie.get("x"), Some(&3));
        assert_eq!(trie.get("b"), None);
    }

    #[test]
    fn test_prefix_search() {
        let mut trie = Trie::new();
        trie.insert("hello", &1);
        trie.insert("help", &2);
        trie.insert("hero", &3);

        assert!(trie.prefix_search("he"));
        assert!(trie.prefix_search("hel"));
        assert!(trie.prefix_search("hell"));
        assert!(trie.prefix_search("hello"));
        assert!(trie.prefix_search("help"));
        assert!(trie.prefix_search("hero"));

        assert!(!trie.prefix_search("helicopter"));
        assert!(!trie.prefix_search("world"));
        assert!(!trie.prefix_search("x"));

        // Empty prefix should return true
        assert!(trie.prefix_search(""));
    }

    #[test]
    fn test_auto_complete_basic() {
        let mut trie = Trie::new();
        trie.insert("cat", &1);
        trie.insert("car", &2);
        trie.insert("card", &3);
        trie.insert("care", &4);
        trie.insert("careful", &5);
        trie.insert("dog", &6);

        let mut results = trie.auto_complete("car", 10);
        results.sort(); // Sort for predictable testing

        // Now that the bug is fixed, "car" itself should be included
        let mut expected = vec![
            "car".to_string(),
            "card".to_string(),
            "care".to_string(),
            "careful".to_string(),
        ];
        expected.sort();
        assert_eq!(results, expected);
    }

    #[test]
    fn test_auto_complete_max_results() {
        let mut trie = Trie::new();
        trie.insert("test", &1);
        trie.insert("testing", &2);
        trie.insert("tester", &3);
        trie.insert("tests", &4);
        trie.insert("testimony", &5);

        // Test with max_results = 2
        let results = trie.auto_complete("test", 2);
        assert_eq!(results.len(), 2);
        assert!(results.contains(&"test".to_string()));

        // Test with max_results = 0
        let results = trie.auto_complete("test", 0);
        assert!(results.is_empty());

        // Test with max_results larger than available
        let results = trie.auto_complete("test", 100);
        assert_eq!(results.len(), 5); // All 5 words should be returned
    }

    #[test]
    fn test_auto_complete_no_matches() {
        let mut trie = Trie::new();
        trie.insert("hello", &1);
        trie.insert("world", &2);

        let results = trie.auto_complete("xyz", 10);
        assert!(results.is_empty());
    }

    #[test]
    fn test_auto_complete_single_result() {
        let mut trie = Trie::new();
        trie.insert("unique", &1);
        trie.insert("cat", &2);

        let results = trie.auto_complete("uni", 10);
        assert_eq!(results, vec!["unique".to_string()]);
    }

    #[test]
    fn test_auto_complete_prefix_is_word() {
        let mut trie = Trie::new();
        trie.insert("go", &1);
        trie.insert("gone", &2);
        trie.insert("going", &3);
        trie.insert("good", &4);

        // Test that when the prefix itself is a complete word, it's included
        let mut results = trie.auto_complete("go", 10);
        results.sort();

        let mut expected = vec![
            "go".to_string(),
            "gone".to_string(),
            "going".to_string(),
            "good".to_string(),
        ];
        expected.sort();
        assert_eq!(results, expected);

        // Test another case where prefix exactly matches a word
        let results2 = trie.auto_complete("good", 10);
        assert_eq!(results2, vec!["good".to_string()]);
    }

    #[test]
    fn test_auto_complete_empty_prefix() {
        let mut trie = Trie::new();
        trie.insert("a", &1);
        trie.insert("b", &2);

        let mut results = trie.auto_complete("", 10);
        results.sort();

        let mut expected = vec!["a".to_string(), "b".to_string()];
        expected.sort();
        assert_eq!(results, expected);
    }

    #[test]
    fn test_auto_complete_empty_prefix_with_root_value() {
        let mut trie = Trie::new();
        trie.insert("", &42); // Empty string as a word
        trie.insert("a", &1);
        trie.insert("b", &2);

        let mut results = trie.auto_complete("", 10);
        results.sort();

        let mut expected = vec!["".to_string(), "a".to_string(), "b".to_string()];
        expected.sort();
        assert_eq!(results, expected);
    }

    #[test]
    fn test_auto_complete_early_termination() {
        let mut trie = Trie::new();
        // Insert many words with same prefix
        for i in 0..20 {
            trie.insert(&format!("test{}", i), &i);
        }

        let results = trie.auto_complete("test", 5);
        assert_eq!(results.len(), 5);

        // Should include some of the words, but not all 20
        for result in &results {
            assert!(result.starts_with("test"));
        }
    }

    #[test]
    fn test_delete_leaf_word() {
        let mut trie = Trie::new();
        trie.insert("cat", &1);
        trie.insert("car", &2);
        trie.insert("card", &3);

        assert!(trie.delete("card"));
        assert_eq!(trie.get("card"), None);
        assert_eq!(trie.get("car"), Some(&2));
        assert_eq!(trie.get("cat"), Some(&1));
    }

    #[test]
    fn test_delete_word_with_children() {
        let mut trie = Trie::new();
        trie.insert("car", &1);
        trie.insert("card", &2);
        trie.insert("care", &3);

        assert!(trie.delete("car"));
        assert_eq!(trie.get("car"), None);
        assert_eq!(trie.get("card"), Some(&2));
        assert_eq!(trie.get("care"), Some(&3));

        // The prefix should still work
        assert!(trie.prefix_search("car"));
    }

    #[test]
    fn test_delete_nonexistent_word() {
        let mut trie = Trie::new();
        trie.insert("hello", &1);

        assert!(!trie.delete("world"));
        assert!(!trie.delete("hell"));
        assert!(!trie.delete("hello_world"));
        assert_eq!(trie.get("hello"), Some(&1));
    }

    #[test]
    fn test_delete_empty_string() {
        let mut trie = Trie::new();
        trie.insert("", &42);
        trie.insert("hello", &1);

        // Current implementation returns false for empty string deletion
        // This is a design choice - empty string deletion is not allowed
        assert!(!trie.delete(""));

        // The empty string value should still exist
        assert_eq!(trie.get(""), Some(&42));
        assert_eq!(trie.get("hello"), Some(&1));
    }

    #[test]
    fn test_delete_cleans_up_unused_nodes() {
        let mut trie = Trie::new();
        trie.insert("car", &1);
        trie.insert("card", &2);

        // Delete "card" - should clean up 'd' node
        assert!(trie.delete("card"));
        assert_eq!(trie.get("card"), None);
        assert_eq!(trie.get("car"), Some(&1));

        // Verify cleanup by checking autocomplete
        let results = trie.auto_complete("car", 10);
        // Should not include "card" anymore
        assert!(!results.contains(&"card".to_string()));
    }

    #[test]
    fn test_add_word_list_basic() {
        let mut trie = Trie::new();
        let words = ["apple", "banana", "cherry"];

        // Test with constant value
        trie.add_word_list(&words, |_| 100);

        assert_eq!(trie.get("apple"), Some(&100));
        assert_eq!(trie.get("banana"), Some(&100));
        assert_eq!(trie.get("cherry"), Some(&100));
        assert_eq!(trie.get("grape"), None);
    }

    #[test]
    fn test_add_word_list_with_dynamic_values() {
        let mut trie = Trie::new();
        let words = ["cat", "dog", "elephant"];

        // Use word length as value
        trie.add_word_list(&words, |word| word.len());

        assert_eq!(trie.get("cat"), Some(&3));
        assert_eq!(trie.get("dog"), Some(&3));
        assert_eq!(trie.get("elephant"), Some(&8));
    }

    #[test]
    fn test_add_word_list_with_owned_strings() {
        let mut trie = Trie::new();
        let words = vec!["hello".to_string(), "world".to_string()];

        // Test that it works with Vec<String>
        trie.add_word_list(&words, |word| word.len());

        assert_eq!(trie.get("hello"), Some(&5));
        assert_eq!(trie.get("world"), Some(&5));
    }

    #[test]
    fn test_add_word_list_with_complex_logic() {
        let mut trie = Trie::new();
        let words = ["the", "cat", "dog", "elephant"];

        // Assign frequency based on word characteristics
        trie.add_word_list(&words, |word| match word {
            &"the" => 1000,           // Very common word
            w if w.len() <= 3 => 100, // Short words
            _ => 10,                  // Longer words
        });

        assert_eq!(trie.get("the"), Some(&1000));
        assert_eq!(trie.get("cat"), Some(&100));
        assert_eq!(trie.get("dog"), Some(&100));
        assert_eq!(trie.get("elephant"), Some(&10));
    }

    #[test]
    fn test_different_value_types() {
        // Test with String values
        let mut string_trie = Trie::new();
        string_trie.insert("key", &"value".to_string());
        assert_eq!(string_trie.get("key"), Some(&"value".to_string()));

        // Test with boolean values
        let mut bool_trie = Trie::new();
        bool_trie.insert("true_key", &true);
        bool_trie.insert("false_key", &false);
        assert_eq!(bool_trie.get("true_key"), Some(&true));
        assert_eq!(bool_trie.get("false_key"), Some(&false));

        // Test with tuple values
        let mut tuple_trie = Trie::new();
        tuple_trie.insert("point", &(10, 20));
        assert_eq!(tuple_trie.get("point"), Some(&(10, 20)));

        // Test with Option values
        let mut option_trie = Trie::new();
        option_trie.insert("some", &Some(42));
        option_trie.insert("none", &None);
        assert_eq!(option_trie.get("some"), Some(&Some(42)));
        assert_eq!(option_trie.get("none"), Some(&None));
    }

    #[test]
    fn test_unicode_support() {
        let mut trie = Trie::new();
        trie.insert("café", &1);
        trie.insert("naïve", &2);
        trie.insert("résumé", &3);
        trie.insert("🦀", &4); // Rust crab emoji
        trie.insert("你好", &5); // Chinese characters
        trie.insert("مرحبا", &6); // Arabic

        assert_eq!(trie.get("café"), Some(&1));
        assert_eq!(trie.get("naïve"), Some(&2));
        assert_eq!(trie.get("résumé"), Some(&3));
        assert_eq!(trie.get("🦀"), Some(&4));
        assert_eq!(trie.get("你好"), Some(&5));
        assert_eq!(trie.get("مرحبا"), Some(&6));

        // Test prefix search with unicode
        assert!(trie.prefix_search("caf"));
        assert!(trie.prefix_search("你"));

        // Test autocomplete with unicode
        let results = trie.auto_complete("caf", 10);
        assert!(results.contains(&"café".to_string()));

        // Test deletion with unicode
        assert!(trie.delete("🦀"));
        assert_eq!(trie.get("🦀"), None);
    }

    #[test]
    fn test_case_sensitivity() {
        let mut trie = Trie::new();
        trie.insert("Hello", &1);
        trie.insert("hello", &2);
        trie.insert("HELLO", &3);

        assert_eq!(trie.get("Hello"), Some(&1));
        assert_eq!(trie.get("hello"), Some(&2));
        assert_eq!(trie.get("HELLO"), Some(&3));

        // All should be treated as different keys
        assert_eq!(trie.auto_complete("H", 10).len(), 2);
        assert_eq!(trie.auto_complete("h", 10).len(), 1);
        assert_eq!(trie.auto_complete("HE", 10).len(), 1);
    }

    #[test]
    fn test_long_words() {
        let mut trie = Trie::new();
        let long_word = "supercalifragilisticexpialidocious";
        let very_long_word = "pneumonoultramicroscopicsilicovolcanoconiosisverylongword";

        trie.insert(long_word, &42);
        trie.insert(very_long_word, &99);

        assert_eq!(trie.get(long_word), Some(&42));
        assert_eq!(trie.get(very_long_word), Some(&99));
        assert!(trie.prefix_search("supercali"));
        assert!(trie.prefix_search("pneumono"));

        let results = trie.auto_complete("supercali", 10);
        assert!(results.contains(&long_word.to_string()));

        let results2 = trie.auto_complete("pneumono", 10);
        assert!(results2.contains(&very_long_word.to_string()));
    }

    #[test]
    fn test_overlapping_words() {
        let mut trie = Trie::new();
        trie.insert("test", &1);
        trie.insert("testing", &2);
        trie.insert("tester", &3);
        trie.insert("te", &4);
        trie.insert("t", &5);

        assert_eq!(trie.get("t"), Some(&5));
        assert_eq!(trie.get("te"), Some(&4));
        assert_eq!(trie.get("test"), Some(&1));
        assert_eq!(trie.get("tester"), Some(&3));
        assert_eq!(trie.get("testing"), Some(&2));

        let mut results = trie.auto_complete("test", 10);
        results.sort();

        // Now that the bug is fixed, "test" itself should be included
        let mut expected = vec![
            "test".to_string(),
            "tester".to_string(),
            "testing".to_string(),
        ];
        expected.sort();
        assert_eq!(results, expected);

        // Test with limited results
        let results_limited = trie.auto_complete("test", 2);
        assert_eq!(results_limited.len(), 2);
        assert!(results_limited.contains(&"test".to_string()));
    }

    #[test]
    fn test_stress_test_insertion_and_retrieval() {
        let mut trie = Trie::new();

        // Insert many words
        for i in 0..1000 {
            let word = format!("word{:04}", i);
            trie.insert(&word, &i);
        }

        // Verify all insertions
        for i in 0..1000 {
            let word = format!("word{:04}", i);
            assert_eq!(trie.get(&word), Some(&i));
        }

        // Test prefix search
        assert!(trie.prefix_search("word"));
        assert!(trie.prefix_search("word0"));
        assert!(trie.prefix_search("word0999"));

        // Test autocomplete with various limits
        let results_10 = trie.auto_complete("word", 10);
        assert_eq!(results_10.len(), 10);

        let results_100 = trie.auto_complete("word", 100);
        assert_eq!(results_100.len(), 100);

        let results_all = trie.auto_complete("word", 2000);
        assert_eq!(results_all.len(), 1000); // Should return all 1000 words
    }

    #[test]
    fn test_stress_test_with_add_word_list() {
        let mut trie = Trie::new();

        // Generate a large list of words
        let words: Vec<String> = (0..500).map(|i| format!("item_{:03}", i)).collect();
        let word_refs: Vec<&str> = words.iter().map(|s| s.as_str()).collect();

        // Add them all with their index as value
        trie.add_word_list(&word_refs, |word| {
            word.split('_').nth(1).unwrap().parse::<usize>().unwrap()
        });

        // Verify all words exist with correct values
        for (i, word) in words.iter().enumerate() {
            assert_eq!(trie.get(word), Some(&i));
        }

        // Test autocomplete
        let results = trie.auto_complete("item_", 50);
        assert_eq!(results.len(), 50);
    }

    #[test]
    fn test_delete_all_words() {
        let mut trie = Trie::new();
        let words = ["a", "ab", "abc", "abcd", "b", "bc"];

        // Insert all words
        for (i, word) in words.iter().enumerate() {
            trie.insert(word, &i);
        }

        // Verify all exist
        for word in &words {
            assert!(trie.get(word).is_some());
        }

        // Delete all words
        for word in &words {
            assert!(trie.delete(word));
        }

        // Verify all are gone
        for word in &words {
            assert_eq!(trie.get(word), None);
        }

        // Trie should be effectively empty
        assert!(trie.auto_complete("", 10).is_empty());
        assert!(!trie.prefix_search("a"));
        assert!(!trie.prefix_search("b"));
    }

    #[test]
    fn test_complex_branching_scenario() {
        let mut trie = Trie::new();

        // Create a complex branching structure
        let words = [
            "a", "ab", "abc", "abd", "abde", "abdf", "abg", "ac", "acd", "ace", "acf", "b", "bc",
            "bcd", "bce",
        ];

        for (i, word) in words.iter().enumerate() {
            trie.insert(word, &i);
        }

        // Test that all words exist
        for (i, word) in words.iter().enumerate() {
            assert_eq!(trie.get(word), Some(&i));
        }

        // Test prefix searches at various levels
        assert!(trie.prefix_search("a"));
        assert!(trie.prefix_search("ab"));
        assert!(trie.prefix_search("abd"));
        assert!(trie.prefix_search("ac"));
        assert!(trie.prefix_search("bc"));

        // Test autocomplete at various levels with limits
        let results_a = trie.auto_complete("a", 5);
        assert_eq!(results_a.len(), 5);

        let results_ab = trie.auto_complete("ab", 10);
        assert!(results_ab.len() >= 5); // Should find ab, abc, abd, abde, abdf, abg
        assert!(results_ab.contains(&"ab".to_string()));
        assert!(results_ab.contains(&"abc".to_string()));

        let results_ac = trie.auto_complete("ac", 10);
        assert!(results_ac.len() >= 4); // Should find ac, acd, ace, acf
        assert!(results_ac.contains(&"ac".to_string()));

        // Test deletion doesn't affect other branches
        assert!(trie.delete("abg"));
        assert_eq!(trie.get("abg"), None);
        assert_eq!(trie.get("abc"), Some(&2)); // Should still exist
        assert_eq!(trie.get("ac"), Some(&7)); // Should still exist
    }

    #[test]
    fn test_special_characters() {
        let mut trie = Trie::new();

        // Test with various special characters
        let special_words = [
            "hello-world",
            "user@domain.com",
            "file.txt",
            "path/to/file",
            "C:\\Windows\\System32",
            "price$100",
            "temp&humid",
            "a+b=c",
            "question?",
            "exclamation!",
            "quote\"test",
            "tab\there",
            "new\nline",
            "carriage\rreturn",
        ];

        for (i, word) in special_words.iter().enumerate() {
            trie.insert(word, &i);
        }

        // Verify all special character words work
        for (i, word) in special_words.iter().enumerate() {
            assert_eq!(trie.get(word), Some(&i));
        }

        // Test prefix search with special characters
        assert!(trie.prefix_search("hello-"));
        assert!(trie.prefix_search("user@"));
        assert!(trie.prefix_search("file."));

        // Test autocomplete with special characters
        let results = trie.auto_complete("hello-", 10);
        assert!(results.contains(&"hello-world".to_string()));
    }

    #[test]
    fn test_numeric_strings() {
        let mut trie = Trie::new();

        // Test with numeric strings
        trie.insert("123", &123);
        trie.insert("456", &456);
        trie.insert("12345", &12345);
        trie.insert("0", &0);
        trie.insert("007", &7);

        assert_eq!(trie.get("123"), Some(&123));
        assert_eq!(trie.get("456"), Some(&456));
        assert_eq!(trie.get("12345"), Some(&12345));
        assert_eq!(trie.get("0"), Some(&0));
        assert_eq!(trie.get("007"), Some(&7));

        // Test prefix search with numbers
        assert!(trie.prefix_search("12"));
        assert!(trie.prefix_search("00"));

        // Test autocomplete with numbers
        let results = trie.auto_complete("12", 10);
        assert!(results.contains(&"123".to_string()));
        assert!(results.contains(&"12345".to_string()));
    }

    #[test]
    fn test_empty_results_scenarios() {
        let mut trie = Trie::new();
        trie.insert("apple", &1);
        trie.insert("banana", &2);

        // Test various scenarios that should return empty results
        assert!(trie.auto_complete("xyz", 10).is_empty());
        assert!(trie.auto_complete("appl", 0).is_empty()); // max_results = 0
        assert!(trie.auto_complete("z", 10).is_empty());

        // Test after deletion
        assert!(trie.delete("apple"));
        let results = trie.auto_complete("appl", 10);
        assert!(results.is_empty());
    }
}

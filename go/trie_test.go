package trie

import (
	"fmt"
	"reflect"
	"sort"
	"testing"
)

// Constructor for Trie - you'll need this
func NewTrie[TValue any]() *Trie[TValue] {
	return &Trie[TValue]{
		root: NewTrieNode[TValue](),
	}
}

// Basic functionality tests
func TestNewTrie(t *testing.T) {
	trie := NewTrie[string]()

	if trie == nil {
		t.Fatal("NewTrie should not return nil")
	}

	if trie.root == nil {
		t.Fatal("Trie should have a root node")
	}

	if trie.root.hasChildren() {
		t.Error("New trie root should not have children")
	}

	if trie.root.isEndOfWord() {
		t.Error("New trie root should not be end of word")
	}
}

func TestInsertAndGet(t *testing.T) {
	trie := NewTrie[int]()

	// Test inserting and getting a single word
	key := "apple"
	value := 42

	trie.Insert(&key, value)

	retrievedValue, exists := trie.Get(&key)
	if !exists {
		t.Error("Key should exist after insertion")
	}

	if retrievedValue == nil {
		t.Fatal("Value should not be nil for existing key")
	}

	if *retrievedValue != value {
		t.Errorf("Expected value %d, got %d", value, *retrievedValue)
	}
}

func TestGetNonExistent(t *testing.T) {
	trie := NewTrie[string]()

	key := "nonexistent"
	value, exists := trie.Get(&key)

	if exists {
		t.Error("Non-existent key should not exist")
	}

	if value != nil {
		t.Error("Value should be nil for non-existent key")
	}
}

func TestInsertMultipleWords(t *testing.T) {
	trie := NewTrie[string]()

	words := map[string]string{
		"cat":      "feline",
		"car":      "vehicle",
		"card":     "paper",
		"care":     "concern",
		"careful":  "cautious",
		"careless": "reckless",
	}

	// Insert all words
	for key, value := range words {
		trie.Insert(&key, value)
	}

	// Verify all words exist
	for key, expectedValue := range words {
		value, exists := trie.Get(&key)
		if !exists {
			t.Errorf("Key %s should exist", key)
			continue
		}

		if value == nil {
			t.Errorf("Value for key %s should not be nil", key)
			continue
		}

		if *value != expectedValue {
			t.Errorf("For key %s, expected %s, got %s", key, expectedValue, *value)
		}
	}
}

func TestInsertUpdateExisting(t *testing.T) {
	trie := NewTrie[int]()

	key := "test"
	originalValue := 1
	newValue := 2

	// Insert original value
	trie.Insert(&key, originalValue)

	value, _ := trie.Get(&key)
	if *value != originalValue {
		t.Errorf("Expected original value %d", originalValue)
	}

	// Update with new value
	trie.Insert(&key, newValue)

	value, exists := trie.Get(&key)
	if !exists {
		t.Error("Key should still exist after update")
	}

	if *value != newValue {
		t.Errorf("Expected updated value %d, got %d", newValue, *value)
	}
}

// Delete tests
func TestDelete(t *testing.T) {
	trie := NewTrie[string]()

	// Insert some words
	words := []string{"cat", "cats", "car"}
	for _, word := range words {
		trie.Insert(&word, word+"_value")
	}

	// Delete "cat"
	key := "cat"
	deleted := trie.Delete(&key)
	if !deleted {
		t.Error("Delete should return true for existing key")
	}

	// Verify "cat" is gone
	_, exists := trie.Get(&key)
	if exists {
		t.Error("Deleted key should not exist")
	}

	// Verify "cats" and "car" still exist
	for _, word := range []string{"cats", "car"} {
		_, exists := trie.Get(&word)
		if !exists {
			t.Errorf("Key %s should still exist after deleting %s", word, key)
		}
	}
}

func TestDeleteNonExistent(t *testing.T) {
	trie := NewTrie[string]()

	key := "nonexistent"
	deleted := trie.Delete(&key)

	if deleted {
		t.Error("Delete should return false for non-existent key")
	}
}

func TestDeleteEmptyKey(t *testing.T) {
	trie := NewTrie[string]()

	emptyKey := ""
	deleted := trie.Delete(&emptyKey)

	if deleted {
		t.Error("Delete should return false for empty key")
	}

	// Test nil key
	deleted = trie.Delete(nil)
	if deleted {
		t.Error("Delete should return false for nil key")
	}
}

func TestDeleteComplexScenario(t *testing.T) {
	trie := NewTrie[int]()

	// Insert words where one is prefix of another
	words := map[string]int{
		"app":         1,
		"apple":       2,
		"apply":       3,
		"application": 4,
	}

	for key, value := range words {
		trie.Insert(&key, value)
	}

	// Delete "app" - should not affect others
	key := "app"
	trie.Delete(&key)

	// Verify "app" is gone
	_, exists := trie.Get(&key)
	if exists {
		t.Error("'app' should be deleted")
	}

	// Verify others still exist
	for _, word := range []string{"apple", "apply", "application"} {
		_, exists := trie.Get(&word)
		if !exists {
			t.Errorf("'%s' should still exist", word)
		}
	}

	// Delete "apple" - should not affect "apply" or "application"
	key = "apple"
	trie.Delete(&key)

	_, exists = trie.Get(&key)
	if exists {
		t.Error("'apple' should be deleted")
	}

	for _, word := range []string{"apply", "application"} {
		_, exists := trie.Get(&word)
		if !exists {
			t.Errorf("'%s' should still exist after deleting apple", word)
		}
	}
}

func TestDeletePruning(t *testing.T) {
	trie := NewTrie[string]()

	// Insert a word that creates a long chain
	word := "testing"
	trie.Insert(&word, "value")

	// Delete it - should prune the entire unused branch
	deleted := trie.Delete(&word)
	if !deleted {
		t.Error("Delete should succeed")
	}

	// Root should have no children after pruning
	if trie.root.hasChildren() {
		t.Error("Root should have no children after pruning unused branch")
	}
}

// Prefix search tests
func TestPrefixSearch(t *testing.T) {
	trie := NewTrie[string]()

	words := []string{"cat", "cats", "car", "card", "care", "careful", "dog", "doggy"}
	for _, word := range words {
		trie.Insert(&word, word+"_value")
	}

	testCases := []struct {
		prefix   string
		expected []string
	}{
		{"ca", []string{"cat", "cats", "car", "card", "care", "careful"}},
		{"car", []string{"car", "card", "care", "careful"}},
		{"care", []string{"care", "careful"}},
		{"careful", []string{"careful"}},
		{"dog", []string{"dog", "doggy"}},
		{"do", []string{"dog", "doggy"}},
		{"xyz", []string{}}, // non-existent prefix
	}

	for _, tc := range testCases {
		results := trie.PrefixSearch(&tc.prefix)

		// Sort both slices for comparison
		sort.Strings(results)
		sort.Strings(tc.expected)

		if !reflect.DeepEqual(results, tc.expected) {
			t.Errorf("PrefixSearch(%s): expected %v, got %v", tc.prefix, tc.expected, results)
		}
	}
}

func TestPrefixSearchEmpty(t *testing.T) {
	trie := NewTrie[string]()

	prefix := "anything"
	results := trie.PrefixSearch(&prefix)

	if len(results) != 0 {
		t.Errorf("Empty trie should return empty results, got %v", results)
	}
}

func TestPrefixSearchEmptyPrefix(t *testing.T) {
	trie := NewTrie[string]()

	words := []string{"cat", "dog", "bird"}
	for _, word := range words {
		trie.Insert(&word, word+"_value")
	}

	prefix := ""
	results := trie.PrefixSearch(&prefix)

	// Empty prefix should return all words
	sort.Strings(results)
	sort.Strings(words)

	if !reflect.DeepEqual(results, words) {
		t.Errorf("Empty prefix should return all words: expected %v, got %v", words, results)
	}
}

func TestPrefixSearchWithPrefixAsWord(t *testing.T) {
	trie := NewTrie[int]()

	// Insert words where some are prefixes of others
	trie.Insert(stringPtr("car"), 1)
	trie.Insert(stringPtr("care"), 2)
	trie.Insert(stringPtr("careful"), 3)
	trie.Insert(stringPtr("careless"), 4)

	prefix := "care"
	results := trie.PrefixSearch(&prefix)
	expected := []string{"care", "careful", "careless"}

	sort.Strings(results)
	sort.Strings(expected)

	if !reflect.DeepEqual(results, expected) {
		t.Errorf("PrefixSearch with prefix as word: expected %v, got %v", expected, results)
	}
}

// AddWordList tests
func TestAddWordList(t *testing.T) {
	trie := NewTrie[int]()

	words := []string{"apple", "banana", "cherry"}
	valueGenerator := func(word string) int {
		return len(word) // Use word length as value
	}

	trie.AddWordList(&words, valueGenerator)

	// Verify all words were added with correct values
	for _, word := range words {
		value, exists := trie.Get(&word)
		if !exists {
			t.Errorf("Word %s should exist after AddWordList", word)
			continue
		}

		expectedValue := len(word)
		if value == nil || *value != expectedValue {
			t.Errorf("Word %s: expected value %d, got %v", word, expectedValue, value)
		}
	}
}

func TestAddWordListWithSameValue(t *testing.T) {
	trie := NewTrie[string]()

	words := []string{"red", "green", "blue"}
	constantValue := "color"

	trie.AddWordList(&words, func(word string) string {
		return constantValue
	})

	// All words should have the same value
	for _, word := range words {
		value, exists := trie.Get(&word)
		if !exists {
			t.Errorf("Word %s should exist", word)
			continue
		}

		if value == nil || *value != constantValue {
			t.Errorf("Word %s: expected value %s, got %v", word, constantValue, value)
		}
	}
}

// Unicode and special character tests
func TestUnicodeSupportTrie(t *testing.T) {
	trie := NewTrie[string]()

	unicodeWords := map[string]string{
		"café":    "coffee",
		"naïve":   "innocent",
		"résumé":  "cv",
		"🌟":       "star",
		"你好":      "hello",
		"مرحبا":   "welcome",
		"Ñoño":    "cute",
		"München": "munich",
	}

	// Insert unicode words
	for key, value := range unicodeWords {
		trie.Insert(&key, value)
	}

	// Verify they can be retrieved
	for key, expectedValue := range unicodeWords {
		value, exists := trie.Get(&key)
		if !exists {
			t.Errorf("Unicode word %s should exist", key)
			continue
		}

		if value == nil || *value != expectedValue {
			t.Errorf("Unicode word %s: expected %s, got %v", key, expectedValue, value)
		}
	}

	// Test prefix search with unicode
	prefix := "caf"
	results := trie.PrefixSearch(&prefix)
	expected := []string{"café"}

	if !reflect.DeepEqual(results, expected) {
		t.Errorf("Unicode prefix search: expected %v, got %v", expected, results)
	}
}

func TestEmojiSupport(t *testing.T) {
	trie := NewTrie[string]()

	emojis := map[string]string{
		"🚀":  "rocket",
		"🌟":  "star",
		"🎉":  "party",
		"❤️": "heart",
		"👍":  "thumbs_up",
	}

	for emoji, meaning := range emojis {
		trie.Insert(&emoji, meaning)
	}

	for emoji, expectedMeaning := range emojis {
		value, exists := trie.Get(&emoji)
		if !exists {
			t.Errorf("Emoji %s should exist", emoji)
			continue
		}

		if value == nil || *value != expectedMeaning {
			t.Errorf("Emoji %s: expected %s, got %v", emoji, expectedMeaning, value)
		}
	}
}

// Edge cases and stress tests
func TestEdgeCases(t *testing.T) {
	trie := NewTrie[string]()

	// Test single character words
	singleChar := "a"
	trie.Insert(&singleChar, "letter_a")

	value, exists := trie.Get(&singleChar)
	if !exists || value == nil || *value != "letter_a" {
		t.Error("Single character word should work")
	}

	// Test very long word (1000 'a' characters)
	longWordRunes := make([]rune, 1000)
	for i := range longWordRunes {
		longWordRunes[i] = 'a'
	}
	longWord := string(longWordRunes)

	trie.Insert(&longWord, "very_long")

	value, exists = trie.Get(&longWord)
	if !exists || value == nil || *value != "very_long" {
		t.Error("Very long word should work")
	}
}

func TestEmptyStringHandling(t *testing.T) {
	trie := NewTrie[string]()

	// Insert empty string
	emptyKey := ""
	trie.Insert(&emptyKey, "empty_value")

	// Should be able to retrieve it
	value, exists := trie.Get(&emptyKey)
	if !exists {
		t.Error("Empty string should be insertable and retrievable")
	}

	if value == nil || *value != "empty_value" {
		t.Error("Empty string should have correct value")
	}

	// Test prefix search with empty string
	results := trie.PrefixSearch(&emptyKey)
	if len(results) == 0 {
		t.Error("Prefix search with empty string should return something if trie has words")
	}

	// Should include the empty string itself if it exists
	found := false
	for _, result := range results {
		if result == "" {
			found = true
			break
		}
	}
	if !found {
		t.Error("Prefix search should include empty string if it exists as a word")
	}
}

func TestLargeDataset(t *testing.T) {
	trie := NewTrie[int]()

	// Generate a large number of words
	const numWords = 1000
	words := make([]string, numWords)
	for i := 0; i < numWords; i++ {
		words[i] = fmt.Sprintf("word_%d", i)
		trie.Insert(&words[i], i)
	}

	// Verify all words exist
	for i, word := range words {
		value, exists := trie.Get(&word)
		if !exists {
			t.Errorf("Word %s (index %d) should exist", word, i)
			continue
		}

		if value == nil || *value != i {
			t.Errorf("Word %s: expected value %d, got %v", word, i, value)
		}
	}

	// Test prefix search on large dataset
	prefix := "word_1"
	results := trie.PrefixSearch(&prefix)

	// Should find words like "word_1", "word_10", "word_11", ..., "word_199"
	if len(results) == 0 {
		t.Error("Should find some words with prefix 'word_1'")
	}

	// Verify all results actually start with the prefix
	for _, result := range results {
		if len(result) < len(prefix) || result[:len(prefix)] != prefix {
			t.Errorf("Result %s should start with prefix %s", result, prefix)
		}
	}
}

func TestConcurrentOperations(t *testing.T) {
	// Note: This test assumes your implementation is NOT thread-safe
	// If you want thread safety, you'd need to add mutexes
	trie := NewTrie[int]()

	// Sequential operations should work fine
	words := []string{"test1", "test2", "test3"}
	for i, word := range words {
		trie.Insert(&word, i)
	}

	for i, word := range words {
		value, exists := trie.Get(&word)
		if !exists || value == nil || *value != i {
			t.Errorf("Sequential operation failed for word %s", word)
		}
	}
}

// Performance benchmark (not a test, but useful)
func BenchmarkInsert(b *testing.B) {
	trie := NewTrie[int]()

	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		word := fmt.Sprintf("word_%d", i)
		trie.Insert(&word, i)
	}
}

func BenchmarkGet(b *testing.B) {
	trie := NewTrie[int]()

	// Pre-populate with some data
	for i := 0; i < 1000; i++ {
		word := fmt.Sprintf("word_%d", i)
		trie.Insert(&word, i)
	}

	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		word := fmt.Sprintf("word_%d", i%1000)
		trie.Get(&word)
	}
}

func BenchmarkPrefixSearch(b *testing.B) {
	trie := NewTrie[int]()

	// Pre-populate with some data
	for i := 0; i < 1000; i++ {
		word := fmt.Sprintf("prefix_%d", i)
		trie.Insert(&word, i)
	}

	prefix := "prefix_"
	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		trie.PrefixSearch(&prefix)
	}
}

// Helper function
func stringPtr(s string) *string {
	return &s
}

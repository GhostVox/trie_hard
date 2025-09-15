package trie

import (
	"errors"
)

type TrieError int

var TrieErrorChildDoesNotExist = errors.New("No child available")

type Trie[TValue any] struct {
	// The root node of the Trie. It does not hold any character itself.
	root *TrieNode[TValue]
}

// Insert adds a key-value pair into the Trie.
// The value marks the end of a string and can be retrieved later.
// If the key already exists, its value is updated.
//
// Example:
//
//	trie := NewTrie()
//	trie.Insert("apple", 1)
//	value, exists := trie.Get("apple")
//	// value == 1, exists == true
func (self *Trie[TValue]) Insert(key *string, value TValue) {
	currentNode := self.root
	for _, char := range *key {
		if !currentNode.containsChild(char) {
			currentNode.addChild(char)
		}
		child, _ := currentNode.getChildMut(char)
		currentNode = child
	}
	currentNode.setValue(value)
}

// Get searches for a key and returns its value if it exists.
// It returns the value and a boolean indicating whether the key was found.
//
// Example:
//
//	trie := NewTrie()
//	trie.Insert("apple", 1)
//	value, exists := trie.Get("apple") // value == 1, exists == true
func (self *Trie[TValue]) Get(key *string) (*TValue, bool) {
	currentNode := self.root
	for _, char := range *key {
		if child, err := currentNode.getChildMut(char); err == nil {
			currentNode = child
		} else {
			return nil, false
		}
	}

	return currentNode.getValue()
}

// Delete removes a key and its associated value from the Trie.
// It returns true if the key was found and deleted, false otherwise.
//
// Example:
//
//	trie := NewTrie()
//	trie.Insert("apple", 1)
//	deleted := trie.Delete("apple") // deleted == true
//	value, exists := trie.Get("apple") // exists == false
func (self *Trie[TValue]) Delete(key *string) bool {
	if key == nil || *key == "" {
		// Empty key is not allowed
		return false
	}
	deleted, _ := self.deleteRecursive(self.root, *key)
	return deleted
}

func (self *Trie[TValue]) deleteRecursive(currentNode *TrieNode[TValue], key string) (bool, bool) {
	if key == "" {

		// We have reached the node corresponding to the key.
		if currentNode.isEndOfWord() {
			currentNode.clearValue()

			// Return true if this node has no children, so the parent can remove it.
			return true, !currentNode.hasChildren()
		}
		// Key doesn't actually exist as a word in the trie.
		return false, false
	}

	runes := []rune(key)
	c := runes[0]
	remaining := string(runes[1:])
	if child, err := currentNode.getChildMut(c); err == nil {
		deleted, shouldDeleteChild := self.deleteRecursive(child, remaining)
		if shouldDeleteChild {
			currentNode.removeChild(c)
		}

		shouldPruneThisNode := deleted && !currentNode.isEndOfWord() && !currentNode.hasChildren()
		return deleted, shouldPruneThisNode
	} else {
		// The path for the key does not exist.
		return false, false
	}
}

func (self *Trie[TValue]) PrefixSearch(prefix *string) bool {
	currentNode := self.root // Use local variable!
	for _, char := range *prefix {
		if child, err := currentNode.getChildMut(char); err == nil {
			currentNode = child // Modify local variable, not self.root
		} else {
			return false
		}
	}
	return true // If we got here, prefix exists
}

// Preforms a prefix serach on the trie and returns all words that start with the given prefix.
// If no words match the prefix, an empty list is returned.
//
// Example:
//
//	trie := NewTrie()
//	trie.Insert("apple", 1)
//	trie.Insert("app", 2)
//	words := trie.AutoComplete("app") // words == ["app", "apple"]
func (self *Trie[TValue]) AutoComplete(prefix *string) []string {
	results := []string{}
	currentNode := self.root
	for _, char := range *prefix {
		if child, err := currentNode.getChildMut(char); err == nil {
			currentNode = child
		} else {
			// Prefix not found, return empty list
			return results
		}
	}

	self.collectWordsRecursive(currentNode, *prefix, &results)
	return results
}

// Helper function to recursively collect words from the trie.
// Adds prefix to results if prefix is  a complete word, then continues to explore children to find
// extensions of the prefix.
func (self *Trie[TValue]) collectWordsRecursive(node *TrieNode[TValue], currPrefix string, results *[]string) {
	// If the current node marks the end of a word, add the current prefix to results.
	if node.isEndOfWord() {
		*results = append(*results, currPrefix)
	}
	// Continue with all children to find extensions.
	for char, child := range node.children {
		newPrefix := currPrefix + string(char)
		self.collectWordsRecursive(child, newPrefix, results)
	}
}

// Takes a list of words and a function that generates a value for each word to store at the end of
// the chain.
func (self *Trie[TValue]) AddWordList(words *[]string, valueGenerator func(string) TValue) {
	for _, word := range *words {
		self.Insert(&word, valueGenerator(word))
	}
}

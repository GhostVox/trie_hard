package trie

import (
	"testing"
)

func TestNewTrieNode(t *testing.T) {
	node := NewTrieNode[string]()

	if node == nil {
		t.Fatal("NewTrieNode should not return nil")
	}

	if node.value != nil {
		t.Error("New node should have nil value")
	}

	if node.children == nil {
		t.Error("New node should have initialized children map")
	}

	if len(node.children) != 0 {
		t.Error("New node should have empty children map")
	}

	if node.hasChildren() {
		t.Error("New node should not have children")
	}

	if node.isEndOfWord() {
		t.Error("New node should not be end of word")
	}
}

func TestSetValue(t *testing.T) {
	node := NewTrieNode[string]()

	// Initially not end of word
	if node.isEndOfWord() {
		t.Error("Node should not be end of word initially")
	}

	// Set a value
	testValue := "test_value"
	node.setValue(testValue)

	if !node.isEndOfWord() {
		t.Error("Node should be end of word after setting value")
	}

	if node.value == nil {
		t.Fatal("Value should not be nil after setting")
	}

	if *node.value != testValue {
		t.Errorf("Expected value %s, got %s", testValue, *node.value)
	}
}

func TestClearValue(t *testing.T) {
	node := NewTrieNode[int]()

	// Set a value first
	testValue := 42
	node.setValue(testValue)

	if !node.isEndOfWord() {
		t.Error("Node should be end of word after setting value")
	}

	// Clear the value
	node.clearValue()

	if node.isEndOfWord() {
		t.Error("Node should not be end of word after clearing value")
	}

	if node.value != nil {
		t.Error("Value should be nil after clearing")
	}
}

func TestAddChild(t *testing.T) {
	node := NewTrieNode[string]()

	// Initially no children
	if node.hasChildren() {
		t.Error("Node should not have children initially")
	}

	// Add a child
	char := 'a'
	child := node.addChild(char)

	if child == nil {
		t.Fatal("addChild should return a valid node")
	}

	if !node.hasChildren() {
		t.Error("Node should have children after adding one")
	}

	if !node.containsChild(char) {
		t.Error("Node should contain the added child")
	}

	// Add the same child again - should return existing
	child2 := node.addChild(char)

	if child != child2 {
		t.Error("Adding same child twice should return the same node")
	}

	if len(node.children) != 1 {
		t.Errorf("Expected 1 child, got %d", len(node.children))
	}
}

func TestContainsChild(t *testing.T) {
	node := NewTrieNode[string]()

	char := 'b'

	// Initially should not contain any children
	if node.containsChild(char) {
		t.Error("Node should not contain child initially")
	}

	// Add child
	node.addChild(char)

	// Now should contain the child
	if !node.containsChild(char) {
		t.Error("Node should contain child after adding")
	}

	// Should not contain different character
	if node.containsChild('z') {
		t.Error("Node should not contain non-existent child")
	}
}

func TestGetChildMut(t *testing.T) {
	node := NewTrieNode[string]()

	char := 'c'

	// Try to get non-existent child
	child, err := node.getChildMut(char)

	if err != TrieErrorChildDoesNotExist {
		t.Errorf("Expected TrieErrorChildDoesNotExist, got %v", err)
	}

	if child != nil {
		t.Error("Should return nil for non-existent child")
	}

	// Add child and then get it
	originalChild := node.addChild(char)
	retrievedChild, err := node.getChildMut(char)
	if err != nil {
		t.Errorf("Expected no error, got %v", err)
	}

	if retrievedChild != originalChild {
		t.Error("Retrieved child should be the same as the original")
	}
}

func TestRemoveChild(t *testing.T) {
	node := NewTrieNode[string]()

	char := 'd'

	// Remove non-existent child (should be no-op)
	node.removeChild(char)

	if node.hasChildren() {
		t.Error("Node should not have children after removing non-existent child")
	}

	// Add child, then remove it
	node.addChild(char)

	if !node.hasChildren() {
		t.Error("Node should have children after adding")
	}

	if !node.containsChild(char) {
		t.Error("Node should contain child after adding")
	}

	// Remove the child
	node.removeChild(char)

	if node.hasChildren() {
		t.Error("Node should not have children after removing")
	}

	if node.containsChild(char) {
		t.Error("Node should not contain child after removing")
	}
}

func TestMultipleChildren(t *testing.T) {
	node := NewTrieNode[int]()

	chars := []rune{'a', 'b', 'c', '🌟'} // Include Unicode

	// Add multiple children
	for _, char := range chars {
		child := node.addChild(char)
		if child == nil {
			t.Fatalf("Failed to add child for character %c", char)
		}
	}

	// Check all children exist
	for _, char := range chars {
		if !node.containsChild(char) {
			t.Errorf("Node should contain child %c", char)
		}

		child, err := node.getChildMut(char)
		if err != nil {
			t.Errorf("Failed to get child %c: %v", char, err)
		}
		if child == nil {
			t.Errorf("Child %c should not be nil", char)
		}
	}

	if len(node.children) != len(chars) {
		t.Errorf("Expected %d children, got %d", len(chars), len(node.children))
	}

	// Remove one child
	node.removeChild(chars[0])

	if node.containsChild(chars[0]) {
		t.Errorf("Node should not contain removed child %c", chars[0])
	}

	if len(node.children) != len(chars)-1 {
		t.Errorf("Expected %d children after removal, got %d", len(chars)-1, len(node.children))
	}

	// Other children should still exist
	for i := 1; i < len(chars); i++ {
		if !node.containsChild(chars[i]) {
			t.Errorf("Node should still contain child %c", chars[i])
		}
	}
}

func TestUnicodeSupport(t *testing.T) {
	node := NewTrieNode[string]()

	unicodeChars := []rune{'🌟', '你', 'ñ', 'ü', '🚀'}

	for _, char := range unicodeChars {
		child := node.addChild(char)
		if child == nil {
			t.Fatalf("Failed to add Unicode child %c", char)
		}

		if !node.containsChild(char) {
			t.Errorf("Node should contain Unicode child %c", char)
		}

		retrievedChild, err := node.getChildMut(char)
		if err != nil {
			t.Errorf("Failed to get Unicode child %c", char)
		}
		if retrievedChild != child {
			t.Errorf("Retrieved Unicode child should match original for %c", char)
		}
	}
}

func TestNodeStateTransitions(t *testing.T) {
	node := NewTrieNode[string]()

	// Start state: no children, no value
	if node.hasChildren() || node.isEndOfWord() {
		t.Error("Initial state should be: no children, not end of word")
	}

	// Add child: has children, no value
	node.addChild('a')
	if !node.hasChildren() || node.isEndOfWord() {
		t.Error("After adding child: should have children, not end of word")
	}

	// Set value: has children, has value
	node.setValue("test")
	if !node.hasChildren() || !node.isEndOfWord() {
		t.Error("After setting value: should have children and be end of word")
	}

	// Clear value: has children, no value
	node.clearValue()
	if !node.hasChildren() || node.isEndOfWord() {
		t.Error("After clearing value: should have children, not end of word")
	}

	// Remove child: no children, no value
	node.removeChild('a')
	if node.hasChildren() || node.isEndOfWord() {
		t.Error("After removing child: should not have children, not end of word")
	}
}

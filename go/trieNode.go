package trie

// Represents a node within a Trie. It is generic over the value it stores.
type TrieNode[TValue any] struct {
	// Note: The `character` for this node is the *key* in the parent's HashMap.
	// We don't need to store it inside the node itself.
	children map[rune]*TrieNode[TValue]

	// The value associated with the full word ending at this node.
	// Using value is key, as intermediate nodes won't have a value.
	value *TValue
}

// Creates a new, "empty" TrieNode without an intitial value.
// This is the correct constructor for a node that isn't the end of a word yet.
func NewTrieNode[TValue any]() *TrieNode[TValue] {
	return &TrieNode[TValue]{
		children: make(map[rune]*TrieNode[TValue]),
		value:    nil,
	}
}

// Checks if the node has any children. A node with no children is a "leaf".
func (self *TrieNode[TValue]) hasChildren() bool {
	return len(self.children) > 0
}

// checks if the node has the character as a child in the map. this allows the trie to ask questions
// about the nodes children with out getting a mutable reference to the child.
func (self *TrieNode[TValue]) containsChild(char rune) bool {
	_, exists := self.children[char]
	return exists
}

// Gets an reference to a child node correspondiding to the character.
func (self *TrieNode[TValue]) getChildMut(character rune) (*TrieNode[TValue], error) {
	if _, exists := self.children[character]; !exists {
		return nil, TrieErrorChildDoesNotExist
	}
	return self.children[character], nil
}

// Creates a new child node for the given character and returns a reference to that node.
// If the child already exists, it simply returns a reference to the existing child.
func (self *TrieNode[TValue]) addChild(character rune) *TrieNode[TValue] {
	if _, exists := self.children[character]; !exists {
		self.children[character] = NewTrieNode[TValue]()
		return self.children[character]
	}
	return self.children[character]
}

// Removes a child node. If the child does not exist, this is a no-op.
func (self *TrieNode[TValue]) removeChild(character rune) {
	delete(self.children, character)
}

// Checks if this node represents the end of a complete word.
func (self *TrieNode[TValue]) isEndOfWord() bool {
	return self.value != nil
}

// Sets the value for this node, marking it as the end of a word.
func (self *TrieNode[TValue]) setValue(value TValue) {
	self.value = &value
}

// Gets the value associated with this node, if it is the end of a word.
func (self *TrieNode[TValue]) getValue() (*TValue, bool) {
	if self.value != nil {
		return self.value, true
	}
	return nil, false
}

// Clears the value on the node, if it is the end of a word.
func (self *TrieNode[TValue]) clearValue() {
	self.value = nil
}

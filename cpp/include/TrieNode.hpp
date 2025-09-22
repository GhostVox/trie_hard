// Header-only implementation for TrieNode.
// Navigation uses raw pointers while ownership stays inside unique_ptr children
// map.
#pragma once
#include <memory>
#include <optional>
#include <unordered_map>

/**
 * @brief Class representing a node in a Trie (prefix tree).
 *
 * @tparam T Generic type for values stored in the Trie.
 */
template <typename T> class TrieNode {
public:
  TrieNode() = default;
  ~TrieNode() = default;

  bool hasChildren() const { return !children.empty(); }

  /**
   * @brief Gets the child node corresponding to the given key.
   *
   * @param[char] key Key to look for.
   */
  TrieNode<T> *getChild(char key) const {
    auto it = children.find(key);
    return it == children.end() ? nullptr : it->second.get();
  }

  /**
   * @brief Adds a child node for the given key if it doesn't already exist.
   *
   * @param[char] key Key to add.
   */
  TrieNode<T> *addChild(char key) {
    auto it = children.find(key);
    if (it == children.end()) {
      it = children.emplace(key, std::make_unique<TrieNode<T>>()).first;
    }
    return it->second.get();
  }

  /**
   * @brief Deletes the child node corresponding to the given key.
   *
   * @param[char] key Key to remove.
   */
  void removeChild(char key) { children.erase(key); }

  /**
   * @brief Checks if the node marks the end of a valid key (i.e., has an
   * associated value).
   *
   * @return True if the node has a value, false otherwise.
   */
  bool isEnd() const { return value.has_value(); }

  /**
   * @brief Gets the value associated with the node, if any.
   *
   */
  const std::optional<T> &getValue() const { return value; }

  /**
   * @brief Clears the value associated with the node and returns the old value,
   * if any.
   *
   */
  std::optional<T> clearValue() {
    auto old = value;
    value.reset();
    return old;
  }

  /**
   * @brief Sets the value associated with the node.
   *
   * @param[const T] v Value to set.
   */
  void setValue(const T &v) { value = v; }

  /**
   * @brief Gets the map of child nodes.
   *
   */
  const std::unordered_map<char, std::unique_ptr<TrieNode<T>>> &
  getChildren() const {
    return children;
  }

private:
  std::unordered_map<char, std::unique_ptr<TrieNode<T>>> children;
  std::optional<T> value;
};

// Header-only implementation for TrieNode.
// Navigation uses raw pointers while ownership stays inside unique_ptr children map.
#pragma once
#include <memory>
#include <optional>
#include <unordered_map>
#include <vector>

template <typename T>
class TrieNode {
  public:
    TrieNode() = default;
    ~TrieNode() = default;

    bool hasChildren() const { return !children.empty(); }

    TrieNode<T> *getChild(char key) const {
        auto it = children.find(key);
        return it == children.end() ? nullptr : it->second.get();
    }

    TrieNode<T> *addChild(char key) {
        auto it = children.find(key);
        if (it == children.end()) {
            it = children.emplace(key, std::make_unique<TrieNode<T>>()).first;
        }
        return it->second.get();
    }

    void removeChild(char key) { children.erase(key); }

    bool isEnd() const { return value.has_value(); }

    const std::optional<T> &getValue() const { return value; }

    std::optional<T> clearValue() {
        auto old = value;
        value.reset();
        return old;
    }

    void setValue(const T &v) { value = v; }

    // Expose read-only access to children for traversal (autocomplete, etc.).
    const std::unordered_map<char, std::unique_ptr<TrieNode<T>>> &getChildren() const { return children; }

  private:
    std::unordered_map<char, std::unique_ptr<TrieNode<T>>> children;
    std::optional<T> value;
};

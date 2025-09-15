// Trie implementation using unique_ptr only for ownership inside nodes.
#pragma once
#include "TrieNode.hpp"
#include <limits>
#include <memory>
#include <optional>
#include <stack>
#include <string>
#include <vector>

/**
 * @brief Generic Trie class. uses strings as the key.
 *
 * @tparam T generic Type T. does not have to implement any methods.
 */
template <typename T>
class Trie {
  public:
    Trie() : root(std::make_unique<TrieNode<T>>()) {}
    ~Trie() = default;

    void insert(const std::string &key, const T &value);
    std::optional<T> get(const std::string &key) const;
    bool remove(const std::string &key);
    bool prefixSearch(const std::string &prefix) const;
    std::vector<T> autocomplete(const std::string &prefix, int limit = 50) const;
    std::vector<T> suggest(std::string);

  private:
    std::unique_ptr<TrieNode<T>> root;
};

/**
 * @brief Insert a new key value pair.
 *
 * @tparam T Generic type T of the value to store.
 * @param key string used to store the object in the Trie.
 * @param value value to store.
 *
 * @code{.cpp}
 * Trie<int> friendCount;
 * friendCount.insert("bob", 5);
 * @endcode
 */
template <typename T>
void Trie<T>::insert(const std::string &key, const T &value) {
    TrieNode<T> *current = root.get();
    for (char c : key) {
        current = current->addChild(c);
    }
    if (!current->getValue().has_value() || *(current->getValue()) != value) {
        current->setValue(value);
    }
}

/**
 * @brief Get the value of stored at the Key.
 *
 * @tparam T Generic Type for the Key.
 * @param key Key for which the value to retrieve is stored.
 */
template <typename T>
std::optional<T> Trie<T>::get(const std::string &key) const {
    const TrieNode<T> *current = root.get();
    for (char c : key) {
        current = current->getChild(c);
        if (!current)
            return std::nullopt;
    }
    return current->getValue();
}

/**
 * @brief remove a Node from the Trie.
 *
 * @tparam T Generic type T for the Key.
 * @param key The key to remove from the Trie.
 * @return True if the removal was successful.
 */
template <typename T>
bool Trie<T>::remove(const std::string &key) {
    // Track path for potential pruning.
    std::vector<std::pair<TrieNode<T> *, char>> path;
    TrieNode<T> *current = root.get();
    for (char c : key) {
        TrieNode<T> *next = current->getChild(c);
        if (!next)
            return false;
        path.emplace_back(current, c);
        current = next;
    }
    if (!current->isEnd())
        return false; // key not present
    current->clearValue();

    // prune from leaf upward
    for (int i = (int)path.size() - 1; i >= 0; --i) {
        TrieNode<T> *parent = path[i].first;
        char edge = path[i].second;
        TrieNode<T> *child = parent->getChild(edge);
        if (child && !child->hasChildren() && !child->isEnd()) {
            parent->removeChild(edge);
        } else {
            break; // stop when node still needed
        }
    }
    return true;
}

template <typename T>
bool Trie<T>::prefixSearch(const std::string &prefix) const {
    const TrieNode<T> *current = root.get();
    for (char c : prefix) {
        current = current->getChild(c);
        if (!current)
            return false;
    }
    return true;
}

template <typename T>
std::vector<T> Trie<T>::autocomplete(const std::string &prefix, int limit) const {
    if (limit == -1)
        limit = std::numeric_limits<unsigned int>::max();
    const TrieNode<T> *start = root.get();
    for (char c : prefix) {
        start = start->getChild(c);
        if (!start)
            return {};
    }
    std::vector<T> results;
    std::stack<const TrieNode<T> *> dfs;
    dfs.push(start);
    while (!dfs.empty() && (int)results.size() < limit) {
        const TrieNode<T> *node = dfs.top();
        dfs.pop();
        if (node->isEnd())
            results.push_back(*(node->getValue()));
        for (const auto &entry : node->getChildren()) {
            dfs.push(entry.second.get());
        }
    }
    return results;
}

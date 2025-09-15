#pragma once
#include "TrieNode.hpp"
#include <memory>
#include <optional>
#include <string>
#include <vector>

template <typename T>
class Trie {
  public:
    Trie() = default;
    ~Trie() = default;
    void insert(std::string &, T &);
    std::optional<T> get(std::string &);
    bool remove(std::string);
    bool prefixSearch(std::string);
    std::vector<T> autocomplete(std::string, int = 50);
    std::vector<T> suggest(std::string);

  private:
    std::unique_ptr<TrieNode<T>> root;
};

template <typename T>
void Trie<T>::insert(std::string &key, T &value) {
    std::unique_ptr<TrieNode<T>> current = root;
    for (char c : key) {
        if (!current->getChild(c)) {
            current->addChild(c);
        }
        current = current->getChild(c);
    }
    if (current->getValue() == value) {
        return;
    }
    current->setValue(value);
}

template <typename T>
std::optional<T> Trie<T>::get(std::string &key) {
    std::unique_ptr<TrieNode<T>> current = root;
    for (char c : key) {
        if (!current->getChild(c)) {
            return std::nullopt;
        }
        current = current->getChild(c);
    }

    return current->getValue();
}

template <typename T>
bool Trie<T>::remove(std::string key) {
    std::unique_ptr<TrieNode<T>> current = root;
    std::vector<std::unique_ptr<TrieNode<T>>> visited;

    for (char c : key) {
        if (!current->getChild(c)) {
            return false;
        }
        visited.push_back(current);
        current = current->getChild(c);
    }
    if (current == root) {
        delete root;
        root = nullptr;
        return true;
    }

    int last = key.length() - 1;
    current->clearValue();
    while (!visited.empty()) {
        std::unique_ptr<TrieNode<T>> parent = visited.back();
        if (!current->hasChildren() && !current->isEnd()) {
            parent->removeChild(key[last]);
        }
        last -= 1;
    }
    return true;
}

template <typename T>
bool Trie<T>::prefixSearch(std::string key) {
    std::unique_ptr<TrieNode<T>> current = root;
    // std::vector<T>

    for (char c : key) {
        if (!current->getChild(c)) {
            return false;
        }
        current = current->getChild(c);
    }
    return true;
}

template <typename T>
std::vector<T> Trie<T>::autocomplete(std::string key, int limit) {
    std::unique_ptr<TrieNode<T>> current = root;
    std::vector<T> words;
    for (char c : key) {
        if (!current->getChild(c)) {
            return words;
        }
        current = current->getChild(c);
    }

    if (current.isEnd()) {
        words.push_back(current.getValue());
    }

    for (TrieNode<T> child : current) {
    }

    return words;
}

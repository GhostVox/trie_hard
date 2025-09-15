#pragma once
#include <memory>
#include <optional>
#include <unordered_map>

template <typename T>
class TrieNode {
  public:
    TrieNode() = default;
    ~TrieNode() = default;
    bool hasChildren();
    std::optional<std::unique_ptr<TrieNode<T>>> getChild(char);
    std::unique_ptr<TrieNode<T>> addChild(char);
    void removeChild(char);
    bool isEnd();
    std::optional<T> getValue();
    std::optional<T> clearValue();
    void setValue(T);
    auto begin() const { return children.cbegin(); }
    auto end() const { return children.cend(); }

  private:
    std::unordered_map<char, std::unique_ptr<TrieNode<T>>> children;
    std::optional<T> value;
};

template <typename T>
bool TrieNode<T>::hasChildren() {
    return children.empty();
}

template <typename T>
std::optional<std::unique_ptr<TrieNode<T>>> TrieNode<T>::getChild(char key) {
    if (children.find(key) != children.end()) {
        return children[key];
    }
    return std::nullopt;
}

template <typename T>
std::unique_ptr<TrieNode<T>> TrieNode<T>::addChild(char key) {
    if (children.find(key) != children.end()) {
        children[key] = std::make_unique<TrieNode<T>>();
    }
    return children[key];
}

template <typename T>
void TrieNode<T>::removeChild(char key) {
    children.erase(key);
}

template <typename T>
bool TrieNode<T>::isEnd() {
    return value.has_value();
}

template <typename T>
std::optional<T> TrieNode<T>::getValue() {
    return value;
}

template <typename T>
std::optional<T> TrieNode<T>::clearValue() {
    std::optional<T> v = value;
    value = std::nullopt;
    return v;
}

template <typename T>
void TrieNode<T>::setValue(T value) {
    this->value = value;
}

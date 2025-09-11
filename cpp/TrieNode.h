#include <memory>
#include <optional>
#include <string>
#include <unordered_map>
using namespace std;

template <typename T>
class TrieNode {
  public:
    TrieNode();
    ~TrieNode();
    bool hasChildren();
    optional<unique_ptr<TrieNode<T>>> getChild(char);
    unique_ptr<TrieNode<T>> addChild(char);
    void removeChild(char);
    bool isEnd();
    optional<T> getValue();
    optional<T> clearValue();
    void setValue(T);
    auto begin() const { return children.cbegin(); }
    auto end() const { return children.cend(); }

  private:
    unordered_map<char, unique_ptr<TrieNode<T>>> children;
    optional<T> value;
};

template <typename T>
TrieNode<T>::TrieNode() = default;

template <typename T>
TrieNode<T>::~TrieNode() = default;

template <typename T>
bool TrieNode<T>::hasChildren() {
    return children.empty();
}

template <typename T>
optional<unique_ptr<TrieNode<T>>> TrieNode<T>::getChild(char key) {
    if (children.find(key) != children.end()) {
        return children[key];
    }
    return nullopt;
}

template <typename T>
unique_ptr<TrieNode<T>> TrieNode<T>::addChild(char key) {
    if (children.find(key) != children.end()) {
        children[key] = make_unique<TrieNode<T>>();
    }
    return children[key];
}

template <typename T>
void TrieNode<T>::removeChild(char key) {
    children.erase(key);
}

#include "TrieNode.hpp"
#include <memory>
#include <optional>
#include <string>
#include <vector>

template <typename T>
class Trie {
  public:
    Trie();
    ~Trie();
    void insert(std::string &, T &);
    std::optional<T> get(std::string &);
    bool remove(std::string);
    std::vector<T> prefixSearch(std::string);
    std::vector<T> autocomplete(std::string);
    std::vector<T> suggest(std::string);

  private:
    std::unique_ptr<TrieNode<T>> root;
};

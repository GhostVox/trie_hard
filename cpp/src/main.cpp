#include "Trie.hpp"
#include <iostream>

int main() {
    Trie<int> trie;
    trie.insert("cat", 1);
    trie.insert("car", 2);
    trie.insert("dog", 3);

    auto v = trie.get("car");
    if (v)
        std::cout << "car = " << *v << "\n";

    auto ac = trie.autocomplete("ca");
    std::cout << "Autocomplete ca -> ";
    for (auto &x : ac)
        std::cout << x << ' ';
    std::cout << "\n";
    return 0;
}

# Trie Hard

## Overview

Trie hard is a library of tries implemented in different programming languages. It provides a simple and efficient way to take a list of words or phrases that you want to search for to find them quickly and easily. One use case is having a database of words or phrases that you want to filtered out of comments on your website. When you combine your database with Trie Hard's API, you can quickly filter out unwanted content while having data persistence.

## Supported Languages

- Python
- TypeScript
- Java
- C++
- Rust
- Go
- Swift
- Kotlin
- C
- Nim ?
- C#
- Ruby
- php
- bash
- D
- B
- F#
- elixir

## Methods supported consistently though all languages

where `T` is a generic type

- `insert(word: str) -> None`: Inserts a word into the trie.
- `get(word: str) -> T`: Searches for a word in the trie.
- `delete(word: str) -> bool`: Deletes a word from the trie.
- `prefix_search(prefix: str) -> bool`: Searches for words with a given prefix.
- `autocomplete(prefix: str) -> List[str]`: Autocompletes a word with a given prefix using either dfs or bfs to find all possible completions.
- `suggest(word: str) -> List[str]`: Suggests words that are similar to the given word.

## Contributions Welcomed

To have the contributions pullled into the repo please make sure to follow the guidelines below

- Make sure your code is well documented and follows the style guide of the language you are contributing to.
- Make sure your code is tested and passes all tests.
- Make sure your code is efficient and does not have any memory leaks.
- If you are writting the library for a new language, please make sure to implement all the unviersal methods, you are more then welcome to add additional functionality to your library.

# trie_hard_rs - Rust Implementation

[![Crates.io](https://img.shields.io/crates/v/trie_hard_rs.svg)](https://crates.io/crates/trie_hard_rs)
[![Documentation](https://docs.rs/trie_hard_rs/badge.svg)](https://docs.rs/trie_hard_rs)
[![CI](https://github.com/GhostVox/trie_hard/workflows/CI/badge.svg)](https://github.com/GhostVox/trie_hard/actions)

A blazing fast, memory-efficient Trie (prefix tree) implementation for Rust with autocomplete support.

**Part of the [trie_hard family](https://github.com/GhostVox/trie_hard) - high-performance Trie implementations across multiple programming languages.**

## Language Implementations

This repository contains Trie implementations for multiple languages, each optimized for its respective ecosystem:

- **[Rust](https://github.com/GhostVox/trie_hard/tree/main/rust)** ← You are here
- **[Go](https://github.com/GhostVox/trie_hard/tree/main/go)** - `go get github.com/GhostVox/trie_hard/go`

*More languages coming soon!*

## Performance

This Rust implementation delivers exceptional performance:

- **Insert 10K words**: 2.17ms
- **Autocomplete (10 results)**: 18.4μs
- **Lookup**: 165μs
- **Memory efficient** with shared prefixes
- **Unicode support** with minimal overhead

## Quick Start

Add to your `Cargo.toml`:
```toml
[dependencies]
trie_hard_rs = "0.1"
```

Basic usage:
```rust
use trie_hard_rs::Trie;

let mut trie = Trie::new();

// Insert words with associated values
trie.insert("cat", &1);
trie.insert("car", &2);
trie.insert("card", &3);

// Fast lookups
assert_eq!(trie.get("cat"), Some(&1));

// Prefix search
assert!(trie.prefix_search("ca"));

// Autocomplete
let suggestions = trie.auto_complete("ca", 10);
// Returns: ["cat", "car", "card"]
```

## Features

- **Generic values**: Store any `Clone` type as values
- **Batch operations**: Efficient `add_word_list` with value generators
- **Unicode support**: Full UTF-8 character support
- **Memory efficient**: Shared prefix storage
- **Fast autocomplete**: Configurable result limits
- **Comprehensive tests**: 37 test cases covering edge cases
- **Benchmarked**: Proven performance characteristics

## Benchmarks

Run benchmarks with:
```bash
cargo bench
```

View detailed HTML reports at `target/criterion/report/index.html`

## Advanced Usage

### Batch Insertion with Value Generation
```rust
let words = ["cat", "car", "card"];
trie.add_word_list(&words, |word| word.len());
// Inserts with word length as value
```

### Custom Value Types
```rust
let mut trie: Trie<(u32, String)> = Trie::new();
trie.insert("word", &(42, "metadata".to_string()));
```

### Autocomplete with Limits
```rust
// Get up to 5 suggestions
let suggestions = trie.auto_complete("prefix", 5);

// Get all suggestions (no limit)
let all_suggestions = trie.auto_complete("prefix", usize::MAX);
```

### Working with Different Value Types
```rust
// String values
let mut string_trie = Trie::new();
string_trie.insert("key", &"value".to_string());

// Numeric values for scoring/ranking
let mut scored_trie = Trie::new();
scored_trie.insert("popular", &100);
scored_trie.insert("common", &50);

// Complex data structures
#[derive(Clone)]
struct WordData {
    frequency: u32,
    category: String,
}

let mut data_trie = Trie::new();
data_trie.insert("example", &WordData {
    frequency: 42,
    category: "noun".to_string(),
});
```

## API Reference

### Core Methods

- `new()` - Create a new empty Trie
- `insert(key, value)` - Insert a key-value pair
- `get(key)` - Get value by exact key match
- `delete(key)` - Remove a key and its value
- `prefix_search(prefix)` - Check if any words start with prefix
- `auto_complete(prefix, max_results)` - Get words starting with prefix

### Batch Operations

- `add_word_list(words, value_generator)` - Insert multiple words with generated values

## Performance Characteristics

- **Time Complexity**:
  - Insert: O(k) where k = key length
  - Lookup: O(k) where k = key length
  - Autocomplete: O(k + m) where k = prefix length, m = results returned
  - Delete: O(k) where k = key length

- **Space Complexity**: O(ALPHABET_SIZE * N * M) where N = number of nodes, M = average key length
  - Efficient prefix sharing reduces actual memory usage significantly

## Cross-Language Compatibility

While each implementation is optimized for its language, they share:

- **Consistent API design** across all languages
- **Similar performance characteristics**
- **Compatible data formats** for cross-language projects
- **Shared test cases** to ensure behavioral consistency

## Why trie_hard?

The trie_hard family was created to provide:

1. **Performance-first design** - Optimized for speed and memory efficiency
2. **Production ready** - Comprehensive testing and benchmarking
3. **Cross-language consistency** - Same API patterns across implementations
4. **Developer friendly** - Clear documentation and examples
5. **Unicode support** - Works with international text out of the box

## Comparison with Other Rust Trie Crates

| Feature | trie_hard | Other Crates |
|---------|-----------|--------------|
| Autocomplete with limits | ✅ | ❌ |
| Generic value types | ✅ | Limited |
| Comprehensive benchmarks | ✅ | Limited |
| Unicode support | ✅ | Varies |
| Batch operations | ✅ | ❌ |
| Sub-microsecond autocomplete | ✅ | ❌ |

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request to the [main repository](https://github.com/GhostVox/trie_hard).

### Development

```bash
# Run tests
cargo test

# Run benchmarks
cargo bench

# Check formatting
cargo fmt --check

# Run lints
cargo clippy
```

## Related Projects

- [Main trie_hard repository](https://github.com/GhostVox/trie_hard) - All language implementations
- [Go implementation](https://github.com/GhostVox/trie_hard/tree/main/go)

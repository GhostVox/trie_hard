use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use std::collections::HashSet;
use trie_hard_rs::Trie;

// Helper function to generate test data
fn generate_words(count: usize, prefix: &str) -> Vec<String> {
    (0..count).map(|i| format!("{}{:06}", prefix, i)).collect()
}

fn generate_realistic_words(count: usize) -> Vec<String> {
    let prefixes = [
        "app", "test", "user", "data", "file", "sys", "web", "api", "db", "cache",
    ];
    let suffixes = [
        "_config",
        "_manager",
        "_service",
        "_handler",
        "_controller",
        "_model",
        "_view",
        "_util",
        "_helper",
        "_factory",
    ];

    (0..count)
        .map(|i| {
            let prefix = prefixes[i % prefixes.len()];
            let suffix = suffixes[i % suffixes.len()];
            format!("{}{}{}", prefix, i % 1000, suffix)
        })
        .collect()
}

fn generate_english_like_words(count: usize) -> Vec<String> {
    let consonants = "bcdfghjklmnpqrstvwxyz";
    let vowels = "aeiou";
    let mut words = Vec::new();

    for i in 0..count {
        let mut word = String::new();
        let length = 3 + (i % 8); // Words 3-10 characters long

        for j in 0..length {
            if j % 2 == 0 {
                word.push(consonants.chars().nth(i % consonants.len()).unwrap());
            } else {
                word.push(vowels.chars().nth(i % vowels.len()).unwrap());
            }
        }
        words.push(word);
    }

    words
}

// Benchmark single insertions
fn bench_insert_single(c: &mut Criterion) {
    let mut group = c.benchmark_group("insert_single");

    for size in [100, 1000, 10000].iter() {
        let words = generate_words(*size, "word");

        group.bench_with_input(BenchmarkId::new("sequential", size), size, |b, _| {
            b.iter(|| {
                let mut trie = Trie::new();
                for word in &words {
                    trie.insert(black_box(word), black_box(&1));
                }
                black_box(trie)
            })
        });
    }

    group.finish();
}

// Benchmark batch insertions using add_word_list
fn bench_insert_batch(c: &mut Criterion) {
    let mut group = c.benchmark_group("insert_batch");

    for size in [100, 1000, 10000].iter() {
        let words = generate_words(*size, "batch");
        let word_refs: Vec<&str> = words.iter().map(|s| s.as_str()).collect();

        group.bench_with_input(BenchmarkId::new("add_word_list", size), size, |b, _| {
            b.iter(|| {
                let mut trie = Trie::new();
                trie.add_word_list(black_box(&word_refs), |word| word.len());
                black_box(trie)
            })
        });
    }

    group.finish();
}

// Benchmark lookups
fn bench_lookup(c: &mut Criterion) {
    let mut group = c.benchmark_group("lookup");

    for size in [100, 1000, 10000].iter() {
        let words = generate_words(*size, "lookup");
        let mut trie = Trie::new();

        // Pre-populate the trie
        for word in &words {
            trie.insert(word, &1);
        }

        // Benchmark successful lookups
        group.bench_with_input(BenchmarkId::new("hit", size), size, |b, _| {
            b.iter(|| {
                for word in &words {
                    let result = trie.get(black_box(word));
                    black_box(result);
                }
            })
        });

        // Benchmark failed lookups
        let missing_words = generate_words(*size, "missing");
        group.bench_with_input(BenchmarkId::new("miss", size), size, |b, _| {
            b.iter(|| {
                for word in &missing_words {
                    let result = trie.get(black_box(word));
                    black_box(result);
                }
            })
        });
    }

    group.finish();
}

// Benchmark prefix search
fn bench_prefix_search(c: &mut Criterion) {
    let mut group = c.benchmark_group("prefix_search");

    for size in [1000, 10000].iter() {
        let words = generate_realistic_words(*size);
        let mut trie = Trie::new();

        // Pre-populate the trie
        for word in &words {
            trie.insert(word, &1);
        }

        let prefixes = ["app", "test", "user", "web", "nonexistent"];

        group.bench_with_input(BenchmarkId::new("realistic", size), size, |b, _| {
            b.iter(|| {
                for prefix in &prefixes {
                    let result = trie.prefix_search(black_box(prefix));
                    black_box(result);
                }
            })
        });
    }

    group.finish();
}

// Benchmark autocomplete with different result limits
fn bench_autocomplete(c: &mut Criterion) {
    let mut group = c.benchmark_group("autocomplete");

    // Test with realistic word distribution
    let words = generate_realistic_words(10000);
    let mut trie = Trie::new();

    for word in &words {
        trie.insert(word, &1);
    }

    let prefixes = ["app", "test", "user", "data", "nonexistent"];

    for max_results in [5, 10, 50, 100].iter() {
        group.bench_with_input(
            BenchmarkId::new("realistic", max_results),
            max_results,
            |b, &max_results| {
                b.iter(|| {
                    for prefix in &prefixes {
                        let results = trie.auto_complete(black_box(prefix), black_box(max_results));
                        black_box(results);
                    }
                })
            },
        );
    }

    group.finish();
}

// Benchmark autocomplete with varying trie sizes
fn bench_autocomplete_scaling(c: &mut Criterion) {
    let mut group = c.benchmark_group("autocomplete_scaling");

    for size in [1000, 5000, 10000, 20000].iter() {
        let words = generate_english_like_words(*size);
        let mut trie = Trie::new();

        for word in &words {
            trie.insert(word, &1);
        }

        group.bench_with_input(BenchmarkId::new("english_like", size), size, |b, _| {
            b.iter(|| {
                // Test common prefix lengths
                for prefix_len in 1..=3 {
                    let prefix = &"test"[..prefix_len];
                    let results = trie.auto_complete(black_box(prefix), black_box(10));
                    black_box(results);
                }
            })
        });
    }

    group.finish();
}

// Benchmark deletion operations
fn bench_delete(c: &mut Criterion) {
    let mut group = c.benchmark_group("delete");

    for size in [100, 1000, 5000].iter() {
        let words = generate_words(*size, "delete");

        group.bench_with_input(BenchmarkId::new("sequential", size), size, |b, _| {
            b.iter_batched(
                || {
                    // Setup: create a fresh trie for each iteration
                    let mut trie = Trie::new();
                    for word in &words {
                        trie.insert(word, &1);
                    }
                    trie
                },
                |mut trie| {
                    // Actual benchmark: delete all words
                    for word in &words {
                        let result = trie.delete(black_box(word));
                        black_box(result);
                    }
                    black_box(trie)
                },
                criterion::BatchSize::SmallInput,
            )
        });
    }

    group.finish();
}

// Benchmark memory efficiency by comparing with other data structures
fn bench_memory_comparison(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_comparison");

    let words = generate_realistic_words(10000);

    // Benchmark Trie construction
    group.bench_function("trie_construction", |b| {
        b.iter(|| {
            let mut trie = Trie::new();
            for word in &words {
                trie.insert(black_box(word), black_box(&1));
            }
            black_box(trie)
        })
    });

    // Benchmark HashSet construction for comparison
    group.bench_function("hashset_construction", |b| {
        b.iter(|| {
            let mut set = HashSet::new();
            for word in &words {
                set.insert(black_box(word.clone()));
            }
            black_box(set)
        })
    });

    group.finish();
}

// Benchmark worst-case scenarios
fn bench_worst_case(c: &mut Criterion) {
    let mut group = c.benchmark_group("worst_case");

    // Create a pathological case: all words share a long common prefix
    let long_prefix = "supercalifragilisticexpialidocious";
    let words: Vec<String> = (0..1000).map(|i| format!("{}{}", long_prefix, i)).collect();

    let mut trie = Trie::new();
    for word in &words {
        trie.insert(word, &1);
    }

    group.bench_function("long_common_prefix_autocomplete", |b| {
        b.iter(|| {
            let results = trie.auto_complete(black_box(long_prefix), black_box(100));
            black_box(results);
        })
    });

    // Test with very long individual words
    let very_long_words: Vec<String> = (0..100)
        .map(|i| format!("{}{}", "a".repeat(1000), i))
        .collect();

    group.bench_function("very_long_words_insert", |b| {
        b.iter(|| {
            let mut trie = Trie::new();
            for word in &very_long_words {
                trie.insert(black_box(word), black_box(&1));
            }
            black_box(trie)
        })
    });

    group.finish();
}

// Real-world simulation benchmark
fn bench_real_world_simulation(c: &mut Criterion) {
    let mut group = c.benchmark_group("real_world_simulation");

    // Simulate a real autocomplete system with mixed operations
    let words = generate_realistic_words(5000);
    let mut trie = Trie::new();

    // Pre-populate with initial data
    for word in &words[..4000] {
        trie.insert(word, &1);
    }

    group.bench_function("mixed_operations", |b| {
        b.iter(|| {
            // Simulate user typing and getting suggestions
            let user_queries = ["app", "te", "user", "data", "fil"];

            for query in &user_queries {
                // Autocomplete simulation
                let results = trie.auto_complete(black_box(query), black_box(10));
                black_box(results);

                // Prefix check simulation
                let has_prefix = trie.prefix_search(black_box(query));
                black_box(has_prefix);
            }

            // Simulate adding a few new words
            for i in 0..5 {
                let new_word = format!("dynamic_word_{}", i);
                trie.insert(black_box(&new_word), black_box(&i));
            }

            // Simulate some lookups
            for word in &words[..10] {
                let result = trie.get(black_box(word));
                black_box(result);
            }
        })
    });

    group.finish();
}

// Benchmark unicode performance
fn bench_unicode(c: &mut Criterion) {
    let mut group = c.benchmark_group("unicode");

    let unicode_words = vec![
        "café",
        "naïve",
        "résumé",
        "🦀",
        "你好",
        "مرحبا",
        "здравствуй",
        "こんにちは",
        "γεια σας",
        "שלום",
        "நமஸ்காரம்",
        "🌟⭐✨",
    ];

    // Extend with variations
    let extended_unicode: Vec<String> = (0..1000)
        .map(|i| format!("{}{}", unicode_words[i % unicode_words.len()], i))
        .collect();

    group.bench_function("unicode_insert", |b| {
        b.iter(|| {
            let mut trie = Trie::new();
            for word in &extended_unicode {
                trie.insert(black_box(word), black_box(&1));
            }
            black_box(trie)
        })
    });

    let mut unicode_trie = Trie::new();
    for word in &extended_unicode {
        unicode_trie.insert(word, &1);
    }

    group.bench_function("unicode_autocomplete", |b| {
        b.iter(|| {
            for prefix in &["caf", "🦀", "你", "مرح"] {
                let results = unicode_trie.auto_complete(black_box(prefix), black_box(10));
                black_box(results);
            }
        })
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_insert_single,
    bench_insert_batch,
    bench_lookup,
    bench_prefix_search,
    bench_autocomplete,
    bench_autocomplete_scaling,
    bench_delete,
    bench_memory_comparison,
    bench_worst_case,
    bench_real_world_simulation,
    bench_unicode
);

criterion_main!(benches);

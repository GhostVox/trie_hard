package trie

import (
	"fmt"
	"runtime"
	"strings"
	"testing"
)

// Helper functions to generate test data (matching Rust implementation)
func generateWords(count int, prefix string) []string {
	words := make([]string, count)
	for i := 0; i < count; i++ {
		words[i] = fmt.Sprintf("%s%06d", prefix, i)
	}
	return words
}

func generateRealisticWords(count int) []string {
	prefixes := []string{"app", "test", "user", "data", "file", "sys", "web", "api", "db", "cache"}
	suffixes := []string{"_config", "_manager", "_service", "_handler", "_controller", "_model", "_view", "_util", "_helper", "_factory"}

	words := make([]string, count)
	for i := 0; i < count; i++ {
		prefix := prefixes[i%len(prefixes)]
		suffix := suffixes[i%len(suffixes)]
		words[i] = fmt.Sprintf("%s%d%s", prefix, i%1000, suffix)
	}
	return words
}

func generateEnglishLikeWords(count int) []string {
	consonants := "bcdfghjklmnpqrstvwxyz"
	vowels := "aeiou"
	words := make([]string, count)

	for i := 0; i < count; i++ {
		var word strings.Builder
		length := 3 + (i % 8) // Words 3-10 characters long

		for j := 0; j < length; j++ {
			if j%2 == 0 {
				word.WriteByte(consonants[i%len(consonants)])
			} else {
				word.WriteByte(vowels[i%len(vowels)])
			}
		}
		words[i] = word.String()
	}
	return words
}

// Benchmark single insertions
func BenchmarkInsertSingle(b *testing.B) {
	sizes := []int{100, 1000, 10000}

	for _, size := range sizes {
		words := generateWords(size, "word")

		b.Run(fmt.Sprintf("sequential-%d", size), func(b *testing.B) {
			b.ResetTimer()
			for i := 0; i < b.N; i++ {
				trie := NewTrie[int]()
				for _, word := range words {
					trie.Insert(&word, i)
				}
				runtime.KeepAlive(trie)
			}
		})
	}
}

// Benchmark batch insertions using AddWordList
func BenchmarkInsertBatch(b *testing.B) {
	sizes := []int{100, 1000, 10000}

	for _, size := range sizes {
		words := generateWords(size, "batch")

		b.Run(fmt.Sprintf("add_word_list-%d", size), func(b *testing.B) {
			b.ResetTimer()
			for i := 0; i < b.N; i++ {
				trie := NewTrie[int]()
				trie.AddWordList(&words, func(word string) int { return len(word) })
				runtime.KeepAlive(trie)
			}
		})
	}
}

// Benchmark lookups
func BenchmarkLookup(b *testing.B) {
	sizes := []int{100, 1000, 10000}

	for _, size := range sizes {
		words := generateWords(size, "lookup")
		trie := NewTrie[int]()

		// Pre-populate the trie
		for _, word := range words {
			trie.Insert(&word, 1)
		}

		b.Run(fmt.Sprintf("hit-%d", size), func(b *testing.B) {
			b.ResetTimer()
			for i := 0; i < b.N; i++ {
				for _, word := range words {
					result, _ := trie.Get(&word)
					runtime.KeepAlive(result)
				}
			}
		})

		// Benchmark failed lookups
		missingWords := generateWords(size, "missing")
		b.Run(fmt.Sprintf("miss-%d", size), func(b *testing.B) {
			b.ResetTimer()
			for i := 0; i < b.N; i++ {
				for _, word := range missingWords {
					result, _ := trie.Get(&word)
					runtime.KeepAlive(result)
				}
			}
		})
	}
}

// Benchmark prefix search
func BenchmarkPrefixSearch(b *testing.B) {
	sizes := []int{1000, 10000}

	for _, size := range sizes {
		words := generateRealisticWords(size)
		trie := NewTrie[int]()

		// Pre-populate the trie
		for _, word := range words {
			trie.Insert(&word, 1)
		}

		prefixes := []string{"app", "test", "user", "web", "nonexistent"}

		b.Run(fmt.Sprintf("realistic-%d", size), func(b *testing.B) {
			b.ResetTimer()
			for i := 0; i < b.N; i++ {
				for _, prefix := range prefixes {
					result := trie.PrefixSearch(&prefix)
					runtime.KeepAlive(result)
				}
			}
		})
	}
}

// Benchmark autocomplete with different result limits
func BenchmarkAutoComplete(b *testing.B) {
	words := generateRealisticWords(10000)
	trie := NewTrie[int]()

	for _, word := range words {
		trie.Insert(&word, 1)
	}

	prefixes := []string{"app", "test", "user", "data", "nonexistent"}
	maxResults := []int{5, 10, 50, 100}

	for _, maxResult := range maxResults {
		b.Run(fmt.Sprintf("realistic-%d", maxResult), func(b *testing.B) {
			b.ResetTimer()
			for i := 0; i < b.N; i++ {
				for _, prefix := range prefixes {
					results := trie.AutoComplete(&prefix)
					if len(results) > maxResult {
						results = results[:maxResult]
					}
					runtime.KeepAlive(results)
				}
			}
		})
	}
}

// Benchmark autocomplete with varying trie sizes
func BenchmarkAutoCompleteScaling(b *testing.B) {
	sizes := []int{1000, 5000, 10000, 20000}

	for _, size := range sizes {
		words := generateEnglishLikeWords(size)
		trie := NewTrie[int]()

		for _, word := range words {
			trie.Insert(&word, 1)
		}

		b.Run(fmt.Sprintf("english_like-%d", size), func(b *testing.B) {
			b.ResetTimer()
			for i := 0; i < b.N; i++ {
				// Test common prefix lengths
				for prefixLen := 1; prefixLen <= 3; prefixLen++ {
					prefix := "test"[:prefixLen]
					results := trie.AutoComplete(&prefix)
					if len(results) > 10 {
						results = results[:10]
					}
					runtime.KeepAlive(results)
				}
			}
		})
	}
}

// Benchmark deletion operations
func BenchmarkDelete(b *testing.B) {
	sizes := []int{100, 1000, 5000}

	for _, size := range sizes {
		words := generateWords(size, "delete")

		b.Run(fmt.Sprintf("sequential-%d", size), func(b *testing.B) {
			b.ResetTimer()
			for i := 0; i < b.N; i++ {
				b.StopTimer()
				// Setup: create a fresh trie for each iteration
				trie := NewTrie[int]()
				for _, word := range words {
					trie.Insert(&word, 1)
				}
				b.StartTimer()

				// Actual benchmark: delete all words
				for _, word := range words {
					result := trie.Delete(&word)
					runtime.KeepAlive(result)
				}
				runtime.KeepAlive(trie)
			}
		})
	}
}

// Real-world simulation benchmark
func BenchmarkRealWorldSimulation(b *testing.B) {
	words := generateRealisticWords(5000)
	trie := NewTrie[int]()

	// Pre-populate with initial data
	for _, word := range words[:4000] {
		trie.Insert(&word, 1)
	}

	b.Run("mixed_operations", func(b *testing.B) {
		b.ResetTimer()
		for i := 0; i < b.N; i++ {
			// Simulate user typing and getting suggestions
			userQueries := []string{"app", "te", "user", "data", "fil"}

			for _, query := range userQueries {
				// Autocomplete simulation
				results := trie.AutoComplete(&query)
				if len(results) > 10 {
					results = results[:10]
				}
				runtime.KeepAlive(results)

				// Prefix check simulation
				hasPrefix := trie.PrefixSearch(&query)
				runtime.KeepAlive(hasPrefix)
			}

			// Simulate adding a few new words
			for j := 0; j < 5; j++ {
				newWord := fmt.Sprintf("dynamic_word_%d", j)
				trie.Insert(&newWord, j)
			}

			// Simulate some lookups
			for _, word := range words[:10] {
				result, _ := trie.Get(&word)
				runtime.KeepAlive(result)
			}
		}
	})
}

// Benchmark unicode performance
func BenchmarkUnicode(b *testing.B) {
	unicodeWords := []string{
		"café", "naïve", "résumé", "🦀", "你好", "مرحبا",
		"здравствуй", "こんにちは", "γεια σας", "שלום", "நமஸ்காரம்", "🌟⭐✨",
	}

	// Extend with variations
	extendedUnicode := make([]string, 1000)
	for i := 0; i < 1000; i++ {
		extendedUnicode[i] = fmt.Sprintf("%s%d", unicodeWords[i%len(unicodeWords)], i)
	}

	b.Run("unicode_insert", func(b *testing.B) {
		b.ResetTimer()
		for i := 0; i < b.N; i++ {
			trie := NewTrie[int]()
			for _, word := range extendedUnicode {
				trie.Insert(&word, 1)
			}
			runtime.KeepAlive(trie)
		}
	})

	unicodeTrie := NewTrie[int]()
	for _, word := range extendedUnicode {
		unicodeTrie.Insert(&word, 1)
	}

	b.Run("unicode_autocomplete", func(b *testing.B) {
		prefixes := []string{"caf", "🦀", "你", "مرح"}
		b.ResetTimer()
		for i := 0; i < b.N; i++ {
			for _, prefix := range prefixes {
				results := unicodeTrie.AutoComplete(&prefix)
				if len(results) > 10 {
					results = results[:10]
				}
				runtime.KeepAlive(results)
			}
		}
	})
}

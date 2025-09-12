using System.Diagnostics.CodeAnalysis;

namespace TrieHard;

public struct UnsafeTrieBlock
{ 
    public bool IsFullValue { get; }

    public TrieBlock(TItem value, bool isFullValue = false)
    {
        this.value = value;
        IsFullValue = isFullValue;
    }
    
    public static bool operator true(TrieBlock<TItem> block)
    {
        return block.IsFullValue;
    }

    public static bool operator false(TrieBlock<TItem> block)
    {
        return !block.IsFullValue;
    }

    public static implicit operator TItem(TrieBlock<TItem> block)
    {
        return block.value;
    }

    public static bool operator ==(TrieBlock<TItem> left, TrieBlock<TItem> right)
    {
        return (left.value?.Equals(right.value) ?? false) && left.IsFullValue == right.IsFullValue;
    }

    public static bool operator !=(TrieBlock<TItem> left, TrieBlock<TItem> right)
    {
        return !(left == right);
    }

    public override bool Equals([NotNullWhen(true)] object? obj)
    {
        return base.Eq
    }
}
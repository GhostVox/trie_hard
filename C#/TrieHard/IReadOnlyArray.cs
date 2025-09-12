namespace TrieHard;

public interface IReadOnlyArray<T> : IReadOnlyCollection<T>, IIndexable<T>
{
    public new ref T this[int index] { get; }
}
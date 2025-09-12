namespace TrieHard;

public interface IIndexable<out T>
{
    public T this[int index] { get; }
}
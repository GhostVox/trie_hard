namespace TrieHard;

public delegate bool RefPredicate<T>(ref T value);

public delegate bool RefPredicateUnmanaged<T>(ref T value);

public interface ITrie<T, TItems> where T : IIndexable<TItems>
{
    public bool this[T value] => Contains(value);

    public bool Contains(T value);

    public void Insert(T value);
    
    public void Delete(T value);

    public bool PrefixExists(T prefix);
    
    public List<T> PrefixSearch(T prefix);

    public List<T> Complete(T prefix);

    public List<T> FuzzySearch(T prefix, RefPredicate<T> searchFunction)
    {
        FuzzySearchFnBridge<T>.Current = searchFunction;

        try
        {
            unsafe
            {
                delegate* managed<ref T, bool> fn = &FuzzySearchFnBridge<T>.Invoke;
                return FuzzySearch(prefix, fn);
            }
        }
        finally
        {
            FuzzySearchFnBridge<T>.Current = null;
        }
    }
    
    public unsafe List<T> FuzzySearch(T prefix, delegate* managed<ref T, bool> searchFunction);
    
    public unsafe List<T> FuzzySearch(T prefix, delegate* unmanaged<ref T, bool> searchFunction);
}

file static class FuzzySearchFnBridge<T>
{
    [ThreadStatic] internal static RefPredicate<T>? Current;

    public static bool Invoke(ref T value)
    {
        RefPredicate<T>? predicate = Current;
        return predicate?.Invoke(ref value) ?? throw new InvalidOperationException("FuzzySearch function bridge not initialized.");
    }
}

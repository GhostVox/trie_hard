using System.Runtime.InteropServices;

namespace TrieHard.Unsafe;

/// <summary>
///     Will implement this as a B-Tree flattened into an array
/// </summary>
public unsafe struct UnsafeArrayTrie<T> : ITrie<UnsafeReadOnlyArray<T>, T> where T : IReadOnlyArray<TItems> where TItems : unmanaged
{
    private UIntPtr _sizeOfTItems;
    
    private UIntPtr* _data;

    private UIntPtr _capacity;
    
    public UnsafeArrayTrie(UIntPtr initialCapacity)
    {
        _sizeOfTItems = (UIntPtr)sizeof(TItems);
        _capacity = initialCapacity;
        _data = (UIntPtr*)NativeMemory.Alloc(initialCapacity, (UIntPtr)sizeof(UIntPtr));
    }

    public bool Contains(T value)
    {
        return Contains(value, 0);
    }

    private bool Contains(T value, int index)
    {
        TItems item = value[index];
        
    }

    public void Insert(T value)
    {
        throw new NotImplementedException();
    }

    public void Delete(T value)
    {
        throw new NotImplementedException();
    }

    public bool PrefixExists(T prefix)
    {
        throw new NotImplementedException();
    }

    public List<T> Complete(T prefix)
    {
        throw new NotImplementedException();
    }

    public List<T> FuzzySearch(T prefix, delegate*<ref T, bool> searchFunction)
    {
        throw new NotImplementedException();
    }

    public List<T> FuzzySearch(T prefix, delegate* unmanaged<ref T, bool> searchFunction)
    {
        throw new NotImplementedException();
    }
}
using System.Collections;
using System.Runtime.InteropServices;

namespace TrieHard.Unsafe;

public unsafe class UnsafeReadOnlyArray<T> : IReadOnlyArray<T> where T : unmanaged
{
    protected T* Data;
    
    protected readonly UIntPtr Size;

    public UnsafeReadOnlyArray(UIntPtr size)
    {
        Size = size;
        Data = (T*)NativeMemory.Alloc(size, (UIntPtr)sizeof(T));
    }

    public UnsafeReadOnlyArray(ICollection<T> collection)
        : this((UIntPtr)collection.Count)
    {
        IEnumerator<T> enumerator = collection.GetEnumerator();
        UIntPtr index = 0;
        while (enumerator.MoveNext())
        {
            Data[index++] = enumerator.Current;
        }
        
        enumerator.Dispose();
    }

    ~UnsafeReadOnlyArray()
    {
        NativeMemory.Free(Data);
    }
    
    public IEnumerator<T> GetEnumerator()
    {
        return new ArrayEnumerator(Data, Size);
    }

    IEnumerator IEnumerable.GetEnumerator()
    {
        return GetEnumerator();
    }

    public int Count => (int)Size;

    public ref T this[int index] => ref Data[index];

    public unsafe struct ArrayEnumerator(T* data, UIntPtr size) : IEnumerator<T>
    {
        private IntPtr _index = -1;

        public bool MoveNext()
        {
            _index++;
            return (UIntPtr)_index < size;
        }

        public void Reset()
        {
            _index = -1;
        }

        T IEnumerator<T>.Current => data[_index];

        object? IEnumerator.Current => data[_index];

        public void Dispose()
        {
            // Nothing to do
        }
    }

    T IIndexable<T>.this[int index] => this[index];
}
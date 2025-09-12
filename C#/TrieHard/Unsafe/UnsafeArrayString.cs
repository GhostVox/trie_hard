namespace TrieHard.Unsafe;


public unsafe class UnsafeArrayString : UnsafeReadOnlyArray<char>
{
    public UnsafeArrayString(UIntPtr size) : base(size)
    { }

    public UnsafeArrayString(ICollection<char> collection) : base(collection)
    { }

    public UnsafeArrayString(char* s) : base(SizeOfString(s) + 1)
    {
        for (UIntPtr i = 0; i < Size; i++)
        {
            Data[i] = s[i];
        }
    }

    public UnsafeArrayString(string s) : this((UIntPtr)s.Length + 1)
    {
        for (UIntPtr i = 0; i < Size; i++)
        {
            Data[i] = s[(int)i];
        }
    }

    private static UIntPtr SizeOfString(char* s)
    {
        if (s is null)
        {
            return UIntPtr.Zero;
        }
        
        UIntPtr size = UIntPtr.Zero;
        while (s[size] != 0)
        {
            size++;
        }

        return size;
    }

    public override string ToString()
    {
        return new string(Data);
    }
}
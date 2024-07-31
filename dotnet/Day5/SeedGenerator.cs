using System.Collections;

namespace Day5;

public class SeedGenerator : IEnumerator<ulong>, IEnumerable<ulong>
{
    private readonly ulong _seedStart;
    private readonly ulong _length;
    private ulong _currentOffset;

    public ulong Current => _seedStart + _currentOffset;
    object IEnumerator.Current => Current;

    public SeedGenerator(ulong seedStart, ulong length)
    {
        _seedStart = seedStart;
        _length = length;
        _currentOffset = ulong.MaxValue;
    }

    public bool MoveNext()
    {
        _currentOffset = _currentOffset == ulong.MaxValue
            ? 0
            : _currentOffset + 1;
        
        return _currentOffset < _length;
    }

    public void Reset()
    {
        _currentOffset = ulong.MaxValue;
    }

    public void Dispose()
    {

    }

    public IEnumerator<ulong> GetEnumerator() => this;
    IEnumerator IEnumerable.GetEnumerator() => this;
}

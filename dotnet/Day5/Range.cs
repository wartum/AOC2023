namespace Day5;

public class Range
{
    private ulong _sourceStart;
    private ulong _destinationStart;
    private ulong _length;

    public Range(ulong destinationStart, ulong sourceStart, ulong length)
    {
        _sourceStart = sourceStart;
        _destinationStart = destinationStart;
        _length = length;
    }

    public bool IsInRange(ulong source)
    {
        ulong offset = source - _sourceStart;
        return offset < _length;
    }

    public ulong Transform(ulong source)
    {
        ulong offset = source - _sourceStart;
        return offset < _length
            ? _destinationStart + offset
            : source;
    }
}

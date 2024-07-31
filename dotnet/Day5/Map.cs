namespace Day5;
public class Map
{
    private List<Range> _ranges = new();

    public void AddRange(Range range)
    {
        _ranges.Add(range);
    }

    public ulong Transform(ulong source)
    {
        var range = _ranges.Where(r => r.IsInRange(source)).FirstOrDefault();
        return range != null
            ? range.Transform(source)
            : source;
    }
}

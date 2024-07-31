namespace Day5;
public class InputReader
{
    private string[] _input;

    public InputReader(string[] input)
    {
        _input = input;
    }

    public Map GetMap(string mapName)
    {
        Map map = new();
        bool inMap = false;
        foreach (string line in _input)
        {
            if (inMap)
            {
                if (line.Trim() != string.Empty)
                {
                    var splits = line.Split(' ', StringSplitOptions.RemoveEmptyEntries);
                    map.AddRange(new(ulong.Parse(splits[0]), ulong.Parse(splits[1]), ulong.Parse(splits[2])));
                }
                else
                {
                    break;
                }
            }

            if (line.Trim() == ($"{mapName} map:")) inMap = true;
        }
        return map;
    }

    public IEnumerable<ulong> GetSeeds()
    {
        return _input
            .Where(l => l.StartsWith("seeds:"))
            .First()
            .Split(':').Last()
            .Split(' ', StringSplitOptions.RemoveEmptyEntries)
            .Select(s => ulong.Parse(s));
    }

    public IEnumerable<SeedGenerator> GetSeedGenerators()
    {
        var seedDescriptors = _input
            .Where(l => l.StartsWith("seeds:"))
            .First().Split(':').Last()
            .Split(' ', StringSplitOptions.RemoveEmptyEntries);
        
        string? seedStart = null;
        string? seedLength = null;
        foreach (var seedDescriptor in seedDescriptors)
        {
            if (seedStart == null)
            {
                seedStart = seedDescriptor;
            }
            else
            {
                seedLength = seedDescriptor;
                ulong p1 = ulong.Parse(seedStart);
                ulong p2 = ulong.Parse(seedLength);
                seedStart = null;
                seedLength = null;
                yield return new SeedGenerator(p1, p2);
            }

        }

    }
}

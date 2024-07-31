namespace Day5;
public class SeedToLocalizationConverter
{
    private Map[] _maps = new Map[7];

    public SeedToLocalizationConverter(InputReader inputReader)
    {
        _maps[0] = inputReader.GetMap("seed-to-soil");
        _maps[1] = inputReader.GetMap("soil-to-fertilizer");
        _maps[2] = inputReader.GetMap("fertilizer-to-water");
        _maps[3] = inputReader.GetMap("water-to-light");
        _maps[4] = inputReader.GetMap("light-to-temperature");
        _maps[5] = inputReader.GetMap("temperature-to-humidity");
        _maps[6] = inputReader.GetMap("humidity-to-location");
    }

    public ulong Transform(ulong seed)
    {
        ulong source = seed;
        ulong destination = 0;
        foreach(var map in _maps)
        {
            destination = map.Transform(source);
            source = destination;
        }
        return destination;
    }
}

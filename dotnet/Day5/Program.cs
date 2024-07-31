using System.Collections.Concurrent;

namespace Day5;
public class Program
{
    static void Main(string[] args)
    {
        const string inputName = "input.txt";
        var input = File.ReadAllLines(inputName);

        InputReader ir = new(input);
        SeedToLocalizationConverter seedToLocalizationConverter = new(ir);

        var closestSeed = ir.GetSeeds()
                            .Select(s => seedToLocalizationConverter.Transform(s))
                            .Min();
        Console.WriteLine(closestSeed);

        ConcurrentBag<ulong> minLocationsBag = new();
        Parallel.ForEach(ir.GetSeedGenerators(), gen => {
            var minLocation = gen
                .Select(s => seedToLocalizationConverter.Transform(s))
                .Min();
            minLocationsBag.Add(minLocation);
        });

        Console.WriteLine(minLocationsBag.Min());
    }
}

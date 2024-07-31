using Day3;

public class Program
{
    public static bool IsPartNumber(Number number, IEnumerable<Symbol> symbols)
    {
        foreach (Symbol symbol in symbols)
        {
            var influence = symbol.GetInfluence();
            if (influence.First().Y > number.Coords[0].Y + 1)
            {
                return false;
            }
            if (influence.Last().Y < number.Coords[0].Y - 1)
            {
                continue;
            }

            foreach (var numberCoords in number.Coords)
            {
                if (influence.Any(influenceCoords => influenceCoords == numberCoords))
                {
                    symbol.NumbersInRange.Add(number.Value);
                    return true;
                }
            }
        }
        return false;
    }

    public static void Main(string[] args)
    {
        const string inputName = "input.txt";
        var input = File.ReadAllBytes(inputName);

        InputAnalyzer analyzer = new(input);
        var symbols = analyzer.GetAllSymbols();
        var numbers = analyzer.GetAllNumbers();

        int sum = numbers
            .Where(n => IsPartNumber(n, symbols))
            .Sum(n => n.Value);

        int ratio = 0;
        foreach (var gear in symbols.Where(s => s.IsGear))
        {
            ratio += gear.NumbersInRange[0] * gear.NumbersInRange[1];
        }
        Console.WriteLine(sum);
        Console.WriteLine(ratio);
    }
}

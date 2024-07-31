using System.Text;

namespace Day3;

public class InputAnalyzer
{
    private byte[] _input;

    public InputAnalyzer(byte[] input)
    {
        _input = input;
    }

    public IEnumerable<Symbol> GetAllSymbols()
    {
        List<Symbol> symbols = new();
        int x = 0;
        int y = 1;
        foreach (byte b in _input)
        {
            x += 1;
            if (b == '\n')
            {
                x = 1;
                y += 1;
                continue;
            }
            if (b == '\r')
            {
                continue;
            }

            if (!char.IsAsciiDigit((char)b) && b != '.')
            {
                symbols.Add(new Symbol((char)b, new Point(x, y)));
            }
        }

        return symbols;
    }

    public IEnumerable<Number> GetAllNumbers()
    {
        List<Number> numbers = new();
        StringBuilder sb = new();
        int x = 0;
        int y = 1;
        foreach (byte b in _input)
        {
            x += 1;
            if (char.IsAsciiDigit((char)b))
            {
                sb.Append((char)b);
            }
            else
            {
                if (sb.Length > 0)
                {
                    int value = 0;
                    bool result = int.TryParse(sb.ToString(), out value);
                    if (!result)
                    {
                        Console.WriteLine($"Could not convert {sb.ToString()} to integer");
                    }
                    else
                    {
                        List<Point> coords = new();
                        for (int coords_x = x - sb.Length; coords_x < x; coords_x++)
                        {
                            coords.Add(new(coords_x, y));
                        }
                        numbers.Add(new(value, coords));
                        sb.Clear();
                    }
                }
            }

            if (b == '\n')
            {
                x = 1;
                y += 1;
                continue;
            }
            if (b == '\r')
            {
                continue;
            }
        }
        return numbers;
    }
}

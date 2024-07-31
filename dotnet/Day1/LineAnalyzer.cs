using System.Text;

public class LineAnalyzer
{
    private string _line;
    private int? _a;
    private int? _b;
    private StringBuilder _sb;

    public LineAnalyzer(string line)
    {
        _line = line;
        _sb = new();
        _a = null;
        _b = null;
    }

    public int? Analyze()
    {
        if (_line == null) return null;

        foreach (char c in _line)
        {
            int? val = null;
            if (char.IsDigit(c))
            {
                val = HandleDigit(c);
            }
            else
            {
                val = HandleAscii(c);
            }

            if (val.HasValue)
            {
                if (_a == null)
                {
                    _a = val.Value;
                }
                else
                {
                    _b = val.Value;
                }
            }
        }

        if (_b == null)
        {
            _b = _a;
        }

        return _a == null || _b == null
            ? null
            : Combine(_a.Value, _b.Value);
    }

    private int? HandleDigit(char c)
    {
        int val;
        _sb.Clear();

        return int.TryParse(c.ToString(), out val)
            ? val
            : null;
    }

    private int? HandleAscii(char c)
    {
        _sb.Append(c);
        string currentString = _sb.ToString();
        foreach (string key in _numbersMapping.Keys)
        {
            if (currentString.Contains(key))
            {
                char lastCharacter = currentString.ElementAt(_sb.Length - 1);
                _sb.Clear();
                _sb.Append(lastCharacter);
                return _numbersMapping[key];
            }
        }
        return null;
    }

    private static readonly Dictionary<string, int> _numbersMapping = new()
    {
        {"zero",  0},
        {"one",   1},
        {"two",   2},
        {"three", 3},
        {"four",  4},
        {"five",  5},
        {"six",   6},
        {"seven", 7},
        {"eight", 8},
        {"nine",  9},
    };

    private static int Combine(int a, int b)
    {
        return a * 10 + b;
    }
}
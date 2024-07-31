namespace Day3;

public class Symbol
{
    public bool IsGear { get => _symbol == '*' && NumbersInRange.Count == 2; }
    public List<int> NumbersInRange { get; set; } = new();

    private char _symbol;
    private Point _coords;
    private int X { get => _coords.X; }
    private int Y { get => _coords.Y; }

    public Symbol(char c, Point coords)
    {
        _symbol = c;
        _coords = coords;
    }

    public IEnumerable<Point> GetInfluence()
    {
        return new List<Point>()
            {
                new(X - 1, Y - 1), new(X, Y - 1), new(X + 1, Y - 1),
                new(X - 1, Y),                    new(X + 1, Y),
                new(X - 1, Y + 1), new(X, Y + 1), new(X + 1, Y + 1),
            };
    }

    override public string ToString()
    {
        return $"{_symbol} - [{X}, {Y}]";
    }

    public static bool operator ==(Symbol a, Symbol b)
    {
        return a._symbol == b._symbol
            && a.X == b.X
            && a.Y == b.Y;
    }

    public static bool operator !=(Symbol a, Symbol b)
    {
        return a._symbol == b._symbol
            && a.X == b.X
            && a.Y == b.Y;
    }
}

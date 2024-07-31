namespace Day4;
public class Card
{
    public int Id { get; }
    public IEnumerable<int> WinningNumbers;
    public IEnumerable<int> MyNumbers;
    public int N { get => WinningNumbers.Where(w => MyNumbers.Contains(w)).Count(); }

    public Card(string line)
    {
        Id = int.Parse(line.Split(':').First().Split(' ', StringSplitOptions.RemoveEmptyEntries).Last().Trim());
        WinningNumbers = line
            .Split(':').Last()
            .Split('|').First()
            .Split(' ', StringSplitOptions.RemoveEmptyEntries)
            .Select(str => int.Parse(str))
            .ToArray();
        MyNumbers = line
            .Split(':').Last()
            .Split('|').Last()
            .Split(' ', StringSplitOptions.RemoveEmptyEntries)
            .Select(str => int.Parse(str))
            .ToArray();
    }

    public int CountScore()
    {
        int score = 0;
        for (int i = 0; i < N; i++)
        {
            score = score == 0
                ? 1
                : score * 2;
        }
        return score;
    }

    override public string ToString()
    {
        string res = $"{{{Id}: [";
        foreach (var x in WinningNumbers)
        {
            res += $" {x}";
        }
        res += " ] [";
        foreach (var x in MyNumbers)
        {
            res += $" {x}";
        }
        res += " ]";
        return res;
    }
}

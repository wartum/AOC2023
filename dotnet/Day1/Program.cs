const string InputName = "input.txt";

int sum = 0;
foreach (var line in File.ReadAllLines(InputName))
{
    LineAnalyzer analyzer = new(line);
    int? val = analyzer.Analyze();
    if (val.HasValue)
    {
        sum += val.Value;
    }
}

Console.WriteLine(sum);
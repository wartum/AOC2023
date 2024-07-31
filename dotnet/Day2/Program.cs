const string inputName = "input.txt";
var input = File.ReadAllLines(inputName);

var sum = input.AsEnumerable()
    .Where(l => l.Length > 0)
    .Select(l => new Game(l))
    .Where(g => g.cubes[Cube.Red] <= 12 && g.cubes[Cube.Green] <= 13 && g.cubes[Cube.Blue] <= 14)
    .Sum(g => g.Id);
Console.WriteLine(sum);

var allGames = input.AsEnumerable()
    .Where(l => l.Length > 0)
    .Select(l => new Game(l));

int power = 0;
foreach (var game in allGames)
{
    int val = 0;
    foreach (var cube in game.cubes)
    {
        val = val == 0
            ? cube.Value
            : val * cube.Value;
    }
    power += val;
}
Console.WriteLine(power);
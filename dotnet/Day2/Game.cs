public class Game
{
    public int Id { get; private set; }
    public Dictionary<Cube, int> cubes;

    public Game(string line)
    {
        string idAsString = line.Split(':')[0].Split(' ')[1];
        Id = int.Parse(idAsString);

        cubes = new();
        var cubeDescriptors =
            line.Split(':')[1]
            .Replace(';', ',')
            .Split(',');

        foreach(var desc in cubeDescriptors)
        {
            var split = desc.Trim().Split(' ');
            var cube = CubeFactory.From(split[1]);
            if (!cubes.ContainsKey(cube))
            {
                cubes[cube] = int.Parse(split[0]);
            }
            else
            {
                var newCubeNumber = int.Parse(split[0]);
                var currentCubeNumber = cubes[cube];
                if (newCubeNumber > currentCubeNumber)
                {
                    cubes[cube] = newCubeNumber;
                }
            }
        }
    }

    override public string ToString()
    {
        string res = $"Id: {Id}";
        foreach(var cube in cubes)
            res += $" [{cube.Key} = {cube.Value}]";
        return res;
    }
}
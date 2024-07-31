public enum Cube
{
    Red,
    Green,
    Blue,
    Unknown
}

public static class CubeFactory
{
    public static Cube From(string str) => str switch
    {
        "red" => Cube.Red,
        "green" => Cube.Green,
        "blue" => Cube.Blue,
        _ => Cube.Unknown
    };
}

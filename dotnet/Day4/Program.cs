namespace Day4;

public class Program
{
    static void Main(string[] args)
    {
        const string inputName = "input.txt";
        var input = File.ReadAllLines(inputName);
        var score = input
            .Select(line => new Card(line).CountScore())
            .Sum();

        CardGameMachine machine = new(input.Select(line => new Card(line)));
        Console.WriteLine(machine.Play());
    }
}

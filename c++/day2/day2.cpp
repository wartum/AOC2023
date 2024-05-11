#include <iostream>
#include <fstream>
#include <string>
#include <map>
#include <vector>

using namespace std;

enum class Cube
{
    UNDEFINED,
    RED,
    BLUE,
    GREEN
};

Cube string2cube(const string& as_string)
{
    if (as_string == "red")
        return Cube::RED;
    if (as_string == "blue")
        return Cube::BLUE;
    if (as_string == "green")
        return Cube::GREEN;

    return Cube::UNDEFINED;
}

struct Game
{
    int id;
    map<Cube, int> cubes;
};

inline bool is_digit(char c)
{
    return (c >= 48 && c <= 57);
}

vector<Game> parse_input(ifstream& file)
{
    vector<Game> games;
    Game game {
        .id = 0,
    };
    while (!file.eof())
    {
        string token;
        file >> token;

        if (token == "Game")
        {
            if (game.id != 0)
            {
                games.push_back(game);
                game = Game();
            }

            file >> token;
            game.id = stoi(token.substr(0, token.length() - 1));
        }
        else if (is_digit(token[0]))
        {
            int cubes_number = stoi(token);
            file >> token;
            char last_char = token[token.length()-1];
            if (last_char == ',' || last_char == ';')
            {
                token = token.substr(0, token.length() - 1);
            }

            Cube cube = string2cube(token);
            if (game.cubes[cube] < cubes_number)
            {
                game.cubes[cube] = cubes_number;
            }
        }
    }
    games.push_back(game);
    return games;
};

int main() {
    ifstream file("input.txt", ios::in);
    if (!file.is_open())
    {
        cerr << "Cannot open input\n";
        return 1;
    }

    vector<Game> games = parse_input(file);
    file.close();

    int sum = 0;
    int power = 0;
    for (Game& game : games)
    {
        if (game.cubes[Cube::RED] <= 12 &&
            game.cubes[Cube::GREEN] <= 13 &&
            game.cubes[Cube::BLUE] <= 14)
        {
            sum += game.id;
        }

        power += (game.cubes[Cube::RED] * game.cubes[Cube::GREEN] * game.cubes[Cube::BLUE]);
    }
    cout << sum << "\n" << power << "\n";
    return 0;
}

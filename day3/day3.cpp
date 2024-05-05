#include <iostream>
#include <fstream>
#include <string>
#include <sstream>
#include <vector>
#include <array>

using namespace std;

struct Point
{
    int x, y;
};

struct SymbolInfluence
{
    SymbolInfluence(int x, int y, bool is_gear) :
        isGear(is_gear),
        partsConnected(vector<int>()),
        points({
            Point{.x = x-1, .y = y-1},
            Point{.x = x,   .y = y-1},
            Point{.x = x+1, .y = y-1},

            Point{.x = x-1, .y = y},
            Point{.x = x+1, .y = y},

            Point{.x = x-1, .y = y+1},
            Point{.x = x,   .y = y+1},
            Point{.x = x+1, .y = y+1},
                })
    {}

    bool isGear;
    vector<int> partsConnected;
    array<Point, 8> points;
};

bool is_digit(char c)
{
    return c >= '0' && c <= '9';
}

bool is_symbol(char c)
{
    return is_digit(c) == false && c != '.';
}

int main()
{
    vector<SymbolInfluence> influences;
    fstream file("input.txt", ios::in);
    if (!file.is_open()) {
        cerr << "Cannot open file\n";
        exit(1);
    }

    int x, y = 0;
    string line;
    while (!file.eof()) {
        getline(file, line);
        x = 0;
        y += 1;
        for (char c : line) {
            x += 1;
            if (is_symbol(c)) {
                influences.push_back(SymbolInfluence(x, y, c == '*'));
            }
        }
    }
    file.close();

    int sum = 0;
    x = 0;
    y = 0;
    file.open("input.txt", ios::in);
    while (!file.eof()) {
        stringstream ss;
        bool is_part = false;
        SymbolInfluence* current_influence = nullptr;

        getline(file, line);
        y += 1;
        x = 0;
        for (char c : line) {
            x += 1;
            if (is_digit(c)) {
                ss << c;
                if (!is_part) {
                    for (auto& inf : influences) {
                        for (auto& p : inf.points) {
                            if (p.x == x && p.y == y) {
                                is_part = true;
                                current_influence = &inf;
                            }
                        }
                    }
                }
            } else if (ss.str().length() > 0) {
                if (is_part) {
                    int digit = stoi(ss.str());
                    sum += digit;
                    if (current_influence->isGear) {
                        current_influence->partsConnected.push_back(digit);
                    }
                }
                is_part = false;
                current_influence = nullptr;
                ss = stringstream();
            }
        }

        if (is_part) {
            int digit = stoi(ss.str());
            sum += digit;
            if (current_influence->isGear) {
                current_influence->partsConnected.push_back(digit);
            }
        }
    }

    int ratios = 0;
    for (auto& inf : influences) {
        if (inf.isGear && inf.partsConnected.size() == 2) {
            ratios += inf.partsConnected[0] * inf.partsConnected[1];
        }
    }

    cout << sum << "\n" << ratios << "\n";

    return 0;
}

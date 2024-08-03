#include <algorithm>
#include <array>
#include <cstdlib>
#include <fstream>
#include <iostream>
#include <sstream>
#include <string>
#include <vector>

#include "hand.h"

using namespace std;

vector<string>
readAllFileLines(string path)
{
    vector<string> input;
    ifstream file(path);
    while (!file.eof()) {
        input.push_back(string());
        getline(file, input.back());
    }
    return input;
}

Hand
lineToHand(string &line)
{
    stringstream ss(line);
    string hand, bid;
    ss >> hand;
    ss >> bid;
    array<CardLabel, 5> hand_array;
    for (int i = 0; i < 5; i++) {
        hand_array[i] = charToCardLabel(hand[i]);
    }

    return Hand(hand_array, atoi(bid.c_str()));
}

int
main()
{
    const string input_name = "input.txt";
    vector<Hand> hands;
    for (auto& line : readAllFileLines(input_name)) {
        if (line.size() > 0) {
            hands.push_back(lineToHand(line));
        }
    }

    sort(hands.begin(), hands.end());

    unsigned long score = 0;
    for (int i = 0; i < hands.size(); i++) {
        int rank = i + 1;
        score += rank * hands[i].bid;
    }
    cout << score << '\n';

    return 0;
}


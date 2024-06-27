#include <algorithm>
#include <iostream>
#include <fstream>
#include <sstream>
#include <string>
#include <chrono>

#include <vector>
#include <array>
#include <map>

using namespace std;

const int NUMBERS_WIN = 10;
const int NUMBERS_MINE = 25;
const int NUMBERS_ALL = NUMBERS_WIN + NUMBERS_MINE;

string input_name = "input.txt";
int max_card_number = 0;

map<int, array<string, NUMBERS_ALL>> numbers_map;
map<int, int> cached_hits;
vector<int> cards[2];
vector<int>* current_cards = &(cards[0]);
vector<int>* another_cards = &(cards[1]);


int count_points(int hits)
{
    if (hits == 0) {
        return 0;
    }

    if (hits == 1) {
        return 1;
    }

    int points = 1;
    for (int i = 1; i < hits; i++) {
        points *= 2;
    }
    return points;
}

vector<int> next_cards(int starting_card, int hits) {
    vector<int> cards;
    for (int i = 1; i <= hits; i++) {
        const int new_card = starting_card + i;
        if (new_card <= max_card_number) {
            cards.push_back(new_card);
        }
    }

    return cards;
}

int main(int argc, char** argv)
{
    if (argc == 2) {
        input_name = argv[1];
    }
    int points = 0;
    ifstream file(input_name);
    string line;
    string word;
    while (!file.eof()) {
        getline(file, line);
        if (line.length() == 0) {
            break;
        }

        stringstream ss(line);
        ss >> word >> word;
        int id = stoi(word.substr(0, word.length() - 1));
        array<string, NUMBERS_ALL> nums;
        auto it = nums.begin();
        while (!ss.eof()) {
            ss >> word;
            if (word != "|") {
                *it = word;
                it += 1;
            }
        }
        numbers_map[id] = nums;
        current_cards->push_back(id);
    }

    for (int c : *current_cards) {
        const auto& arr = numbers_map[c];
        auto w_beg = arr.begin();
        auto w_end = arr.begin() + NUMBERS_WIN;
        auto m_beg = w_end;
        auto m_end = arr.end();

        auto hits = count_if(w_beg, w_end, [&](const string& a){
                    return any_of(m_beg, m_end, [&](const string& b){ return a == b; });
                });
        cached_hits[c] = hits;
        points += count_points(hits);
    }
    cout << points << "\n";

    max_card_number = current_cards->size();
    int total = 0;
    bool quit = false;
    while (!quit) {
        total += current_cards->size();
        for (int c : *current_cards) {
            auto hits = cached_hits[c];
            for(int nc : next_cards(c, hits)) {
                another_cards->push_back(nc);
            }
        }

        if (another_cards->size() == 0) {
            quit = true;
        } else {
            auto tmp = current_cards;
            current_cards = another_cards;
            another_cards = tmp;
            another_cards->clear();
        }
    }
    cout << total << "\n";

    return 0;
}

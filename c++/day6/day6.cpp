#include <cstdlib>
#include <string>
#include <iostream>
#include <fstream>
#include <sstream>
#include <vector>
#include <algorithm>

using namespace std;

const string INPUT_NAME = "input.txt";

struct RaceDetails
{
    long time;
    long distance;

    int count_winning_strategies() {
        int count = 0;
        for (long i = 0; i < time; i++) {
            long d = calculate_distance(i);
            if (d > distance) {
                count += 1;
            }
        }
        return count;
    }

private:
    long calculate_distance(long pressed_time) {
        long time_left = time - pressed_time;
        return time_left * pressed_time;
    }
};

bool is_digit(char c) {
    return c >= '0' && c <= '9';
}

bool is_not_digit(char c) {
    return !is_digit(c);
}

RaceDetails get_race_details(ifstream &file) {
    string time_line;
    string distance_line;

    while (!file.eof()) {
        string line;
        getline(file, line);
        if (line.find("Time:") != string::npos) {
            time_line = std::move(line);
        } else if (line.find("Distance:") != string::npos) {
            distance_line = std::move(line);
        }
    }

    time_line.erase(
            remove_if(time_line.begin(), time_line.end(), is_not_digit)
            , time_line.end());
    distance_line.erase(
            remove_if(distance_line.begin(), distance_line.end(), is_not_digit)
            , distance_line.end());

    return RaceDetails{
        .time = atoi(time_line.c_str()),
        .distance = atol(distance_line.c_str())
    };
}

vector<RaceDetails> get_races_details(ifstream &file) {
    stringstream times;
    stringstream distances;

    while (!file.eof()) {
        string line;
        getline(file, line);
        if (line.find("Time:") != string::npos) {
            times.str(line);
        } else if (line.find("Distance:") != string::npos) {
            distances.str(line);
        }
    }
    string buffer;
    times >> buffer;
    distances >> buffer;

    vector<RaceDetails> races;
    while (!times.eof() || !distances.eof()) {
        times >> buffer;
        int time = atoi(buffer.c_str());
        distances >> buffer;
        int distance = atol(buffer.c_str());
        races.push_back(RaceDetails {
                    .time = time,
                    .distance = distance
                });
    }

    return races;
}


int main() {
    ifstream file(INPUT_NAME);
    vector<RaceDetails> races_details = get_races_details(file);
    file.close();

    int output = 0;
    for (auto &r : races_details) {
        int count_ws = r.count_winning_strategies();
        output = output == 0
            ? count_ws
            : output * count_ws;
    }

    cout << output << "\n";

    file.open(INPUT_NAME);
    RaceDetails race_details = get_race_details(file);
    file.close();

    cout << race_details.count_winning_strategies() << "\n";
    return 0;
}

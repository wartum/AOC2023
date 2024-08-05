#include <climits>
#include <iostream>
#include <numeric>
#include <sstream>
#include <algorithm>
#include <string>
#include <vector>
#include <optional>
#include <fstream>

using namespace std;

struct Region
{
    string name;
    Region* left;
    Region* right;

    Region(string name) : Region(name, nullptr, nullptr) { }
    Region(string name, Region* left, Region* right) :
        name(name),
        left(left),
        right(right)
    { }
};

ostream&
operator<<(ostream &os, Region &region)
{
    os << region.name <<": ";
    if (region.left == nullptr) {
        os << "left=NULL";
    } else {
        os << "left=" << region.left->name;
    }
    os << " ";
    if (region.right == nullptr) {
        os << "right=NULL";
    } else {
        os << "right=" << region.right->name;
    }

    return os;
}

optional<Region>
newRegionFromLine(string &line)
{
    if (line.find(" = ") == string::npos){
        return {};
    }

    stringstream ss(line);
    string region_name;
    ss >> region_name;
    return Region(region_name);
}

void
linkAllRegions(vector<Region> &regions, vector<string> &input)
{
    for (auto &line : input) {
        if (line.find(" = ") == string::npos){
            continue;
        }

        stringstream ss(line);
        string buffer;
        ss >> buffer;
        auto it_tgt_region = find_if(regions.begin(), regions.end(),
                [&](Region region){return region.name == buffer;});
        ss >> buffer >> buffer;
        buffer = buffer.substr(1, 3);
        auto it_lft_region = find_if(regions.begin(), regions.end(),
                [&](Region region){return region.name == buffer;});
        ss >> buffer;
        buffer = buffer.substr(0, 3);
        auto it_rgt_region = find_if(regions.begin(), regions.end(),
                [&](Region region){return region.name == buffer;});

        it_tgt_region->left = it_lft_region.base();
        it_tgt_region->right = it_rgt_region.base();
    }
}

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

optional<string>
getDirections(vector<string> &input)
{
    for (auto &line : input) {
        if (line.size() > 0 &&
            line.find(" = ") == string::npos &&
            (line.find('L') != string::npos || line.find('R') != string::npos))
        {
            return line;
        }
    }

    return {};
}

int
travel(string &directions, Region *first_region)
{
    int steps = 0;
    auto current_region = first_region;
    while (current_region->name != "ZZZ") {
        char direction = directions[steps % directions.size()];
        if (direction == 'L') {
            current_region = current_region->left;
        } else {
            current_region = current_region->right;
        }
        steps += 1;
    }
    return steps;
}

int
travelAsGhost(string &directions, Region *first_region)
{
    int steps = 0;
    auto current_region = first_region;
    while (current_region->name[2] != 'Z') {
        char direction = directions[steps % directions.size()];
        if (direction == 'L') {
            current_region = current_region->left;
        } else {
            current_region = current_region->right;
        }
        steps += 1;
    }
    return steps;
}

unsigned long
lcm(unsigned long a, unsigned long b) {
    return (a / gcd(a, b)) * b;
}

unsigned long long
lcm(const std::vector<int>& nums) {
    return std::accumulate(nums.begin(), nums.end(), 1UL,
            [](unsigned long a, unsigned long b) { return lcm(a, static_cast<unsigned long long>(b)); });
}

int
main()
{
    auto input = readAllFileLines("input.txt");
    string directions;
    vector<Region> regions;
    regions.reserve(input.size());

    auto directions_maybe = getDirections(input);
    if (!directions_maybe.has_value()) {
        cerr << "No directions!\n";
        return -1;
    } else {
        directions = directions_maybe.value();
    }

    for (auto &line : input) {
        auto region_maybe = newRegionFromLine(line);
        if (region_maybe.has_value()) {
            regions.push_back(region_maybe.value());
        }
    }
    linkAllRegions(regions, input);

    auto it_first_region = find_if(regions.begin(), regions.end(), 
            [](Region region){ return region.name == "AAA"; });
    unsigned long steps = travel(directions, it_first_region.base());
    cout << steps << endl;

    vector<int> steps_vec;
    for (auto &region : regions) {
        if (region.name[2] == 'A') {
            steps_vec.push_back(travelAsGhost(directions, &region));
        }
    }
    cout << lcm(steps_vec) << "\n";

    return 0;
}

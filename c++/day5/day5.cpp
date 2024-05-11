#include <climits>
#include <cstdlib>
#include <stdint.h>

#include <sstream>
#include <iostream>
#include <fstream>

#include <optional>
#include <vector>
#include <array>
#include <tuple>

#include <algorithm>
#include <future>

using namespace std;

const string INPUT_NAME = "input.txt";

struct Range
{
    uint32_t beg_to, beg_from, len;

    Range() :
        beg_to(0),
        beg_from(0),
        len(0)
    {}
    Range(uint32_t to, uint32_t from, uint32_t l) :
        beg_to(to),
        beg_from(from),
        len(l)
    {}

    inline bool is_from_in_range(uint32_t from)
    {
        return from >= beg_from && from <= beg_from + len;
    }

    inline uint32_t get_to(uint32_t from)
    {
        const uint32_t offset = from - beg_from;
        return beg_to + offset;
    }

    friend ostream& operator<<(ostream& os, const Range& range)
    {
        os << "[ " << range.beg_to << ", " << range.beg_from << ", " << range.len << " ]";
        return os;
    }
};

struct Mapping
{
    vector<Range> ranges;

    Mapping() { ranges.reserve(30); }

    uint32_t get_to(uint32_t a)
    {
        for (auto& r : ranges) {
            if (r.is_from_in_range(a)) {
                return r.get_to(a);
            }
        }
        return a;
    }

    friend ostream& operator<<(ostream& os, const Mapping& mapping)
    {
        os << "MAPPING - " << mapping.ranges.size() << ":\n";
        for (auto& r : mapping.ranges) {
            os << r << "\n";
        }
        return os;
    }
};

struct SeedGenerator
{
    int id;
    Range range;
    uint32_t current_offset;

    SeedGenerator(int id) :
        id(id),
        current_offset(0)
    {}

    optional<uint32_t> next_seed()
    {
        if (current_offset >= range.len) {
            return optional<uint32_t>();
        }

        if (current_offset % 1000000 == 0) {
            cout << "Seed Generator[" << id << "]: Current Offset: " << current_offset << "\n";
        }

        auto seed = range.beg_from + current_offset;
        current_offset += 1;
        return seed;
    }
};

tuple<vector<SeedGenerator>, array<Mapping, 7>>
parse_input(ifstream &file)
{
    vector<SeedGenerator> generators;
    int gen_id = 0;
    array<Mapping, 7> mappings;
    int current_map_index = -1;

    string line;
    while (!file.eof()) {
        getline(file, line);
        if (line.length() == 0) {
            continue;
        }

        stringstream ss(line);
        string word;

        ss >> word;
        if (word == "seeds:") {
            while (!ss.eof()) {
                ss >> word;
                auto beg = stoul(word);
                ss >> word;
                auto len = stoul(word);

                SeedGenerator gen(gen_id++);
                gen.range = Range(0, beg, len);
                generators.push_back(gen);
            }
        } else
        if (word.find("-to-") != string::npos) {
            current_map_index += 1;
        } else {
            auto beg_a = stoul(word);
            ss >> word;
            auto beg_b = stoul(word);
            ss >> word;
            auto len = stoul(word);
            mappings[current_map_index].ranges.push_back(Range(beg_a, beg_b, len));
        }
    }
    return make_tuple(generators, mappings);
}

int main()
{
    ifstream file(INPUT_NAME, ios::in);
    if (!file.is_open()) {
        cerr << "Could not open input file\n";
        exit(1);
    }

    auto [seed_gens, mappings] = parse_input(file);
    file.close();

    auto run = [&mappings](SeedGenerator* gen) {
                uint32_t min_location = UINT_MAX;
                auto seed = gen->next_seed();
                while (seed.has_value()) {
                    auto mapped_value = seed.value();
                    for (auto& m : mappings) {
                        mapped_value = m.get_to(mapped_value);
                    }
                    min_location = min(mapped_value, min_location);
                    seed = gen->next_seed();
                }
                return min_location;
            };

    vector<future<uint32_t>> futures;
    vector<uint32_t> locations;
    for (auto& g : seed_gens) {
        futures.push_back(async(launch::async, run, &g));
    }
    for (auto& f : futures) {
        locations.push_back(f.get());
    }

    auto min = ranges::min_element(locations);
    cout << *min << endl;

    return 0;
}

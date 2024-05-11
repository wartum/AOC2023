#include <iostream>
#include <fstream>
#include <string>
#include <map>
#include <optional>

using namespace std;

const map<int, string> digits {
    { 1, "one"   },
    { 2, "two"   },
    { 3, "three" },
    { 4, "four"  },
    { 5, "five"  },
    { 6, "six"   },
    { 7, "seven" },
    { 8, "eight" },
    { 9, "nine"  },
};

inline bool is_digit(char c)
{
    return (c >= 48) && (c <=57);
}

inline int to_digit(char c)
{
    return c - 48;
}

inline int combine(int a, int b)
{
    return a*10 + b;
}

class DigitConverter 
{
private:
    string buffer;
    char last_char;
public:
    void clear()
    {
        buffer.clear();
    }

    void add_char(char c)
    {
        buffer += c;
        last_char = c;
    }

    optional<int> todigit()
    {
        if (is_digit(last_char))
        {
            buffer.clear();
            return make_optional(to_digit(last_char));
        }
        else
        {
            for (auto& digit : digits)
            {
                if (buffer.find(digit.second) != string::npos)
                {
                    buffer.clear();
                    buffer += last_char;
                    return make_optional(digit.first);
                }
            }
        }
        return optional<int>();
    }
};

int main()
{
    fstream file("input.txt", ios::in);
    if (!file.is_open())
    {
        cerr << "Cannot open file\n";
        return 1;
    }

    int sum = 0;
    optional<int> a;
    optional<int> b;
    string line;
    while (!file.eof())
    {
        std::getline(file, line);
        if (line.empty())
            continue;

        DigitConverter converter;
        for (char c : line)
        {
            converter.add_char(c);
            auto d = converter.todigit();
            if (d.has_value())
            {
                if (!a.has_value())
                {
                    a = make_optional(d.value());
                }
                else
                {
                    b = make_optional(d.value());
                }
            }
        }

        if (!b.has_value())
            b = make_optional(a.value());

        sum += combine(a.value(), b.value());
        a.reset();
        b.reset();
    }

    cout << sum << "\n";
    file.close();
    return 0;
}

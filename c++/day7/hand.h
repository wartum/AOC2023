#pragma once

#include <ostream>
#include <array>
#include <map>

enum CardLabel
{
    CL_J = 1,
    CL_2 = 2,
    CL_3,
    CL_4,
    CL_5,
    CL_6,
    CL_7,
    CL_8,
    CL_9,
    CL_T,
    CL_Q,
    CL_K,
    CL_A,
};

CardLabel charToCardLabel(const char c);

enum HandType
{
    HighCard,
    OnePair,
    TwoPair,
    ThreeKind,
    FullHouse,
    FourKind,
    FiveKind,
};

std::ostream& operator<<(std::ostream &os, const HandType &hand_type);

struct Hand
{
    std::array<CardLabel, 5> hand;
    HandType hand_type;
    int bid;

    Hand(std::array<CardLabel, 5> hand, int bid);
    bool operator<(const Hand &other) const;
private:
    HandType checkHandType();
    HandType cardMapToHandType(std::map<CardLabel, int> &cardMap);
};

std::ostream& operator<<(std::ostream &os, const Hand &hand);

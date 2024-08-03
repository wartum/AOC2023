#include "hand.h"

#include <algorithm>
#include <utility>
#include <iostream>

using namespace std;

CardLabel
charToCardLabel(const char c)
{
    switch (c) {
        case '2': return CL_2;
        case '3': return CL_3;
        case '4': return CL_4;
        case '5': return CL_5;
        case '6': return CL_6;
        case '7': return CL_7;
        case '8': return CL_8;
        case '9': return CL_9;
        case 'T': return CL_T;
        case 'J': return CL_J;
        case 'Q': return CL_Q;
        case 'K': return CL_K;
        case 'A': return CL_A;
        default: return CL_2;
    }
}

std::ostream& operator<<(std::ostream &os, const HandType &hand_type)
{
    switch(hand_type)
    {
    case HighCard: os << "HighCard"; break;
    case OnePair: os << "OnePair"; break;
    case TwoPair: os << "TwoPair"; break;
    case ThreeKind: os << "ThreeKind"; break;
    case FullHouse: os << "FullHouse"; break;
    case FourKind: os << "FourKind"; break;
    case FiveKind: os << "FiveKind"; break;
    }
    return os;
}

Hand::Hand(array<CardLabel, 5> hand, int bid) :
    hand(hand),
    hand_type(checkHandType()),
    bid(bid)
{ }

bool
Hand::operator<(const Hand &other) const
{
    if (hand_type < other.hand_type) {
        return true;
    }

    if (hand_type > other.hand_type) {
        return false;
    }

    for (int i = 0; i < 5; i++) {
        if (hand[i] < other.hand[i]) {
            return true;
        }
        if (hand[i] > other.hand[i]) {
            return false;
        }
    }

    return false;
}

std::ostream&
operator<<(std::ostream& os, const Hand &hand)
{
    os << hand.hand_type << ":\t";
    for (auto card : hand.hand)
        os << card << " ";
    return os;
}

HandType
Hand::checkHandType()
{
    map<CardLabel, int> card_count;
    for (auto card : hand) {
        if (card_count.find(card) == card_count.end()) {
            card_count[card] = 1;
        } else {
            card_count[card] += 1;
        }
    }

    if (card_count.size() > 1 && card_count.count(CL_J) > 0) {
        auto me = max_element(card_count.begin(), card_count.end(),
                [](auto a, auto b)
                {
                    return a.first == CL_J
                        ? true
                        : a.second < b.second;
                });
        auto joker_count = card_count[CL_J];
        card_count[me->first] += joker_count;
        card_count.erase(CL_J);
    }

    return cardMapToHandType(card_count);
}

HandType
Hand::cardMapToHandType(map<CardLabel, int> &cardMap)
{
    if (std::any_of(cardMap.begin(), cardMap.end(), [](const pair<CardLabel, int> p) { return p.second == 5;}))
    {
        return HandType::FiveKind;
    }
    if (std::any_of(cardMap.begin(), cardMap.end(), [](const pair<CardLabel, int> p) { return p.second == 4;}))
    {
        return HandType::FourKind;
    }
    if (std::any_of(cardMap.begin(), cardMap.end(), [](const pair<CardLabel, int> p) { return p.second == 3;}) &&
        std::any_of(cardMap.begin(), cardMap.end(), [](const pair<CardLabel, int> p) { return p.second == 2;}))
    {
        return HandType::FullHouse;
    }
    if (std::any_of(cardMap.begin(), cardMap.end(), [](const pair<CardLabel, int> p) { return p.second == 3;}) &&
        std::count_if(cardMap.begin(), cardMap.end(), [](const pair<CardLabel, int> p) { return p.second == 1;}) == 2)
    {
        return HandType::ThreeKind;
    }
    if (std::count_if(cardMap.begin(), cardMap.end(), [](const pair<CardLabel, int> p) { return p.second == 2;}) == 2 &&
        std::any_of(cardMap.begin(), cardMap.end(), [](const pair<CardLabel, int> p) { return p.second == 1;}))
    {
        return HandType::TwoPair;
    }
    if (std::count_if(cardMap.begin(), cardMap.end(), [](const pair<CardLabel, int> p) { return p.second == 1;}) == 3 &&
        std::any_of(cardMap.begin(), cardMap.end(), [](const pair<CardLabel, int> p) { return p.second == 2;}))
    {
        return HandType::OnePair;
    }
    return HandType::HighCard;
}

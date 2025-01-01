//
// Created by nikola on 12/31/24.
//

#ifndef CPP_DAY07_H
#define CPP_DAY07_H


#include "Day.h"

using ull = unsigned long long;

struct Equation {
    ull left;
    std::vector<ull> right;
};


class Day07 : public Day {
public:
    void PreprocessData() override;

    void SolvePart1() override;

    void SolvePart2() override;

    bool CanSolve1(const Equation &equation, int pos = 0, ull res = 0) const;

    bool CanSolve2(const Equation &equation, int pos = 0, ull res = 0) const;

    std::vector<Equation> equations;
};


#endif //CPP_DAY07_H

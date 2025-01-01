//
// Created by nikola on 1/1/25.
//

#ifndef CPP_DAY08_H
#define CPP_DAY08_H


#include "Day.h"

class Day08 : public Day {
public:
    void PreprocessData() override;

    void SolvePart1() override;

    void SolvePart2() override;

    bool IsWithinBounds(int x, int y) const;

};


#endif //CPP_DAY08_H

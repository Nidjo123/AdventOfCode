#ifndef CPP_DAY06_H
#define CPP_DAY06_H


#include "Day.h"


struct Position {
    int x;
    int y;
};

class Day06 : public Day {
public:
    void PreprocessData() override;

    void SolvePart1() override;

    void SolvePart2() override;

    Position GetStartPosition() const;

    bool IsPositionInsideMap(const Position &) const;
};


#endif //CPP_DAY06_H

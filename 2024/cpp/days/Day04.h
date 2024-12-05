#ifndef DAY04_H
#define DAY04_H
#include "Day.h"


class Day04 : public Day {
public:
    void SolvePart1() override;

    void SolvePart2() override;

private:
    int search_word(int x, int y, int dir, const std::string &word, int depth = 0);
};


#endif //DAY04_H

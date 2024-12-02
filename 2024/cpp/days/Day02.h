#ifndef DAY02_H
#define DAY02_H
#include "Day.h"

using Report = std::vector<int>;

class Day02 : public Day {
public:
    void SolvePart1() override;

    void SolvePart2() override;

    void PreprocessData() override;

private:
    std::vector<Report> reports;
};


#endif //DAY02_H

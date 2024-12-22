#ifndef CPP_DAY05_H
#define CPP_DAY05_H


#include "Day.h"
#include <vector>

class Day05 : public Day {
public:
    void PreprocessData() override;

    void SolvePart1() override;

    void SolvePart2() override;

private:
    struct Constraint {
        int before;
        int after;
    };

    std::vector<Constraint> constraints;
    std::vector<std::vector<int>> print_jobs;
};

#endif //CPP_DAY05_H

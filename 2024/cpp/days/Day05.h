#ifndef CPP_DAY05_H
#define CPP_DAY05_H


#include "Day.h"
#include <vector>
#include <set>
#include <map>

class Day05 : public Day {
public:
    void PreprocessData() override;

    void SolvePart1() override;

    void SolvePart2() override;

private:
    using PrintJob = std::vector<int>;

    PrintJob GetSortedJob(const PrintJob &) const;

    std::map<int, std::set<int>> constraints;
    std::vector<PrintJob> print_jobs;
};

#endif //CPP_DAY05_H

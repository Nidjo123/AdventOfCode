#ifndef DAY_H
#define DAY_H
#include <string>
#include <vector>


class Day {
public:
    void SetData(const std::vector<std::string> &lines);

    virtual void PreprocessData();

    virtual void SolvePart1() = 0;

    virtual void SolvePart2() = 0;

    void Solve();

protected:
    std::vector<std::string> lines;
};


#endif //DAY_H

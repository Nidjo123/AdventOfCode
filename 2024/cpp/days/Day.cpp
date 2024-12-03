#include "Day.h"

void Day::SetData(const std::vector<std::string> &lines) {
    this->lines = lines;
    PreprocessData();
}

void Day::PreprocessData() {
}

void Day::Solve() {
    SolvePart1();
    SolvePart2();
}
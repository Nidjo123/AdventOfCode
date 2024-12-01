#include <iostream>
#include "Day01.h"
#include "util/Util.h"

int main() {
    Day01 day01;

    auto lines = LoadInputLines("inputs/input01");
    day01.SetData(lines);
    day01.Solve();

    return 0;
}

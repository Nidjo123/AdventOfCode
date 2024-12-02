#include <iostream>
#include "Day01.h"
#include "Day02.h"
#include "util/Util.h"

int main() {
    std::vector<std::shared_ptr<Day> > days;
    days.push_back(std::make_shared<Day01>());
    days.push_back(std::make_shared<Day02>());

    for (auto i = 0; i < days.size(); i++) {
        auto lines = LoadInputLines(std::format("inputs/input{:02}", i + 1));
        const auto &day = days[i];
        day->SetData(lines);
        day->Solve();
    }

    return 0;
}

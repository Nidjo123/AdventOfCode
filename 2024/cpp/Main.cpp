#include <iostream>
#include "util/Util.h"
#include "Day01.h"
#include "Day02.h"
#include "Day03.h"
#include "Day04.h"
#include "Day05.h"
#include "Day06.h"

#ifdef NDEBUG
#define INPUT_FILE_FORMAT "inputs/input{:02}"
#else
#define INPUT_FILE_FORMAT "inputs/test_input{:02}"
#endif

int main() {
    std::vector<std::shared_ptr<Day> > days;
    days.push_back(std::make_shared<Day01>());
    days.push_back(std::make_shared<Day02>());
    days.push_back(std::make_shared<Day03>());
    days.push_back(std::make_shared<Day04>());
    days.push_back(std::make_shared<Day05>());
    days.push_back(std::make_shared<Day06>());

    for (auto i = 0; i < days.size(); i++) {
        auto lines = LoadInputLines(std::format(INPUT_FILE_FORMAT, i + 1));
        const auto &day = days[i];
        day->SetData(lines);
        day->Solve();
        std::cout << std::endl;
    }

    return 0;
}

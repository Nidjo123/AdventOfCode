#include "Day03.h"

#include <iostream>
#include <regex>

void Day03::SolvePart1() {
    const std::regex regex(R"(mul\(\s*(\d{1,3})\s*,\s*(\d{1,3})\s*\))");
    long sum = 0;

    for (const auto &line: lines) {
        auto match_iterator = std::sregex_iterator(line.begin(), line.end(), regex);
        const auto end_iterator = std::sregex_iterator{};
        for (; match_iterator != end_iterator; ++match_iterator) {
            auto match = *match_iterator;
            const auto x = std::stoi(match[1]);
            const auto y = std::stoi(match[2]);
            sum += x * y;
        }
    }

    std::cout << "Day03::Part1: " << sum << std::endl;
}

void Day03::SolvePart2() {
}

#include "Day01.h"

#include <algorithm>
#include <iostream>
#include <map>
#include <sstream>
#include <utility>

using IndexedValue = std::pair<int, int>;

void Day01::SolvePart1() {
    std::vector<IndexedValue> left, right;
    for (auto i = 0; i < lines.size(); i++) {
        std::istringstream ss{lines[i]};
        int lval, rval;
        ss >> lval >> rval;
        left.emplace_back(i, lval);
        right.emplace_back(i, rval);
    }

    auto comp = [](const IndexedValue &a, const IndexedValue &b) {
        if (a.second < b.second) {
            return true;
        } else if (a.second == b.second) {
            return a.first < b.first;
        }
        return false;
    };

    std::sort(left.begin(), left.end(), comp);
    std::sort(right.begin(), right.end(), comp);

    int distance_sum = 0;
    for (auto i = 0; i < left.size(); i++) {
        distance_sum += std::abs(right[i].second - left[i].second);
    }

    std::cout << "Day01::Part1: " << distance_sum << std::endl;
}

void Day01::SolvePart2() {
    std::vector<int> left;
    std::map<int, int> right_count;
    for (const auto &line: lines) {
        std::istringstream ss{line};
        int lval, rval;
        ss >> lval >> rval;
        left.push_back(lval);
        right_count[rval]++;
    }
    long similarity_score = 0;
    for (const auto &lval: left) {
        similarity_score += lval * right_count[lval];
    }
    std::cout << "Day01::Part2: " << similarity_score << std::endl;
}

#include "Day02.h"

#include <iostream>
#include <sstream>


void Day02::PreprocessData() {
    for (const auto &line: lines) {
        std::istringstream iss{line};

        std::vector<int> report;
        while (iss) {
            if (int level; iss >> level) {
                report.push_back(level);
            }
        }
        reports.push_back(report);
    }
}

bool IsSafe(const Report &report) {
    if (report.size() <= 1) {
        return true;
    }
    if (report[0] == report[1]) {
        return false;
    }
    const bool ascending = report[0] < report[1];
    for (auto i = 0; i < report.size() - 1; i++) {
        const auto diff = report[i] - report[i + 1];
        if (ascending && diff >= 0) {
            return false;
        } else if (!ascending && diff <= 0) {
            return false;
        }
        const auto abs_diff = std::abs(diff);
        if (abs_diff < 1 || abs_diff > 3) {
            return false;
        }
    }
    return true;
}

void Day02::SolvePart1() {
    const auto safe_reports = std::count_if(reports.begin(), reports.end(), IsSafe);
    std::cout << "Day02::Part1: " << safe_reports << std::endl;
}

void Day02::SolvePart2() {
}

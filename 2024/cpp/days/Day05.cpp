#include "Day05.h"
#include <algorithm>
#include <regex>
#include <iostream>

const std::regex constraint_regex{R"((\d+)\|(\d+))"};
const std::regex page_list_regex{R"((\d+),*)"};

void Day05::PreprocessData() {
    for (auto &line: lines) {
        if (line.empty()) {
            continue;
        }
        std::smatch match;
        if (std::regex_match(line, match, constraint_regex)) {
            auto before = std::stoi(match[1]);
            auto after = std::stoi(match[2]);
            if (constraints.contains(before)) {
                constraints[before].insert(after);
            } else {
                constraints.insert({before, {after}});
            }
        } else {
            std::vector<int> pages;
            auto match_iterator = std::sregex_iterator(line.begin(), line.end(), page_list_regex);
            const auto end_iterator = std::sregex_iterator{};
            for (; match_iterator != end_iterator; ++match_iterator) {
                const auto &page_match = *match_iterator;
                pages.push_back(std::stoi(page_match[1].str()));
            }
            print_jobs.push_back(pages);
        }
    }
}

void Day05::SolvePart1() {
    auto mid_sum = 0;
    for (const auto &print_job: print_jobs) {
        const auto sorted_job = GetSortedJob(print_job);
        if (print_job == sorted_job) {
            auto mid_idx = print_job.size() / 2;
            mid_sum += print_job[mid_idx];
        }
    }

    std::cout << "Day05::Part1: " << mid_sum << std::endl;
}

void Day05::SolvePart2() {
    auto mid_sum = 0;
    for (const auto &print_job: print_jobs) {
        const auto sorted_job = GetSortedJob(print_job);
        if (print_job != sorted_job) {
            auto mid_idx = sorted_job.size() / 2;
            mid_sum += sorted_job[mid_idx];
        }
    }

    std::cout << "Day05::Part2: " << mid_sum << std::endl;
}

Day05::PrintJob Day05::GetSortedJob(const Day05::PrintJob &print_job) const {
    std::vector<int> sorted_pages = print_job;
    std::sort(sorted_pages.begin(), sorted_pages.end(), [this](int x, int y) -> bool {
        return constraints.contains(x) && constraints.at(x).contains(y);
    });
    return sorted_pages;
}

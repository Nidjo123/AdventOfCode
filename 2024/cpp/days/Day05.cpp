#include "Day05.h"
#include <algorithm>
#include <regex>
#include <set>
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
            constraints.push_back(Constraint{before, after});
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
    std::set<int> constrained_pages;
    for (const auto &constraint: constraints) {
        constrained_pages.insert(constraint.before);
        constrained_pages.insert(constraint.after);
    }

    auto mid_sum = 0;
    for (const auto &print_job: print_jobs) {
        std::vector<int> filtered_pages;
        std::copy_if(print_job.begin(), print_job.end(), std::back_inserter(filtered_pages),
                     [&](int page) { return constrained_pages.contains(page); });
        bool valid = true;
        for (auto i = 0; i < filtered_pages.size(); i++) {
            for (const auto &constraint: constraints) {
                if (constraint.before == filtered_pages[i]) {
                    const auto end = filtered_pages.begin() + i;
                    auto it = std::find(filtered_pages.begin(), end, constraint.after);
                    if (it != end) {
                        valid = false;
                    }
                }
                if (!valid) {
                    break;
                }
            }
            if (!valid) {
                break;
            }
        }

        if (valid) {
            auto mid_idx = print_job.size() / 2;
            mid_sum += print_job[mid_idx];
        }
    }

    std::cout << "Day05::Part1: " << mid_sum << std::endl;
}

void Day05::SolvePart2() {

}

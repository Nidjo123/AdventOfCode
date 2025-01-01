#include "Day07.h"
#include <sstream>
#include <vector>
#include <iostream>


void Day07::PreprocessData() {
    for (const auto &line: lines) {
        const auto colon_idx = line.find(":");

        const auto ref_number = std::stoull(line.substr(0, colon_idx));
        std::vector<ull> numbers;

        ull x;
        std::istringstream iss{line.substr(colon_idx + 1)};
        while (iss >> x) {
            numbers.push_back(x);
        }
        equations.emplace_back(ref_number, numbers);
    }
}

void Day07::SolvePart1() {
    ull solvable_ref_sum = 0;
    for (const auto &equation: equations) {
        if (CanSolve1(equation)) {
            solvable_ref_sum += equation.left;
        }
    }
    std::cout << "Day07::Part1: " << solvable_ref_sum << std::endl;
}

void Day07::SolvePart2() {
    ull solvable_ref_sum = 0;
    for (const auto &equation: equations) {
        if (CanSolve2(equation)) {
            solvable_ref_sum += equation.left;
        }
    }
    std::cout << "Day07::Part2: " << solvable_ref_sum << std::endl;
}

bool Day07::CanSolve1(const Equation &equation, int pos, ull res) const {
    if (pos >= equation.right.size()) {
        return res == equation.left;
    }
    return CanSolve1(equation, pos + 1, res + equation.right[pos]) ||
           CanSolve1(equation, pos + 1, res * equation.right[pos]);
}

ull concat(ull x, ull y) {
    auto yy = y;
    while (yy) {
        yy /= 10;
        x *= 10;
    }
    return x + y;
}

bool Day07::CanSolve2(const Equation &equation, int pos, ull res) const {
    if (pos >= equation.right.size()) {
        return res == equation.left;
    }
    return CanSolve2(equation, pos + 1, res + equation.right[pos]) ||
           CanSolve2(equation, pos + 1, res * equation.right[pos]) ||
           CanSolve2(equation, pos + 1, concat(res, equation.right[pos]));
}

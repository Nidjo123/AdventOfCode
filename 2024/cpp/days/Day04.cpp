#include "Day04.h"

#include <iostream>


constexpr int N_DIRS = 8;
constexpr int dx[N_DIRS] = {+1, +1, 0, -1, -1, -1, 0, +1};
constexpr int dy[N_DIRS] = {0, -1, -1, -1, 0, +1, +1, +1};

void Day04::SolvePart1() {
    const std::string needle{"XMAS"};
    int count = 0;
    for (int y = 0; y < lines.size(); y++) {
        for (int x = 0; x < lines[y].size(); x++) {
            if (lines[y][x] == 'X') {
                for (int i = 0; i < N_DIRS; i++) {
                    count += search_word(x, y, i, needle);
                }
            }
        }
    }

    std::cout << "Day04::Part1: " << count << std::endl;
}

void Day04::SolvePart2() {
}

int Day04::search_word(int x, int y, int dir, const std::string &word, int depth) {
    if (x < 0 || y < 0 || y >= lines.size() || x >= lines[y].size()) {
        return 0;
    }
    if (depth < word.size() && lines[y][x] != word[depth]) {
        return 0;
    }
    if (depth == word.size() - 1) {
        return lines[y][x] == word[depth];
    }
    return search_word(x + dx[dir], y + dy[dir], dir, word, depth + 1);;
}

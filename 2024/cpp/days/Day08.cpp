#include "Day08.h"
#include <map>
#include <set>
#include <valarray>
#include <vector>
#include <iostream>

void Day08::PreprocessData() {
}

void Day08::SolvePart1() {
    using Point = std::valarray<int>;
    std::map<char, std::vector<Point>> points;
    for (auto y = 0; y < lines.size(); y++) {
        for (auto x = 0; x < lines[y].size(); x++) {
            const auto freq = lines[y][x];
            if (freq == '.') {
                continue;
            }
            const Point point{x, y};
            if (points.contains(freq)) {
                points[freq].push_back(point);
            } else {
                points[freq] = {point};
            }
        }
    }

    struct PointComparison {
        bool operator()(const Point &a, const Point &b) const {
            for (auto i = 0; i < a.size(); i++) {
                if (i >= b.size()) {
                    return false;
                }
                if (a[i] < b[i]) {
                    return true;
                }
                if (a[i] > b[i]) {
                    return false;
                }
            }
            return false;
        }
    };

    std::set<Point, PointComparison> antinodes;
    for (auto &[freq, freq_points]: points) {
        for (auto i = 0; i < freq_points.size() - 1; i++) {
            for (auto j = i + 1; j < freq_points.size(); j++) {
                const auto &a = freq_points[i];
                const auto &b = freq_points[j];
                const auto v = b - a;
                if (const auto p = a - v; IsWithinBounds(p[0], p[1])) {
                    antinodes.insert(p);
                }
                if (const auto p = b + v; IsWithinBounds(p[0], p[1])) {
                    antinodes.insert(p);
                }
            }
        }
    }

    std::cout << "Day08::Part1: " << antinodes.size() << std::endl;
}

void Day08::SolvePart2() {

}

bool Day08::IsWithinBounds(int x, int y) const {
    return x >= 0 && y >= 0 && y < lines.size() && x < lines[0].size();
}

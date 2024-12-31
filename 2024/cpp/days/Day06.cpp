#include "Day06.h"
#include <algorithm>
#include <cassert>
#include <iostream>
#include <set>
#include <tuple>

static const char GUARD_START_CHAR = '^';

struct Guard {
    enum class Direction {
        UP = 0,
        RIGHT = 1,
        DOWN = 2,
        LEFT = 3,
        Size
    };

    Direction getRightDirection(const Direction &direction) const {
        switch (direction) {
            case Direction::UP:
                return Direction::RIGHT;
            case Direction::RIGHT:
                return Direction::DOWN;
            case Direction::DOWN:
                return Direction::LEFT;
            case Direction::LEFT:
                return Direction::UP;
            default:
                assert(false);
        }
    }

    static constexpr int DX[] = {0, 1, 0, -1};
    static constexpr int DY[] = {-1, 0, 1, 0};

    Position GetNextPosition(const Position &pos, const Direction &dir) const {
        auto dir_idx = static_cast<int>(dir);
        return Position{pos.x + DX[dir_idx], pos.y + DY[dir_idx]};
    }

    using PathElement = std::tuple<int, int, Direction>;

    Position position;
    Direction direction;
    std::set<PathElement> path;
};

void Day06::SolvePart1() {
    auto map = lines;
    auto pos = GetStartPosition();
    auto guard = Guard{.position= pos, .direction=Guard::Direction::UP};

    do {
        map[guard.position.y][guard.position.x] = 'X';
        guard.path.insert({guard.position.x, guard.position.y, guard.direction});

        auto next_pos = guard.GetNextPosition(guard.position, guard.direction);
        if (!IsPositionInsideMap(next_pos)) {
            break;
        } else if (map[next_pos.y][next_pos.x] == '#') {
            auto new_dir = (static_cast<int>(guard.direction) + 1) % static_cast<int>(Guard::Direction::Size);
            guard.direction = static_cast<Guard::Direction>(new_dir);
        } else {
            guard.position = next_pos;
        }
    } while (IsPositionInsideMap(guard.position));

    auto visit_count = 0;
    for (const auto &row: map) {
        visit_count += std::count_if(row.begin(), row.end(), [](char c) { return c == 'X'; });
    }

    std::cout << "Day06::Part1: " << visit_count << std::endl;
}

void Day06::SolvePart2() {
}

Position Day06::GetStartPosition() const {
    for (auto y = 0; y < lines.size(); y++) {
        for (auto x = 0; x < lines[y].size(); x++) {
            if (lines[y][x] == GUARD_START_CHAR) {
                return Position{x, y};
            }
        }
    }
    assert(false);
}

void Day06::PreprocessData() {
    if (!lines.empty()) {
        const auto first_size = lines[0].size();
        for (auto i = 1; i < lines.size(); i++) {
            assert(lines[i].size() == first_size);
        }
    }
}

bool Day06::IsPositionInsideMap(const Position &position) const {
    return position.x >= 0 && position.y >= 0 && position.y < lines.size() && position.x < lines[0].size();
}

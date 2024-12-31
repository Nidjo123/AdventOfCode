#include "Day06.h"
#include <algorithm>
#include <cassert>
#include <iostream>
#include <set>
#include <tuple>

static const char GUARD_START_CHAR = '^';

Guard::Guard(const Position &pos, const Direction &dir) : position{pos}, direction{dir} {
    path.emplace_back(pos.x, pos.y, dir);
}

void Guard::Move(const Direction &dir = Direction::Size) {
    if (dir != Direction::Size) {
        direction = dir;
    }
    position = GetNextPosition(position, direction);
    path.emplace_back(position.x, position.y, direction);
}

Guard::Direction Guard::GetRightDirection(const Direction &direction) {
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

Position Guard::GetNextPosition(const Position &pos, const Direction &dir) {
    auto dir_idx = static_cast<int>(dir);
    return Position{pos.x + DX[dir_idx], pos.y + DY[dir_idx]};
}

std::set<Position> Guard::GetVisitedPositions() const {
    std::set<Position> visited_positions;
    for (const auto &path_elem: path) {
        const auto x = std::get<0>(path_elem);
        const auto y = std::get<1>(path_elem);
        visited_positions.insert({x, y});
    }
    return visited_positions;
}

void Guard::TurnRight() {
    direction = GetRightDirection(direction);
}

bool Guard::IsLooping() const {
    return std::find(path.begin(), path.end(), PathElement{position.x, position.y, direction}) < path.end() - 1;
}

void Day06::SolvePart1() {
    auto start_pos = GetStartPosition();
    auto guard = Guard(start_pos, Guard::Direction::UP);

    guard = SimulateGuard(guard);

    std::cout << "Day06::Part1: " << guard.GetVisitedPositions().size() << std::endl;
}

void Day06::SolvePart2() {
    auto &map = lines;
    auto start_pos = GetStartPosition();
    Guard guard(start_pos, Guard::Direction::UP);

    std::set<Position> potential_obstacles;
    do {
        auto next_pos = Guard::GetNextPosition(guard.position, guard.direction);
        if (!IsPositionInsideMap(next_pos)) {
            break;
        } else if (map[next_pos.y][next_pos.x] == '#') {
            guard.TurnRight();
        } else {
            if (!potential_obstacles.contains(next_pos)) {
                // try putting an obstacle in front
                const auto old_val = map[next_pos.y][next_pos.x];
                map[next_pos.y][next_pos.x] = '#';
                Guard new_guard(start_pos, Guard::Direction::UP);
                auto simulated_guard = SimulateGuard(new_guard);
                if (simulated_guard.IsLooping()) {
                    potential_obstacles.insert(next_pos);
                }
                map[next_pos.y][next_pos.x] = old_val;
            }
            guard.Move();
        }
    } while (IsPositionInsideMap(guard.position));

    std::cout << "Day06::Part2: " << potential_obstacles.size() << std::endl;
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

Guard Day06::SimulateGuard(Guard guard) const {
    const auto &map = lines;
    do {
        auto next_pos = Guard::GetNextPosition(guard.position, guard.direction);
        if (!IsPositionInsideMap(next_pos)) {
            break;
        } else if (map[next_pos.y][next_pos.x] == '#') {
            guard.TurnRight();
        } else {
            guard.Move();
            if (guard.IsLooping()) {
                break;
            }
        }
    } while (IsPositionInsideMap(guard.position));

    return guard;
}

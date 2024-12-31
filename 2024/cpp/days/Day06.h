#ifndef CPP_DAY06_H
#define CPP_DAY06_H


#include "Day.h"
#include <vector>
#include <set>


struct Position {
    int x;
    int y;

    bool operator<(const Position &other) const {
        return x < other.x || (x == other.x && y < other.y);
    }
};

struct Guard {
    enum class Direction {
        UP = 0,
        RIGHT = 1,
        DOWN = 2,
        LEFT = 3,
        Size
    };

    Guard(const Position &pos, const Direction &dir);

    void Move(const Direction &dir);

    static Direction GetRightDirection(const Direction &direction) ;

    static constexpr int DX[] = {0, 1, 0, -1};
    static constexpr int DY[] = {-1, 0, 1, 0};

    static Position GetNextPosition(const Position &pos, const Direction &dir);

    std::set<Position> GetVisitedPositions() const;

    void TurnRight();

    bool IsLooping() const;

    using PathElement = std::tuple<int, int, Direction>;

    Position position;
    Direction direction;
    std::vector<PathElement> path;
};

class Day06 : public Day {
public:
    void PreprocessData() override;

    void SolvePart1() override;

    void SolvePart2() override;

    Guard SimulateGuard(Guard) const;

    Position GetStartPosition() const;

    bool IsPositionInsideMap(const Position &) const;
};


#endif //CPP_DAY06_H

package com.nidjo123.days;

import com.nidjo123.Day;

import java.util.ArrayList;
import java.util.HashSet;
import java.util.List;
import java.util.Set;

public class Day09 extends Day {
    private enum MoveDir {
        LEFT(-1, 0), RIGHT(+1, 0), DOWN(0, -1), UP(0, +1);

        public final int dx;
        public final int dy;

        MoveDir(int dx, int dy) {
            this.dx = dx;
            this.dy = dy;
        }

        public static MoveDir fromString(String s) {
            return switch (s) {
                case "R" -> RIGHT;
                case "L" -> LEFT;
                case "D" -> DOWN;
                case "U" -> UP;
                default -> throw new RuntimeException("unknown direction: " + s);
            };
        }
    }

    private record Move(MoveDir direction, int steps) {
    }

    private record Position(int x, int y) {
        public Position getTranslated(int dx, int dy) {
            return new Position(x + dx, y + dy);
        }
    }

    private static class Rope {
        private final List<Position> parts = new ArrayList<>();

        public static Rope newFolded(int numParts, Position position) {
            Rope rope = new Rope();
            for (int i = 0; i < numParts; i++) {
                rope.addPart(position);
            }
            return rope;
        }

        public int length() {
            return parts.size();
        }

        public void addPart(Position position) {
            parts.add(position);
        }

        public Position getPartPos(int index) {
            return parts.get(index);
        }

        public void moveHead(MoveDir moveDir) {
            if (parts.size() == 0) {
                return;
            }
            parts.set(0, parts.get(0).getTranslated(moveDir.dx, moveDir.dy));
            for (int i = 1; i < length(); i++) {
                Position headPos = parts.get(i - 1);
                Position tailPos = parts.get(i);
                int dx = headPos.x - tailPos.x;
                int dy = headPos.y - tailPos.y;
                int adx = Math.abs(dx);
                int ady = Math.abs(dy);
                if (adx > 1 || ady > 1) {
                    int ddx = headPos.x < tailPos.x ? +1 : -1;
                    int ddy = headPos.y < tailPos.y ? +1 : -1;
                    if (dy == 0 || adx > ady) {
                        dx += ddx;
                    } else if (dx == 0 || adx < ady) {
                        dy += ddy;
                    } else {
                        dx += ddx;
                        dy += ddy;
                    }
                    parts.set(i, tailPos.getTranslated(dx, dy));
                } else {
                    break;
                }
            }
        }
    }

    private final List<Move> moves = new ArrayList<>();

    @Override
    public void init() {
        super.init();
        for (String line : lines) {
            if (line.isBlank()) {
                continue;
            }
            String[] parts = line.trim().split("\\s+");
            String dir = parts[0];
            int steps = Integer.parseInt(parts[1]);
            moves.add(new Move(MoveDir.fromString(dir), steps));
        }
    }

    private static final Position START_POS = new Position(0, 0);

    private int countUniqueTailPositions(Rope rope) {
        Set<Position> tailPositions = new HashSet<>();
        tailPositions.add(rope.getPartPos(rope.length() - 1));
        for (Move move : moves) {
            for (int step = 0; step < move.steps; step++) {
                rope.moveHead(move.direction);
                tailPositions.add(rope.getPartPos(rope.length() - 1));
            }
        }
        return tailPositions.size();
    }

    @Override
    public void part1() {
        Rope rope = Rope.newFolded(2, START_POS);
        System.out.println(countUniqueTailPositions(rope));
    }

    @Override
    public void part2() {
        Rope rope = Rope.newFolded(10, START_POS);
        System.out.println(countUniqueTailPositions(rope));
    }
}

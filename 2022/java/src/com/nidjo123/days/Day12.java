package com.nidjo123.days;

import com.nidjo123.Day;

import java.util.ArrayDeque;
import java.util.List;
import java.util.Queue;
import java.util.stream.Collectors;

public class Day12 extends Day {
    private static final char START_ELEVATION = 'a';
    private static final char END_ELEVATION = 'z';

    private static class HeightMap {
        private final List<String> map;
        private boolean[][] visited;

        public HeightMap(List<String> map) {
            this.map = map;
            resetVisited();
        }

        public int getWidth() {
            return map.get(0).length();
        }

        public int getHeight() {
            return map.size();
        }

        public char getPositionHeight(Position position) {
            char ch = map.get(position.y).charAt(position.x);
            return switch (ch) {
                case 'S' -> START_ELEVATION;
                case 'E' -> END_ELEVATION;
                default -> ch;
            };
        }

        private Position getCharPosition(char ch) {
            for (int y = 0; y < getHeight(); y++) {
                String s = map.get(y);
                int x = s.indexOf(ch);
                if (x >= 0) {
                    return new Position(x, y);
                }
            }
            throw new IllegalArgumentException("char " + ch + " not found");
        }

        public Position getStartPosition() {
            return getCharPosition('S');
        }

        public Position getEndPosition() {
            return getCharPosition('E');
        }

        public void visit(Position position) {
            visited[position.y][position.x] = true;
        }

        public boolean hasVisited(Position position) {
            return visited[position.y][position.x];
        }

        public void resetVisited() {
            visited = new boolean[getHeight()][getWidth()];
        }
    }

    private record Position(int x, int y) {
        public Position moved(int dx, int dy) {
            return new Position(x + dx, y + dy);
        }
    }

    private record PositionInfo(Position position, int steps) {
    }

    private HeightMap heightMap;

    private static final int[] dx = {0, +1, 0, -1};
    private static final int[] dy = {+1, 0, -1, 0};

    private boolean isPositionInsideMap(Position position) {
        return position.x >= 0 && position.y >= 0 && position.x < heightMap.getWidth() && position.y < heightMap.getHeight();
    }

    private int getMinStepsFrom(Position startPosition) {
        heightMap.resetVisited();
        Queue<PositionInfo> queue = new ArrayDeque<>();
        final Position endPosition = heightMap.getEndPosition();
        queue.add(new PositionInfo(startPosition, 0));
        while (!queue.isEmpty()) {
            PositionInfo positionInfo = queue.remove();
            Position currentPosition = positionInfo.position;
            if (heightMap.hasVisited(currentPosition)) {
                continue;
            }
            if (currentPosition.equals(endPosition)) {
                return positionInfo.steps;
            }
            heightMap.visit(currentPosition);
            char currentHeight = heightMap.getPositionHeight(currentPosition);

            for (int i = 0; i < dx.length; i++) {
                Position newPosition = currentPosition.moved(dx[i], dy[i]);
                if (!isPositionInsideMap(newPosition)) {
                    continue;
                }
                char newHeight = heightMap.getPositionHeight(newPosition);
                if (currentHeight - newHeight >= -1) {
                    queue.add(new PositionInfo(newPosition, positionInfo.steps + 1));
                }
            }
        }
        return Integer.MAX_VALUE;
    }

    @Override
    public void init() {
        super.init();
        List<String> nonBlankLines = lines.stream()
                .filter(s -> !s.isBlank())
                .map(String::trim)
                .collect(Collectors.toList());
        heightMap = new HeightMap(nonBlankLines);
    }

    @Override
    public void part1() {
        Position startPosition = heightMap.getStartPosition();
        System.out.println(getMinStepsFrom(startPosition));
    }

    @Override
    public void part2() {
        int minSteps = Integer.MAX_VALUE;
        for (int y = 0; y < heightMap.getHeight(); y++) {
            for (int x = 0; x < heightMap.getWidth(); x++) {
                Position position = new Position(x, y);
                if (heightMap.getPositionHeight(position) == 'a') {
                    minSteps = Math.min(minSteps, getMinStepsFrom(position));
                }
            }
        }
        System.out.println(minSteps);
    }
}

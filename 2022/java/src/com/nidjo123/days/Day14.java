package com.nidjo123.days;

import com.nidjo123.Day;

import java.util.*;

public class Day14 extends Day {
    private record Point(int x, int y) {
        public Point offsetBy(int dx, int dy) {
            return new Point(x + dx, y + dy);
        }

        public static Point fromString(String s) {
            String[] parts = s.trim().split("\\s*,\\s*");
            int x = Integer.parseInt(parts[0]);
            int y = Integer.parseInt(parts[1]);
            return new Point(x, y);
        }

        @Override
        public String toString() {
            return String.format("%d,%d", x, y);
        }
    }

    private record AABB(Point min, Point max) {
        boolean contains(Point p) {
            return p.x >= min.x && p.x <= max.x && p.y >= min.y && p.y <= max.y;
        }
    }

    private record LineSegment(Point a, Point b) {
        public boolean intersects(Point p) {
            if (a.x == b.x) {
                // vertical
                return p.x == a.x && p.y >= Math.min(a.y, b.y) && p.y <= Math.max(a.y, b.y);
            } else {
                // horizontal
                assert a.y == b.y;
                return p.y == a.y && p.x >= Math.min(a.x, b.x) && p.x <= Math.max(a.x, b.x);
            }
        }

        @Override
        public String toString() {
            return String.format("%s -> %s", a, b);
        }
    }

    private static class ConnectedLineSegments {
        private final List<LineSegment> lineSegments;
        private final AABB boundingBox;

        public ConnectedLineSegments(List<LineSegment> lineSegments) {
            this.lineSegments = lineSegments;
            this.boundingBox = calculateBoundingBox();
        }

        private AABB calculateBoundingBox() {
            int minx, miny;
            int maxx, maxy;
            minx = miny = Integer.MAX_VALUE;
            maxx = maxy = Integer.MIN_VALUE;
            for (LineSegment lineSegment : lineSegments) {
                for (Point p : List.of(lineSegment.a, lineSegment.b)) {
                    minx = Math.min(minx, p.x);
                    miny = Math.min(miny, p.y);
                    maxx = Math.max(maxx, p.x);
                    maxy = Math.max(maxy, p.y);
                }
            }
            return new AABB(new Point(minx, miny), new Point(maxx, maxy));
        }

        public boolean intersects(Point p) {
            if (boundingBox.contains(p)) {
                for (LineSegment lineSegment : lineSegments) {
                    if (lineSegment.intersects(p)) {
                        return true;
                    }
                }
            }
            return false;
        }

        public Point getMax() {
            return boundingBox.max;
        }
    }

    private final List<ConnectedLineSegments> lineSegmentGroups = new ArrayList<>();

    @Override
    public void init() {
        super.init();

        for (String line : lines) {
            if (line.isBlank()) {
                continue;
            }
            String[] parts = line.split("\\s*->\\s*");
            List<Point> points = Arrays.stream(parts).map(Point::fromString).toList();
            List<LineSegment> lineSegments = new ArrayList<>();
            for (int i = 0; i < points.size() - 1; i++) {
                lineSegments.add(new LineSegment(points.get(i), points.get(i + 1)));
            }
            lineSegmentGroups.add(new ConnectedLineSegments(lineSegments));
        }

        maxY = findMaxY();
    }

    private final Set<Point> sandParticles = new HashSet<>();

    private static final int[] dx = {0, -1, +1};
    private static final int[] dy = {+1, +1, +1};

    private int maxY;

    private int findMaxY() {
        OptionalInt optMaxY = lineSegmentGroups.stream().map(connectedLineSegments -> connectedLineSegments.getMax().y).mapToInt(Integer::intValue).max();
        assert optMaxY.isPresent();
        return optMaxY.getAsInt();
    }

    private boolean simulateSingleParticle() {
        int prevSize = sandParticles.size();
        Point sand = Day14.spawnPoint;
        while (sand.y <= maxY) {
            boolean moved = false;
            for (int i = 0; i < dx.length; i++) {
                Point candidate = sand.offsetBy(dx[i], dy[i]);
                boolean skip = false;
                for (ConnectedLineSegments lineSegmentGroup : lineSegmentGroups) {
                    if (lineSegmentGroup.intersects(candidate)) {
                        skip = true;
                        break;
                    }
                }
                if (skip) {
                    continue;
                }
                if (sandParticles.contains(candidate)) {
                    continue;
                }
                sand = candidate;
                moved = true;
                break;
            }
            if (!moved) {
                break;
            }
        }
        sandParticles.add(sand);
        return sand.y > maxY || prevSize >= sandParticles.size();
    }

    private static final Point spawnPoint = new Point(500, 0);

    @Override
    public void part1() {
        while (true) {
            if (simulateSingleParticle()) {
                break;
            }
        }
        System.out.println(sandParticles.size() - 1);
    }

    @Override
    public void part2() {
        maxY = findMaxY();
        Point left = new Point(Integer.MIN_VALUE, maxY + 2);
        Point right = new Point(Integer.MAX_VALUE, maxY + 2);
        LineSegment floor = new LineSegment(left, right);
        lineSegmentGroups.add(new ConnectedLineSegments(List.of(floor)));
        maxY = findMaxY();
        sandParticles.clear();
        while (true) {
            if (simulateSingleParticle()) {
                break;
            }
        }
        System.out.println(sandParticles.size());
    }
}

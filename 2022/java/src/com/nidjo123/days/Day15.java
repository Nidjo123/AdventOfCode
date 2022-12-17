package com.nidjo123.days;

import com.nidjo123.Day;

import java.util.HashSet;
import java.util.Set;
import java.util.regex.Matcher;
import java.util.regex.Pattern;

public class Day15 extends Day {
    private record Point(int x, int y) {
        public Point offsetBy(int dx, int dy) {
            return new Point(x + dx, y + dy);
        }

        public Point min(Point other) {
            return new Point(Math.min(x, other.x), Math.min(y, other.y));
        }

        public Point max(Point other) {
            return new Point(Math.max(x, other.x), Math.max(y, other.y));
        }

        public int manhattanDistance(Point other) {
            return Math.abs(x - other.x) + Math.abs(y - other.y);
        }

        @Override
        public String toString() {
            return String.format("%d,%d", x, y);
        }
    }

    private record Beacon(Point location) {
    }

    private static class Sensor {
        private final Point location;
        private final int closestBeaconDistance;

        public Sensor(Point location, Beacon closestBeacon) {
            this.location = location;
            this.closestBeaconDistance = location.manhattanDistance(closestBeacon.location);
        }

        public Point getMinDetectablePoint() {
            return location.offsetBy(-closestBeaconDistance, -closestBeaconDistance);
        }

        public Point getMaxDetectablePoint() {
            return location.offsetBy(closestBeaconDistance, closestBeaconDistance);
        }

        public boolean isBeaconLocationImpossible(Point p) {
            return location.manhattanDistance(p) <= closestBeaconDistance;
        }

        public int getNextPossibleX(Point p) {
            return p.x + closestBeaconDistance - location.manhattanDistance(p) + 1;
        }
    }

    private static final Pattern linePattern = Pattern.compile("Sensor at x=(?<sensorX>-?\\d+), y=(?<sensorY>-?\\d+): closest beacon is at x=(?<beaconX>-?\\d+), y=(?<beaconY>-?\\d+)");

    private final Set<Sensor> sensors = new HashSet<>();
    private Point minPoint = new Point(Integer.MAX_VALUE, Integer.MAX_VALUE);
    private Point maxPoint = new Point(Integer.MIN_VALUE, Integer.MIN_VALUE);

    @Override
    public void init() {
        super.init();

        for (String line : lines) {
            if (line.isBlank()) {
                continue;
            }
            Matcher matcher = linePattern.matcher(line);
            if (matcher.matches()) {
                int beaconX = Integer.parseInt(matcher.group("beaconX"));
                int beaconY = Integer.parseInt(matcher.group("beaconY"));
                Beacon beacon = new Beacon(new Point(beaconX, beaconY));
                int sensorX = Integer.parseInt(matcher.group("sensorX"));
                int sensorY = Integer.parseInt(matcher.group("sensorY"));
                Sensor sensor = new Sensor(new Point(sensorX, sensorY), beacon);
                minPoint = minPoint.min(sensor.getMinDetectablePoint());
                maxPoint = maxPoint.max(sensor.getMaxDetectablePoint());
                sensors.add(sensor);
            }
        }
    }

    private int countImpossibleBeaconLocations(int minX, int maxX) {
        assert minX <= maxX;
        Point beaconLocation = new Point(minX, 2_000_000);
        int impossibleBeaconLocationCount = 0;
        while (beaconLocation.x <= maxX) {
            int dx = 1;
            for (Sensor sensor : sensors) {
                if (sensor.isBeaconLocationImpossible(beaconLocation)) {
                    int nextXForSensor = Math.min(sensor.getNextPossibleX(beaconLocation), maxX);
                    dx = nextXForSensor - beaconLocation.x;
                    impossibleBeaconLocationCount += Math.max(dx - 1, 0);
                    break;
                }
            }
            beaconLocation = beaconLocation.offsetBy(Math.max(dx, 1), 0);
        }
        assert impossibleBeaconLocationCount <= (maxX - minX + 1);
        return impossibleBeaconLocationCount;
    }

    private long getTuningFrequency(Point p) {
        return p.x * 4_000_000L + p.y;
    }

    @Override
    public void part1() {
        System.out.println(countImpossibleBeaconLocations(minPoint.x, maxPoint.x));
    }

    @Override
    public void part2() {
        final int min = 0;
        final int max = 4_000_000;
        for (int y = min; y <= max; y++) {
            Point p = new Point(min, y);
            while (p.x <= max) {
                boolean possible = true;
                int nextX = p.x + 1;
                for (Sensor sensor : sensors) {
                    if (sensor.isBeaconLocationImpossible(p)) {
                        possible = false;
                        nextX = sensor.getNextPossibleX(p);
                        break;
                    }
                }
                if (possible) {
                    System.out.println(getTuningFrequency(p));
                    return;
                }
                p = new Point(nextX, y);
            }
        }
    }
}

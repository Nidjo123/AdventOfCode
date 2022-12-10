package com.nidjo123.days;

import com.nidjo123.Day;

import java.util.ArrayList;
import java.util.List;

public class Day04 extends Day {
    private static class Range {
        private final int a;
        private final int b;

        public Range(String s) {
            String[] parts = s.split("-");
            if (parts.length == 2) {
                this.a = Integer.parseInt(parts[0]);
                this.b = Integer.parseInt(parts[1]);
            } else {
                throw new IllegalArgumentException("cannot parse range from " + s);
            }
        }

        public boolean contains(Range other) {
            return this.a <= other.a && this.b >= other.b;
        }

        public boolean overlaps(Range other) {
            return (this.a >= other.a && this.a <= other.b) || (this.b >= other.a && this.b <= other.b) || contains(other);
        }
    }

    private final List<Range[]> elfPairs = new ArrayList<>();

    @Override
    public void part1() {
        int containedCount = 0;
        for (String line : lines) {
            if (line.isBlank()) {
                continue;
            }
            String[] rangeStrings = line.split(",");
            Range left = new Range(rangeStrings[0]);
            Range right = new Range(rangeStrings[1]);
            elfPairs.add(new Range[]{left, right});

            if (left.contains(right) || right.contains(left)) {
                containedCount++;
            }
        }
        System.out.println(containedCount);
    }

    @Override
    public void part2() {
        int overlapCount = 0;
        for (Range[] pair : elfPairs) {
            Range left = pair[0];
            Range right = pair[1];
            if (left.overlaps(right)) {
                overlapCount++;
            }
        }
        System.out.println(overlapCount);
    }
}

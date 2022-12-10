package com.nidjo123.days;

import com.nidjo123.Day;

import java.util.List;
import java.util.stream.Collectors;

class HeightField {
    private final int width;
    private final int height;
    private final byte[][] heightField;
    boolean[][] visible;

    private HeightField(int width, int height) {
        this.width = width;
        this.height = height;
        heightField = new byte[height][width];
    }

    public static HeightField fromLines(List<String> lines) {
        int fieldWidth = lines.get(0).length();
        int fieldHeight = lines.size();
        HeightField heightField = new HeightField(fieldWidth, fieldHeight);
        for (int y = 0; y < lines.size(); y++) {
            String line = lines.get(y);
            for (int x = 0; x < line.length(); x++) {
                String currHeight = String.valueOf(line.charAt(x));
                heightField.heightField[y][x] = Byte.parseByte(currHeight);
            }
        }
        heightField.determineVisibilityMap();
        return heightField;
    }

    void markInnerVisibility(int x, int y, int dx, int dy) {
        assert (dx != 0) ^ (dy != 0);
        int maxSoFar = heightField[y - dy][x - dx];
        while (y > 0 && y < height - 1 && x > 0 && x < width - 1) {
            int currHeight = heightField[y][x];
            if (currHeight > maxSoFar) {
                visible[y][x] = true;
                maxSoFar = currHeight;
            }
            x += dx;
            y += dy;
        }
    }

    private void determineVisibilityMap() {
        visible = new boolean[height][width];
        for (int y = 0; y < height; y++) {
            visible[y][0] = visible[y][width - 1] = true;
            markInnerVisibility(1, y, 1, 0);
            markInnerVisibility(width - 2, y, -1, 0);
        }
        for (int x = 0; x < width; x++) {
            visible[0][x] = visible[height - 1][x] = true;
            markInnerVisibility(x, 1, 0, 1);
            markInnerVisibility(x, height - 2, 0, -1);
        }
    }

    public int visibleTreeCount() {
        int visCount = 0;
        for (int y = 0; y < height; y++) {
            for (int x = 0; x < width; x++) {
                if (visible[y][x]) {
                    visCount++;
                }
            }
        }
        return visCount;
    }

    private int visibleTrees(int x, int y, int dx, int dy) {
        final int refHeight = heightField[y][x];
        x += dx;
        y += dy;
        int visCount = 0;
        while (x >= 0 && x < width && y >= 0 && y < height) {
            int currHeight = heightField[y][x];
            visCount++;
            if (currHeight >= refHeight) {
                break;
            }
            x += dx;
            y += dy;
        }
        return visCount;
    }

    public int getScenicScore(int x, int y) {
        if (x <= 0 || y <= 0 || x >= width - 1 || y >= height - 1) {
            return 0;
        }
        int visLeft = visibleTrees(x, y, -1, 0);
        int visRight = visibleTrees(x, y, +1, 0);
        int visUp = visibleTrees(x, y, 0, -1);
        int visDown = visibleTrees(x, y, 0, +1);
        return visLeft * visRight * visUp * visDown;
    }

    public int getWidth() {
        return width;
    }

    public int getHeight() {
        return height;
    }
}

public class Day08 extends Day {
    private HeightField heightField;

    @Override
    public void init() {
        super.init();
        List<String> trimmedNonEmptyLines = lines.stream().filter(s -> !s.isBlank()).map(String::strip).collect(Collectors.toList());
        heightField = HeightField.fromLines(trimmedNonEmptyLines);
    }

    @Override
    public void part1() {
        System.out.println(heightField.visibleTreeCount());
    }

    @Override
    public void part2() {
        int maxScenicScore = 0;
        for (int y = 0; y < heightField.getHeight(); y++) {
            for (int x = 0; x < heightField.getWidth(); x++) {
                maxScenicScore = Math.max(maxScenicScore, heightField.getScenicScore(x, y));
            }
        }
        System.out.println(maxScenicScore);
    }
}

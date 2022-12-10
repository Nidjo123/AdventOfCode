package com.nidjo123.days;

import com.nidjo123.Day;
import com.nidjo123.Utils;

public class Day06 extends Day {
    @Override
    public void init() {
        super.init();
        assert lines.size() == 1;
    }

    private String getSignal() {
        return lines.get(0);
    }

    private int indexOfFirstUniqueChunk(String signal, int chunkSize) {
        for (int i = 0; i < signal.length() - chunkSize; i++) {
            String chunk = signal.substring(i, i + chunkSize);
            if (Utils.makeSetFromString(chunk).size() == chunkSize) {
                return i + chunkSize;
            }
        }
        return -1;
    }

    @Override
    public void part1() {
        int startIndex = indexOfFirstUniqueChunk(getSignal(), 4);
        System.out.println(startIndex);
    }

    @Override
    public void part2() {
        int startIndex = indexOfFirstUniqueChunk(getSignal(), 14);
        System.out.println(startIndex);
    }
}

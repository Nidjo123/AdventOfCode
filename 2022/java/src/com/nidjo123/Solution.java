package com.nidjo123;

import java.util.List;

public interface Solution {
    /**
     * @param lines lines from the input file for given day
     */
    void setInput(List<String> lines);

    default void init() {
    }

    /**
     * Solve and print the solution for part 1.
     */
    void part1();

    /**
     * Solve and print the solution for part 2.
     */
    void part2();
}

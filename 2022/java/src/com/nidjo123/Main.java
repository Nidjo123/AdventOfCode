package com.nidjo123;

import com.nidjo123.days.*;

import java.io.IOException;
import java.util.List;

public class Main {
    static Solution[] solutions = {
            new Day01(),
            new Day02(),
            new Day03(),
            new Day04(),
            new Day05(),
            new Day06(),
            new Day07(),
            new Day08(),
            new Day09(),
            new Day10(),
            new Day11(),
            new Day12(),
    };

    public static void main(String[] args) throws IOException {
        for (int i = 0; i < solutions.length; i++) {
            System.out.println("== Day " + (i + 1) + " ==");
            List<String> lines = Utils.readInputForDay(i + 1);
            Solution solution = solutions[i];
            solution.setInput(lines);
            solution.init();
            System.out.print("Part 1: ");
            solution.part1();
            System.out.print("Part 2: ");
            solution.part2();
        }
    }
}
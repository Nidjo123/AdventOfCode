package com.nidjo123.days;

import com.nidjo123.Day;

import java.util.ArrayList;
import java.util.List;
import java.util.OptionalInt;

public class Day01 extends Day {
    private final List<Integer> elfCalories = new ArrayList<>();

    private void determineElfCalories() {
        List<Integer> running = new ArrayList<>();
        for (String line : lines) {
            if (!line.trim().isEmpty()) {
                int calories = Integer.parseInt(line);
                running.add(calories);
            } else {
                if (!running.isEmpty()) {
                    int calories = running.stream().mapToInt(i -> i).sum();
                    elfCalories.add(calories);
                }
                running.clear();
            }
        }
    }

    @Override
    public void part1() {
        determineElfCalories();
        OptionalInt maxElfCalories = elfCalories.stream().mapToInt(Integer::intValue).max();
        if (maxElfCalories.isPresent()) {
            System.out.println(maxElfCalories.getAsInt());
        } else {
            throw new RuntimeException();
        }
    }

    @Override
    public void part2() {
        List<Integer> sortedElfCalories = elfCalories.stream().mapToInt(Integer::intValue).sorted().boxed().toList();
        int maxThreeElfs = sortedElfCalories.subList(sortedElfCalories.size() - 3, sortedElfCalories.size()).stream().mapToInt(Integer::intValue).sum();
        System.out.println(maxThreeElfs);
    }
}

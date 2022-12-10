package com.nidjo123.days;

import com.nidjo123.Day;
import com.nidjo123.Utils;

import java.util.ArrayList;
import java.util.List;
import java.util.Set;

public class Day03 extends Day {
    private int getPriority(char item) {
        if (Character.isLowerCase(item)) {
            return item - 'a' + 1;
        } else {
            return item - 'A' + ('z' - 'a' + 1) + 1;
        }
    }


    private char intersectionOfStrings(List<String> strings) {
        assert strings.size() != 0;
        Set<Character> common = Utils.makeSetFromString(strings.get(0));
        for (int i = 1; i < strings.size(); i++) {
            common.retainAll(Utils.makeSetFromString(strings.get(i)));
        }
        assert common.size() == 1;
        return (char) common.toArray()[0];
    }

    @Override
    public void part1() {
        int sumOfPriorities = 0;
        for (String rucksack : lines) {
            if (rucksack.isBlank()) {
                continue;
            }
            assert rucksack.length() % 2 == 0;
            int half = rucksack.trim().length() / 2;
            String compartment1 = rucksack.substring(0, half);
            String compartment2 = rucksack.substring(half);
            char commonItem = intersectionOfStrings(List.of(compartment1, compartment2));
            sumOfPriorities += getPriority(commonItem);
        }
        System.out.println(sumOfPriorities);
    }

    @Override
    public void part2() {
        int sumOfPriorities = 0;
        for (int i = 0; i < lines.size(); i += 3) {
            if (lines.get(i).isBlank()) {
                continue;
            }
            List<String> items = new ArrayList<>();
            for (int j = 0; j < 3; j++) {
                items.add(lines.get(i + j));
            }
            char commonItem = intersectionOfStrings(items);
            sumOfPriorities += getPriority(commonItem);
        }
        System.out.println(sumOfPriorities);
    }
}

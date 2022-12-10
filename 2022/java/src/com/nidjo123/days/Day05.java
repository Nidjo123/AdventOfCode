package com.nidjo123.days;

import com.nidjo123.Day;

import java.util.ArrayList;
import java.util.List;
import java.util.regex.Matcher;
import java.util.regex.Pattern;

public class Day05 extends Day {
    private final List<String> crates = new ArrayList<>();
    private final List<Move> moves = new ArrayList<>();

    private record Move(int stackSize, int from, int to) {
    }

    private static final Pattern movePattern = Pattern.compile("move\\s+(\\d+)\\s+from\\s+(\\d+)\\s+to\\s+(\\d+)\\s*");

    private void parseInput() {
        boolean readCrates = false;
        for (int i = 0; i < lines.size(); i++) {
            String line = lines.get(i);
            if (line.isBlank()) {
                continue;
            }
            if (line.replaceAll("\\s+", "").matches("\\d+")) {
                for (int j = 0; j < line.length(); j++) {
                    if (Character.isDigit(line.charAt(j))) {
                        StringBuilder sb = new StringBuilder();
                        for (int k = i - 1; k >= 0; k--) {
                            char c = lines.get(k).charAt(j);
                            if (Character.isLetter(c)) {
                                sb.append(c);
                            } else {
                                break;
                            }
                        }
                        crates.add(sb.toString());
                    }
                }
                readCrates = true;
            } else if (readCrates) {
                Matcher matcher = movePattern.matcher(line);
                if (matcher.matches()) {
                    int stackSize = Integer.parseInt(matcher.group(1));
                    int from = Integer.parseInt(matcher.group(2));
                    int to = Integer.parseInt(matcher.group(3));
                    moves.add(new Move(stackSize, from - 1, to - 1));
                }
            }
        }
    }

    @Override
    public void init() {
        parseInput();
    }

    private String reversed(String s) {
        StringBuilder sb = new StringBuilder(s);
        return sb.reverse().toString();
    }

    private String getTopCrates(List<String> crates) {
        StringBuilder sb = new StringBuilder();
        for (String stack : crates) {
            sb.append(stack.charAt(stack.length() - 1));
        }
        return sb.toString();
    }

    @Override
    public void part1() {
        List<String> myCrates = new ArrayList<>(crates);
        for (Move move : moves) {
            String from = myCrates.get(move.from);
            String to = myCrates.get(move.to);
            myCrates.set(move.from, from.substring(0, from.length() - move.stackSize));
            myCrates.set(move.to, to + reversed(from.substring(from.length() - move.stackSize)));
        }
        System.out.println(getTopCrates(myCrates));
    }

    @Override
    public void part2() {
        List<String> myCrates = new ArrayList<>(crates);
        for (Move move : moves) {
            String from = myCrates.get(move.from);
            String to = myCrates.get(move.to);
            myCrates.set(move.from, from.substring(0, from.length() - move.stackSize));
            myCrates.set(move.to, to + from.substring(from.length() - move.stackSize));
        }
        System.out.println(getTopCrates(myCrates));
    }
}

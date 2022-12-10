package com.nidjo123.days;

import com.nidjo123.Day;

public class Day02 extends Day {
    private enum Hand {
        ROCK(1),
        PAPER(2),
        SCISSORS(3);

        private final int score;

        Hand(int score) {
            this.score = score;
        }

        public boolean defeats(Hand other) {
            if (this == ROCK) {
                return other == SCISSORS;
            } else if (this == SCISSORS) {
                return other == PAPER;
            } else {
                return other == ROCK;
            }
        }
    }

    private enum Outcome {
        LOST(0),
        TIE(3),
        WON(6);

        private final int score;

        Outcome(int score) {
            this.score = score;
        }
    }

    private Hand decryptLetter(String letter) {
        return switch (letter) {
            case "A", "X" -> Hand.ROCK;
            case "B", "Y" -> Hand.PAPER;
            case "C", "Z" -> Hand.SCISSORS;
            default -> throw new RuntimeException("unknown letter: " + letter);
        };
    }

    private Outcome roundOutcome(Hand enemy, Hand you) {
        if (you.defeats(enemy)) {
            return Outcome.WON;
        } else if (enemy == you) {
            return Outcome.TIE;
        } else {
            return Outcome.LOST;
        }
    }

    private int totalScore(Hand enemy, Hand you) {
        return you.score + roundOutcome(enemy, you).score;
    }

    @Override
    public void part1() {
        int score = 0;
        for (String line : lines) {
            if (line.trim().isEmpty()) {
                continue;
            }
            String[] letters = line.trim().split("\\s+");
            assert letters.length == 2;
            Hand enemy = decryptLetter(letters[0]);
            Hand you = decryptLetter(letters[1]);
            score += totalScore(enemy, you);
        }
        System.out.println(score);
    }

    private Outcome decryptOutcome(String letter) {
        if (letter.contentEquals("X")) {
            return Outcome.LOST;
        } else if (letter.contentEquals("Y")) {
            return Outcome.TIE;
        } else if (letter.contentEquals("Z")) {
            return Outcome.WON;
        }
        throw new RuntimeException("unknown outcome letter: " + letter);
    }

    private Hand handForOutcome(Hand enemy, Outcome outcome) {
        for (Hand you : Hand.values()) {
            if (roundOutcome(enemy, you) == outcome) {
                return you;
            }
        }
        throw new RuntimeException();
    }

    @Override
    public void part2() {
        int score = 0;
        for (String line : lines) {
            if (line.trim().isEmpty()) {
                continue;
            }
            String[] letters = line.trim().split("\\s+");
            assert letters.length == 2;
            Hand enemy = decryptLetter(letters[0]);
            Outcome outcome = decryptOutcome(letters[1]);
            Hand you = handForOutcome(enemy, outcome);
            assert roundOutcome(enemy, you) == outcome;
            score += totalScore(enemy, you);
        }
        System.out.println(score);
    }
}

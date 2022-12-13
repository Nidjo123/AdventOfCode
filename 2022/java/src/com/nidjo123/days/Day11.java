package com.nidjo123.days;

import com.nidjo123.Day;

import java.math.BigInteger;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.function.Function;
import java.util.function.Predicate;
import java.util.regex.Matcher;
import java.util.regex.Pattern;

public class Day11 extends Day {
    private record Item(BigInteger worry) {
    }

    private static class Monkey {
        private final List<Item> items = new ArrayList<>();
        private Function<BigInteger, BigInteger> expression;
        private Function<BigInteger, BigInteger> nextValueGetter;
        private Predicate<BigInteger> test;
        private Monkey nextFalse;
        private Monkey nextTrue;
        private int inspectionCount = 0;


        public void addItem(Item item) {
            items.add(item);
        }

        public void setExpression(Function<BigInteger, BigInteger> expression) {
            assert this.expression == null;
            this.expression = expression;
        }

        public void setNextValueGetter(Function<BigInteger, BigInteger> nextValueGetter) {
            this.nextValueGetter = nextValueGetter;
        }

        public void setTest(Predicate<BigInteger> test) {
            assert this.test == null;
            this.test = test;
        }

        public void setNextFalse(Monkey nextFalse) {
            assert nextFalse != this;
            this.nextFalse = nextFalse;
        }

        public void setNextTrue(Monkey nextTrue) {
            assert nextTrue != this;
            this.nextTrue = nextTrue;
        }

        public void inspectItems() {
            for (Item item : items) {
                BigInteger newWorry = nextValueGetter.apply(expression.apply(item.worry));
                Item newItem = new Item(newWorry);
                if (test.test(newWorry)) {
                    nextTrue.addItem(newItem);
                } else {
                    nextFalse.addItem(newItem);
                }
            }
            inspectionCount += items.size();
            items.clear();
        }

        public int getInspectionCount() {
            return inspectionCount;
        }
    }

    private final List<Monkey> monkeys = new ArrayList<>();
    private long commonMultiple;
    private static final Pattern MONKEY_BEGIN = Pattern.compile("Monkey \\d+:");
    private static final Pattern MONKEY_ITEMS = Pattern.compile("\\s*Starting items: ([\\d,\\s]+)");
    private static final Pattern WORRY_EXPR = Pattern.compile("\\s*Operation: new\\s*=\\s*(.+)");
    private static final Pattern TEST_EXPR = Pattern.compile("\\s*Test: divisible by (\\d+)");
    private static final Pattern DECISION = Pattern.compile("\\s*If (true|false): throw to monkey (\\d+)");

    private static Function<BigInteger, BigInteger> exprFromString(String expr) {
        String[] parts = expr.split("\\s+");
        assert parts.length == 3 && parts[0].equals("old");
        String operator = parts[1];
        String operand = parts[2];
        return switch (operator) {
            case "+" -> (x) -> x.add(operand.equals("old") ? x : BigInteger.valueOf(Long.parseLong(operand)));
            case "*" -> (x) -> x.multiply(operand.equals("old") ? x : BigInteger.valueOf(Long.parseLong(operand)));
            default -> throw new RuntimeException("cannot parse expression: " + expr);
        };
    }

    @Override
    public void init() {
        super.init();

        commonMultiple = 1;
        Monkey monkey = null;
        Map<Monkey, Integer> nextMonkeyIfTrue = new HashMap<>();
        Map<Monkey, Integer> nextMonkeyIfFalse = new HashMap<>();
        for (String line : lines) {
            if (line.isBlank()) {
                if (monkey != null) {
                    monkeys.add(monkey);
                    monkey = null;
                }
                continue;
            }

            Matcher monkeyBeginMatcher = MONKEY_BEGIN.matcher(line);
            if (monkeyBeginMatcher.matches()) {
                monkey = new Monkey();
                continue;
            }

            assert monkey != null;

            Matcher monkeyItemsMatcher = MONKEY_ITEMS.matcher(line);
            if (monkeyItemsMatcher.matches()) {
                String items = monkeyItemsMatcher.group(1);
                for (String item : items.split("\\s*,\\s*")) {
                    BigInteger worry = BigInteger.valueOf(Integer.parseInt(item));
                    monkey.addItem(new Item(worry));
                }
                continue;
            }

            Matcher worryExprMatcher = WORRY_EXPR.matcher(line);
            if (worryExprMatcher.matches()) {
                String expr = worryExprMatcher.group(1);
                monkey.setExpression(exprFromString(expr));
                continue;
            }

            Matcher testExprMatcher = TEST_EXPR.matcher(line);
            if (testExprMatcher.matches()) {
                int divisor = Integer.parseInt(testExprMatcher.group(1));
                commonMultiple *= divisor;
                monkey.setTest((x) -> x.mod(BigInteger.valueOf(divisor)).equals(BigInteger.ZERO));
                continue;
            }

            Matcher decisionMatcher = DECISION.matcher(line);
            if (decisionMatcher.matches()) {
                String test = decisionMatcher.group(1);
                int nextMonkey = Integer.parseInt(decisionMatcher.group(2));
                if (test.equalsIgnoreCase("false")) {
                    nextMonkeyIfFalse.put(monkey, nextMonkey);
                } else {
                    nextMonkeyIfTrue.put(monkey, nextMonkey);
                }
                continue;
            }

            throw new RuntimeException("cannot parse line: " + line);
        }
        if (monkey != null) {
            monkeys.add(monkey);
        }

        for (Map.Entry<Monkey, Integer> entry : nextMonkeyIfFalse.entrySet()) {
            Monkey next = monkeys.get(entry.getValue());
            entry.getKey().setNextFalse(next);
        }
        for (Map.Entry<Monkey, Integer> entry : nextMonkeyIfTrue.entrySet()) {
            Monkey next = monkeys.get(entry.getValue());
            entry.getKey().setNextTrue(next);
        }
    }

    private void doRounds(int rounds) {
        for (int round = 0; round < rounds; round++) {
            for (Monkey monkey : monkeys) {
                monkey.inspectItems();
            }
        }
    }

    private long getMonkeyBusiness() {
        long[] inspectionCounts = monkeys.stream().mapToLong(Monkey::getInspectionCount).sorted().toArray();
        int size = inspectionCounts.length;
        return inspectionCounts[size - 1] * inspectionCounts[size - 2];
    }

    private void resetMonkeys() {
        monkeys.clear();
        init();
    }

    @Override
    public void part1() {
        for (Monkey monkey : monkeys) {
            monkey.setNextValueGetter(x -> x.divide(BigInteger.valueOf(3)));
        }
        doRounds(20);
        System.out.println(getMonkeyBusiness());
    }

    @Override
    public void part2() {
        resetMonkeys();
        for (Monkey monkey : monkeys) {
            monkey.setNextValueGetter(x -> x.remainder(BigInteger.valueOf(commonMultiple)));
        }
        doRounds(10_000);
        System.out.println(getMonkeyBusiness());
    }
}

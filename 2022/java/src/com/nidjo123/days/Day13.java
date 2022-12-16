package com.nidjo123.days;

import com.nidjo123.Day;

import java.util.ArrayList;
import java.util.List;
import java.util.regex.Matcher;
import java.util.regex.Pattern;

public class Day13 extends Day {
    private record ListItem(Day13.ListItem.ItemType type, Object object) implements Comparable<ListItem> {
        private enum ItemType {
            INTEGER, LIST
        }

        public static int compareLists(List<ListItem> left, List<ListItem> right) {
            int i = 0;
            while (i < left.size() && i < right.size()) {
                ListItem leftItem = left.get(i);
                ListItem rightItem = right.get(i);
                int comparison = leftItem.compareTo(rightItem);
                if (comparison != 0) {
                    return comparison;
                }
                i++;
            }
            return Integer.compare(left.size(), right.size());
        }

        @Override
        public int compareTo(ListItem other) {
            if (type == other.type) {
                if (type == ItemType.INTEGER) {
                    return Integer.compare((int) object, (int) other.object);
                } else {
                    return compareLists((List<ListItem>) object, (List<ListItem>) other.object);
                }
            } else {
                if (type == ItemType.INTEGER) {
                    return compareLists(List.of(this), (List<ListItem>) other.object);
                } else {
                    return compareLists((List<ListItem>) object, List.of(other));
                }
            }
        }

        @Override
        public String toString() {
            if (type == ItemType.INTEGER) {
                return String.valueOf((int) object);
            } else {
                assert type == ItemType.LIST;
                List<ListItem> list = (List<ListItem>) object;
                StringBuilder sb = new StringBuilder("[");
                for (int i = 0; i < list.size(); i++) {
                    sb.append(list.get(i));
                    if (i != list.size() - 1) {
                        sb.append(',');
                    }
                }
                sb.append(']');
                return sb.toString();
            }
        }
    }

    private static final Pattern intPattern = Pattern.compile("\\d+");

    private int parseListImpl(String s, int index, List<ListItem> acc) {
        if (index >= s.length()) {
            return index;
        }
        char ch = s.charAt(index);
        Matcher intMatcher = intPattern.matcher(s);
        if (intMatcher.find(index) && intMatcher.start() == index) {
            int value = Integer.parseInt(intMatcher.group());
            acc.add(new ListItem(ListItem.ItemType.INTEGER, value));
            return parseListImpl(s, intMatcher.end(), acc);
        } else if (ch == '[') {
            List<ListItem> subList = new ArrayList<>();
            int nextIndex = parseListImpl(s, index + 1, subList);
            acc.add(new ListItem(ListItem.ItemType.LIST, subList));
            return parseListImpl(s, nextIndex, acc);
        } else if (ch == ']') {
            return index + 1;
        }
        assert ch == ',';
        return parseListImpl(s, index + 1, acc);
    }

    private List<ListItem> parseList(String s) {
        // strip first []
        String inside = s.substring(1, s.length() - 1);
        List<ListItem> list = new ArrayList<>();
        parseListImpl(inside, 0, list);
        return list;
    }

    private record PacketPair(List<ListItem> left, List<ListItem> right) {
    }

    private final List<List<ListItem>> packets = new ArrayList<>();

    @Override
    public void init() {
        super.init();

        for (String line : lines) {
            if (!line.isBlank()) {
                packets.add(parseList(line.trim()));
            }
        }

        List<String> nonEmptyLines = lines.stream().filter(s -> !s.isBlank()).toList();
        for (int i = 0; i < packets.size(); i++) {
            ListItem listItem = new ListItem(ListItem.ItemType.LIST, packets.get(i));
            assert nonEmptyLines.get(i).equals(String.valueOf(listItem));
        }
    }

    @Override
    public void part1() {
        List<PacketPair> packetPairs = new ArrayList<>();
        for (int i = 0; i < packets.size(); i += 2) {
            packetPairs.add(new PacketPair(packets.get(i), packets.get(i + 1)));
        }

        int indexSum = 0;
        for (int i = 0; i < packetPairs.size(); i++) {
            PacketPair packetPair = packetPairs.get(i);
            int comparison = ListItem.compareLists(packetPair.left, packetPair.right);
            if (comparison < 0) {
                indexSum += i + 1;
            }
        }
        System.out.println(indexSum);
    }

    @Override
    public void part2() {
        List<ListItem> divider1 = parseList("[[2]]");
        List<ListItem> divider2 = parseList("[[6]]");
        packets.add(divider1);
        packets.add(divider2);
        packets.sort(ListItem::compareLists);
        int div1Index = packets.indexOf(divider1) + 1;
        int div2Index = packets.indexOf(divider2) + 1;
        System.out.println(div1Index * div2Index);
    }
}

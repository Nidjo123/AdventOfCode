package com.nidjo123;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.List;
import java.util.Objects;
import java.util.Set;
import java.util.stream.Collectors;

public class Utils {

    public static List<String> readInputForDay(int day) throws IOException {
        String name = String.format("/input%02d", day);
        String resPath = Objects.requireNonNull(Utils.class.getResource(name)).getPath();
        if (resPath == null) {
            throw new RuntimeException("Input does not exist: " + resPath);
        }
        Path path = Path.of(resPath);
        return Files.readAllLines(path);
    }

    public static Set<Character> makeSetFromString(String s) {
        return s.chars().mapToObj(c -> (char) c).collect(Collectors.toSet());
    }
}

package com.nidjo123.days;

import com.nidjo123.Day;

import java.util.ArrayList;
import java.util.List;
import java.util.Optional;

class File {
    private final File parent;
    private final String name;
    private final List<File> children = new ArrayList<>();
    private int size = 0;

    public File(File parent, String name) {
        this.parent = parent;
        this.name = name;
    }

    public File(File parent, String name, int size) {
        assert size >= 0;
        this.parent = parent;
        this.name = name;
        this.size = size;
    }

    public File addChild(String name) {
        return addChild(name, 0);
    }

    public File addChild(String name, int size) {
        for (File child : children) {
            if (child.name.equals(name)) {
                assert child.size == size;
                return child;
            }
        }
        File file = new File(this, name, size);
        children.add(file);
        return file;
    }

    public File getRoot() {
        File root = this;
        while (root.parent != null) {
            root = root.parent;
        }
        return root;
    }

    public boolean isDir() {
        return size == 0;
    }

    public File cd(String name) {
        if (name.equals("..")) {
            return this.parent;
        } else if (name.equals("/")) {
            return getRoot();
        }
        for (File child : children) {
            if (child.name.equals(name) && child.isDir()) {
                return child;
            }
        }
        return addChild(name);
    }

    public long getTotalSize() {
        long totalSize = size;
        for (File child : children) {
            totalSize += child.getTotalSize();
        }
        return totalSize;
    }

    public long getSizeSumOfDirsAtMost(long biggestSize) {
        long totalSize = 0;
        for (File child : children) {
            totalSize += child.getSizeSumOfDirsAtMost(biggestSize);
        }
        if (isDir()) {
            long thisTotalSize = getTotalSize();
            if (thisTotalSize <= biggestSize) {
                totalSize += thisTotalSize;
            }
        }
        return totalSize;
    }

    public long getSmallestDirSizeAtLeast(long smallestSize) {
        long resultSize = Integer.MAX_VALUE;
        if (isDir()) {
            long totalSize = getTotalSize();
            if (totalSize >= smallestSize) {
                resultSize = Math.min(resultSize, totalSize);
            }
            for (File child : children) {
                resultSize = Math.min(resultSize, child.getSmallestDirSizeAtLeast(smallestSize));
            }
        }
        return resultSize;
    }
}

public class Day07 extends Day {
    private final File root = new File(null, "/");

    private Optional<String> getCommand(String line) {
        if (line.trim().startsWith("$")) {
            return Optional.of(line.split("\\s+")[1]);
        }
        return Optional.empty();
    }

    @Override
    public void init() {
        super.init();

        File currDir = root;
        for (String line : lines) {
            if (line.isBlank()) {
                continue;
            }
            Optional<String> command = getCommand(line);
            if (command.isPresent() && command.get().equals("cd")) {
                String[] parts = line.trim().split("\\s+");
                currDir = currDir.cd(parts[parts.length - 1]);
            } else if (command.isEmpty()) {
                // ls output
                String[] lsParts = line.trim().split("\\s+");
                String name = lsParts[1];
                if (lsParts[0].equals("dir")) {
                    currDir.addChild(name);
                } else {
                    int size = Integer.parseInt(lsParts[0]);
                    currDir.addChild(name, size);
                }
            }
        }
    }

    @Override
    public void part1() {
        System.out.println(root.getSizeSumOfDirsAtMost(100_000));
    }

    @Override
    public void part2() {
        final long totalSize = 70_000_000;
        final long needFree = 30_000_000;
        long currentlyFree = totalSize - root.getTotalSize();
        long needToRemove = needFree - currentlyFree;
        System.out.println(root.getSmallestDirSizeAtLeast(needToRemove));
    }
}

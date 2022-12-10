package com.nidjo123.days;

import com.nidjo123.Day;

import java.util.ArrayList;
import java.util.List;

class CPU {
    private int X = 1;

    public interface Instruction {
        void execute(CPU cpu);

        int getCycleDuration();
    }

    public static class NoopInstr implements Instruction {
        @Override
        public void execute(CPU cpu) {
        }

        @Override
        public int getCycleDuration() {
            return 1;
        }
    }

    public static class AddxInstr implements Instruction {
        private final int V;

        public AddxInstr(int V) {
            this.V = V;
        }

        @Override
        public void execute(CPU cpu) {
            cpu.X += V;
        }

        @Override
        public int getCycleDuration() {
            return 2;
        }
    }

    public List<Integer> execute(List<Instruction> instructions) {
        List<Integer> values = new ArrayList<>();
        for (Instruction instruction : instructions) {
            for (int i = 0; i < instruction.getCycleDuration(); i++) {
                values.add(X);
            }
            instruction.execute(this);
        }
        return values;
    }
}

class CRT {
    private final int width;
    private final int height;

    public CRT(int width, int height) {
        this.width = width;
        this.height = height;
    }

    public boolean[][] render(List<Integer> spritePositions) {
        boolean[][] pixels = new boolean[height][width];
        for (int cycle = 0; cycle < width * height; cycle++) {
            int spriteX = spritePositions.get(cycle);
            int x = cycle % width;
            int y = cycle / width;
            pixels[y][x] = Math.abs(spriteX - x) <= 1;
        }
        return pixels;
    }
}

public class Day10 extends Day {
    private final List<CPU.Instruction> instructions = new ArrayList<>();
    List<Integer> xValues = new ArrayList<>();

    @Override
    public void init() {
        super.init();
        for (String line : lines) {
            if (line.isBlank()) {
                continue;
            }
            String[] parts = line.trim().split("\\s+");
            if (parts.length == 1) {
                instructions.add(new CPU.NoopInstr());
            } else {
                assert parts.length == 2;
                int value = Integer.parseInt(parts[1]);
                instructions.add(new CPU.AddxInstr(value));
            }
        }
        xValues.addAll(new CPU().execute(instructions));
    }

    private List<Integer> getKeyCycles() {
        List<Integer> keyCycles = new ArrayList<>();
        for (int i = 20; i <= 220; i += 40) {
            keyCycles.add(i);
        }
        return keyCycles;
    }

    @Override
    public void part1() {
        System.out.println(getKeyCycles().stream().mapToInt(cycle -> cycle * xValues.get(cycle - 1)).sum());
    }

    @Override
    public void part2() {
        CRT crt = new CRT(40, 6);
        boolean[][] pixels = crt.render(xValues);
        System.out.println();
        for (boolean[] pixel : pixels) {
            for (boolean b : pixel) {
                System.out.print(b ? '#' : '.');
            }
            System.out.println();
        }
    }
}

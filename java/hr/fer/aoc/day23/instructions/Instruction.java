package hr.fer.aoc.day23.instructions;

import hr.fer.aoc.day23.processor.Registers;

public interface Instruction {
	void execute(Registers registers);
}

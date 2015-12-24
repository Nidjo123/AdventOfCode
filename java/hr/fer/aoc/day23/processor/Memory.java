package hr.fer.aoc.day23.processor;

import java.util.ArrayList;
import java.util.List;

import hr.fer.aoc.day23.instructions.Instruction;

public class Memory {
	private List<Instruction> memory;

	public Memory(int size) {
		memory = new ArrayList<>(size);
	}

	public Instruction get(int pos) throws MemoryException {
		if (pos >= memory.size()) {
			throw new MemoryException();
		}

		return memory.get(pos);
	}

	public void set(int pos, Instruction instruction) throws MemoryException {
		if (pos >= memory.size()) {
			throw new MemoryException();
		}

		memory.set(pos, instruction);
	}

	public void load(List<Instruction> instructions) {
		memory.clear();

		memory.addAll(instructions);
	}
}

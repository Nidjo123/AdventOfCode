package hr.fer.aoc.day23.processor;

import java.util.List;

import hr.fer.aoc.day23.instructions.Instruction;

public class Processor {
	private Registers registers;
	private Memory memory;

	private static final int MEMORY_SIZE = 1000;

	public Processor() {
		registers = new Registers();
		memory = new Memory(MEMORY_SIZE);
	}

	public void loadInstructions(List<Instruction> instructions) {
		memory.load(instructions);
	}

	public void start() throws MemoryException {
		while (true) {
			int PC = registers.getPC();
			Instruction instruction = memory.get(PC);
			registers.setPC(PC + 1);
			instruction.execute(registers);
		}
	}

	public void dump() {
		System.out.println("=== DUMP ===");
		System.out.println("Register A: " + registers.getA());
		System.out.println("Register B: " + registers.getB());
		System.out.println("Register PC: " + registers.getPC());
		System.out.println("============");
	}

	public void reset() {
		registers.setA(0);
		registers.setB(0);
		registers.setPC(0);
	}

	public Registers getRegisters() {
		return registers;
	}

	public Memory getMemory() {
		return memory;
	}

}

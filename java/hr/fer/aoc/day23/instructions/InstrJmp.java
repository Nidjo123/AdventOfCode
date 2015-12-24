package hr.fer.aoc.day23.instructions;

import hr.fer.aoc.day23.processor.Registers;

public class InstrJmp implements Instruction {
	private int offset;

	public InstrJmp(int offset) {
		this.offset = offset;
	}

	@Override
	public void execute(Registers registers) {
		int pc = registers.getPC() - 1;
		registers.setPC(pc + offset);
	}

}

package hr.fer.aoc.day23.instructions;

import hr.fer.aoc.day23.processor.Registers;
import hr.fer.aoc.day23.processor.Registers.Register;

public class InstrJie implements Instruction {
	private Register register;
	private int offset;

	public InstrJie(Register register, int offset) {
		this.register = register;
		this.offset = offset;
	}

	@Override
	public void execute(Registers registers) {
		boolean shouldJump = false;

		if (register == Register.A) {
			shouldJump = (registers.getA() & 1) == 0;
		} else if (register == Register.B) {
			shouldJump = (registers.getB() & 1) == 0;
		}

		if (shouldJump) {
			int pc = registers.getPC() - 1;
			registers.setPC(pc + offset);
		}
	}

}

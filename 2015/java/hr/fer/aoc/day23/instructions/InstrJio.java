package hr.fer.aoc.day23.instructions;

import hr.fer.aoc.day23.processor.Registers;
import hr.fer.aoc.day23.processor.Registers.Register;

public class InstrJio implements Instruction {
	private Register register;
	private int offset;

	public InstrJio(Register register, int offset) {
		this.register = register;
		this.offset = offset;
	}

	@Override
	public void execute(Registers registers) {
		boolean shouldJump = false;

		if (register == Register.A) {
			shouldJump = registers.getA() == 1;
		} else if (register == Register.B) {
			shouldJump = registers.getB() == 1;
		}

		if (shouldJump) {
			int pc = registers.getPC() - 1;
			registers.setPC(pc + offset);
		}
	}

}

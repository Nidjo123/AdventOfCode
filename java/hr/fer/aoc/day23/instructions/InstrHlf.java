package hr.fer.aoc.day23.instructions;

import hr.fer.aoc.day23.processor.Registers;
import hr.fer.aoc.day23.processor.Registers.Register;

public class InstrHlf implements Instruction {
	private Register register;

	public InstrHlf(Register register) {
		this.register = register;
	}

	@Override
	public void execute(Registers registers) {
		if (register == Register.A) {
			int a = registers.getA() >> 1;
			registers.setA(a);
		} else if (register == Register.B) {
			int b = registers.getB() >> 1;
			registers.setB(b);
		}
	}

}

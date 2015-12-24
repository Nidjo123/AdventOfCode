package hr.fer.aoc.day23.processor;

public final class Registers {
	private int a;
	private int b;
	private int PC;

	public int getA() {
		return a;
	}

	public void setA(int a) {
		this.a = a;
	}

	public int getB() {
		return b;
	}

	public void setB(int b) {
		this.b = b;
	}

	public int getPC() {
		return PC;
	}

	public void setPC(int pC) {
		PC = pC;
	}

	public enum Register {
		A, B, PC
	}

}

package hr.fer.aoc.day25;

public class Day25 {
	private static final long INITIAL_VALUE = 20151125;
	private static final long MULTIPLY_VALUE = 252533;
	private static final long DIVIDE_VALUE = 33554393;

	private static final int TARGET_ROW = 2947;
	private static final int TARGET_COLUMN = 3029;

	private static long calculateNext(long current) {
		return current * MULTIPLY_VALUE % DIVIDE_VALUE;
	}

	private static long getCode(int targetRow, int targetColumn) {
		int maxRow = 1;
		int row = 1;
		int column = 1;
		long value = INITIAL_VALUE;

		while (row != targetRow || column != targetColumn) {
			row--;
			column++;

			if (row == 0) {
				row = ++maxRow;
				column = 1;
			}

			value = calculateNext(value);
		}

		return value;
	}

	public static void main(String[] args) {
		long code = getCode(TARGET_ROW, TARGET_COLUMN);
		System.out.println(String.format("Code for (%d, %d) is: %d", TARGET_ROW, TARGET_COLUMN, code));
	}

}

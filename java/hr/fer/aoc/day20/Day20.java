package hr.fer.aoc.day20;

import java.util.HashSet;
import java.util.Set;

public class Day20 {
	private final static int PRESENTS = 29_000_000;

	private static int bruteForce(int n) {
		int res = 0;

		int limit = (int) Math.sqrt(n);

		Set<Integer> divisors = new HashSet<>();
		divisors.add(1);
		divisors.add(n);

		for (int i = 2; i <= limit; i++) {
			if (n % i == 0) {
				divisors.add(i);
				int tmp = n;

				while (tmp % i == 0) {
					tmp /= i;
					divisors.add(tmp);
				}
			}
		}

		res = divisors.stream().mapToInt((x) -> x).sum() * 10;

		return res;
	}

	private static int bruteForce2(int n) {
		int res = 0;

		int limit = (int) Math.sqrt(n);

		Set<Integer> divisors = new HashSet<>();
		divisors.add(1);
		divisors.add(n);

		for (int i = 2; i <= limit; i++) {
			if (n % i == 0) {
				if (i * 50 >= n)
					divisors.add(i);

				int tmp = n;

				while (tmp % i == 0) {
					tmp /= i;
					if (tmp * 50 < n)
						continue;
					divisors.add(tmp);
				}
			}
		}

		res = divisors.stream().mapToInt((x) -> x).sum() * 11;

		return res;
	}

	public static void main(String[] args) {
		int n = 10;

		while (bruteForce(n) < PRESENTS) {
			n++;
		}

		System.out.println("n = " + n + ", " + bruteForce(n) + " presents");

		n = 10;

		while (bruteForce2(n) < PRESENTS) {
			n++;
		}

		System.out.println("n = " + n + ", " + bruteForce2(n) + " presents");
	}

}

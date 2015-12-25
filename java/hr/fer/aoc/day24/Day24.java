package hr.fer.aoc.day24;

import java.io.IOException;
import java.net.URISyntaxException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.util.Collections;
import java.util.List;
import java.util.Vector;

public class Day24 {
	private static List<Integer> presents = new Vector<>();
	private static long bestQE;
	private static long currentQE;
	private static List<Integer> passengerCompartment;
	private static List<Integer> bestPassengerCompartment;
	private static int presentsSum;
	private static int compartmentSum;

	private static long calculateQE(List<Integer> presents) {
		return presents.stream()//
				.mapToLong((x) -> x)//
				.reduce(1L, (last, current) -> last *= current);
	}

	private static int listSum(List<Integer> list) {
		return list.stream()//
				.mapToInt((x) -> x)//
				.sum();
	}

	private static void distributePassengerCompartment(List<Integer> a, int targetSize, int index, int nrCompartments) {
		if (index > presents.size() || a.size() > targetSize)
			return;

		int sa = listSum(a);

		if (sa > compartmentSum) {
			return;
		} else if (a.size() == targetSize && sa == compartmentSum) {
			List<Integer> left = new Vector<>(presents);
			left.removeAll(a);

			passengerCompartment = a;
			currentQE = calculateQE(passengerCompartment);
			Vector<List<Integer>> compartments = new Vector<>();
			for (int i = 0; i < nrCompartments; i++)
				compartments.add(new Vector<>());
			distributePresents(left, compartments, 0);
		} else if (index < presents.size()) {
			Integer present = presents.get(index);

			a.add(present);
			distributePassengerCompartment(a, targetSize, index + 1, nrCompartments);
			a.remove(present);

			distributePassengerCompartment(a, targetSize, index + 1, nrCompartments);
		}

	}

	private static void distributePresents(List<Integer> left, List<List<Integer>> compartments, int index) {
		int nrCompartments = compartments.size();
		int[] sums = new int[nrCompartments];

		for (int i = 0; i < nrCompartments; i++) {
			sums[i] = listSum(compartments.get(i));
		}

		if (index >= left.size()) {
			for (int i = 0; i < nrCompartments; i++)
				if (sums[i] != compartmentSum)
					return;

			if (bestPassengerCompartment == null) {
				bestQE = currentQE;
				bestPassengerCompartment = new Vector<>(passengerCompartment);
				return;
			}

			if (passengerCompartment.size() < bestPassengerCompartment.size()) {
				bestQE = currentQE;
				bestPassengerCompartment = new Vector<>(passengerCompartment);
				return;
			}

			if (currentQE < bestQE) {
				bestQE = currentQE;
				bestPassengerCompartment = new Vector<>(passengerCompartment);
			}

			return;
		}

		Integer present = left.get(index);

		for (int i = 0; i < nrCompartments; i++) {
			if (sums[i] + present <= compartmentSum) {
				compartments.get(i).add(present);
				distributePresents(left, compartments, index + 1);
				compartments.get(i).remove(present);
			}
		}
	}

	public static void main(String[] args) throws URISyntaxException, IOException {
		Path path = Paths.get(Day24.class.getResource("24.input").toURI());

		List<String> lines = Files.readAllLines(path);

		lines.forEach((line) -> presents.add(Integer.parseInt(line)));

		Collections.reverse(presents);

		presentsSum = listSum(presents);
		compartmentSum = presentsSum / 3;

		// last param should be (nr_of_compartments - 1)
		distributePassengerCompartment(new Vector<>(), 6, 0, 2);

		System.out.println("Min QE: " + bestQE);

		bestPassengerCompartment.forEach(System.out::println);
	}

}

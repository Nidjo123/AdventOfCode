package hr.fer.aoc.day19;

import java.io.IOException;
import java.net.URISyntaxException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.util.ArrayList;
import java.util.Collections;
import java.util.HashSet;
import java.util.List;
import java.util.Set;
import java.util.Stack;

public class Day19 {

	private static class Replacement {
		private String pattern;
		private String replacement;

		public Replacement(String pattern, String replacement) {
			this.pattern = pattern;
			this.replacement = replacement;
		}

		public Replacement(String[] replacement) {
			this.pattern = replacement[0];
			this.replacement = replacement[1];
		}

		public String getPattern() {
			return pattern;
		}

		public String getReplacement() {
			return replacement;
		}

		@Override
		public String toString() {
			return pattern + " => " + replacement;
		}
	}

	public static List<Replacement> inverse(List<Replacement> replacements) {
		List<Replacement> result = new ArrayList<>();

		for (Replacement replacement : replacements) {
			result.add(new Replacement(replacement.getReplacement(), replacement.getPattern()));
		}

		return result;
	}

	private static class State {
		private String molecule;
		private int steps;

		public State(String molecule, int steps) {
			this.molecule = molecule;
			this.steps = steps;
		}

		public String getMolecule() {
			return molecule;
		}

		public int getSteps() {
			return steps;
		}

		@Override
		public String toString() {
			return "Steps: " + steps + ", " + molecule;
		}
	}

	// too slow... :(
	private static int reduceMolecule(String molecule, List<Replacement> replacements) {
		int res = Integer.MAX_VALUE;
		Set<String> visited = new HashSet<>();
		Stack<State> DFS = new Stack<>();

		Collections.shuffle(replacements);

		DFS.add(new State(molecule, 0));

		while (DFS.size() > 0) {

			// System.out.println(BFS.size());
			State state = DFS.pop();
			// System.out.println(state);

			String tmp = state.getMolecule();
			int steps = state.getSteps();

			visited.add(tmp);

			for (Replacement r : replacements) {
				if (tmp.contains(r.getPattern())) {
					String newMolecule = tmp.replace(r.getPattern(), r.getReplacement());

					int d = (r.getPattern().length() - r.getReplacement().length());

					if (d == 0)
						d = r.getPattern().length();
					steps += (tmp.length() - newMolecule.length()) / d;
					if (newMolecule.equals("e")) {
						System.out.println(tmp + " " + newMolecule);
						return steps;
					}

					if (!visited.contains(newMolecule)) {
						DFS.add(new State(newMolecule, steps));
					} else {
						Collections.shuffle(replacements);
					}
				}
			}
		}

		return res;
	}

	/**
	 * Thank you Andrew Skalski!
	 * 
	 * https://www.reddit.com/r/adventofcode/comments/3xflz8/day_19_solutions/
	 * cy4etju
	 * 
	 */
	private static int minimumSteps(String molecule) {
		int molecules = (int) molecule.chars().filter((c) -> Character.isUpperCase(c)).count();
		int ys = (int) molecule.chars().filter((c) -> (char) c == 'Y').count();
		int arRn = 0;
		int start = 0, pos;

		while ((pos = molecule.indexOf("Ar", start)) >= 0) {
			arRn++;
			start = pos + 1;
		}

		return molecules - 2 * arRn - 2 * ys - 1;
	}

	public static void main(String[] args) throws URISyntaxException {
		Path inputPath = Paths.get(Day19.class.getResource("19.input").toURI());
		List<String> lines = null;
		List<Replacement> replacements = new ArrayList<>();

		try {
			lines = Files.readAllLines(inputPath);

			for (String line : lines) {
				if (line.isEmpty()) {
					break;
				}

				replacements.add(new Replacement(line.split(" => ")));
			}
		} catch (IOException e) {
			e.printStackTrace();
			System.exit(1);
		}

		Set<String> newMolecules = new HashSet<>();

		String molecule = lines.get(lines.size() - 1);
		for (Replacement replacement : replacements) {
			String pattern = replacement.getPattern();
			int startIndex = molecule.indexOf(replacement.getPattern());
			int patternLength = replacement.getPattern().length();

			while (startIndex >= 0) {
				String result = molecule.substring(0, startIndex) + replacement.getReplacement()
						+ molecule.substring(startIndex + patternLength);

				newMolecules.add(result);

				startIndex = molecule.indexOf(pattern, startIndex + 1);
			}
		}

		System.out.println("First part: " + newMolecules.size());

		System.out.println("Second part: " + minimumSteps(molecule));
	}
}

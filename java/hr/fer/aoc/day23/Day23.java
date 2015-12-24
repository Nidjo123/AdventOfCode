package hr.fer.aoc.day23;

import java.io.IOException;
import java.net.URISyntaxException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.util.ArrayList;
import java.util.List;

import hr.fer.aoc.day23.instructions.InstrHlf;
import hr.fer.aoc.day23.instructions.InstrInc;
import hr.fer.aoc.day23.instructions.InstrJie;
import hr.fer.aoc.day23.instructions.InstrJio;
import hr.fer.aoc.day23.instructions.InstrJmp;
import hr.fer.aoc.day23.instructions.InstrTpl;
import hr.fer.aoc.day23.instructions.Instruction;
import hr.fer.aoc.day23.processor.MemoryException;
import hr.fer.aoc.day23.processor.Processor;
import hr.fer.aoc.day23.processor.Registers.Register;

public class Day23 {

	private static Register getRegister(String s) {
		s = s.toLowerCase();

		if (s.equals("a")) {
			return Register.A;
		} else if (s.equals("b")) {
			return Register.B;
		}

		return null;
	}

	private static List<Instruction> loadInstructions(Path path) throws IOException {
		List<Instruction> instructions = new ArrayList<>();
		List<String> lines = Files.readAllLines(path);

		for (String line : lines) {
			String[] tokens = line.split(" ");
			Instruction instruction = null;

			switch (tokens[0].toLowerCase()) {
			case "inc":
				instruction = new InstrInc(getRegister(tokens[1]));
				break;
			case "hlf":
				instruction = new InstrHlf(getRegister(tokens[1]));
				break;
			case "tpl":
				instruction = new InstrTpl(getRegister(tokens[1]));
				break;
			case "jmp":
				instruction = new InstrJmp(Integer.parseInt(tokens[1]));
				break;
			case "jio":
				// remove comma
				tokens[1] = tokens[1].substring(0, tokens[1].length() - 1);
				instruction = new InstrJio(getRegister(tokens[1]), Integer.parseInt(tokens[2]));
				break;
			case "jie":
				// remove comma
				tokens[1] = tokens[1].substring(0, tokens[1].length() - 1);
				instruction = new InstrJie(getRegister(tokens[1]), Integer.parseInt(tokens[2]));
				break;
			default:
				System.out.println("Unknown instruction: " + line);
				break;
			}

			instructions.add(instruction);
		}

		return instructions;
	}

	public static void main(String[] args) throws IOException, URISyntaxException {
		Processor processor = new Processor();

		List<Instruction> instructions = loadInstructions(Paths.get(Day23.class.getResource("23.input").toURI()));
		processor.loadInstructions(instructions);

		try {
			processor.start();
		} catch (MemoryException e) {
			processor.dump();
		}

		processor.reset();
		processor.loadInstructions(instructions);
		processor.getRegisters().setA(1);

		try {
			processor.start();
		} catch (MemoryException e) {
			processor.dump();
		}
	}

}

package hr.fer.aoc.day21;

import java.util.ArrayList;
import java.util.HashSet;
import java.util.List;
import java.util.Set;

import hr.fer.aoc.day21.Game.Winner;
import hr.fer.aoc.day21.items.Shop;

public class Day21 {

	private static Set<String> generatePairs(int max) {
		Set<String> res = new HashSet<>();

		res.add("00");

		for (int i = 0; i <= max; i++) {
			for (int j = i + 1; j <= max; j++) {
				res.add("" + i + j);
			}
		}

		return res;
	}

	private static Set<String> generateSetups(String setup, int level, Set<String> res) {
		if (level > 3) {
			res.add(setup);
			return res;
		}

		if (level == 0) {
			// generate weapons
			for (int i = 1; i <= Shop.getAvailableWeapons().size(); i++) {
				generateSetups(setup + i, level + 1, res);
			}
		} else if (level == 1) {
			// generate armor
			for (int i = 0; i <= Shop.getAvailableArmor().size(); i++) {
				generateSetups(setup + i, level + 1, res);
			}
		} else if (level == 2) {
			// generate rings
			Set<String> pairs = generatePairs(Shop.getAvailableRings().size());
			for (String ringPair : pairs) {
				res.add(setup + ringPair);
			}
		}

		return res;
	}

	public static void playGame() {
		Game game = new Game();
		Shop shop = game.getShop();
		int minGoldSpentAndWon = Integer.MAX_VALUE;
		int maxGoldSpentAndLost = 0;

		List<String> weapons = new ArrayList<>(Shop.getAvailableWeapons());
		List<String> armor = new ArrayList<>(Shop.getAvailableArmor());
		List<String> rings = new ArrayList<>(Shop.getAvailableRings());

		Set<String> setups = generateSetups("", 0, new HashSet<String>());

		for (String setup : setups) {
			int weaponIndex = Integer.parseInt(setup.substring(0, 1)) - 1;
			int armorIndex = Integer.parseInt(setup.substring(1, 2)) - 1;
			int ring1Index = Integer.parseInt(setup.substring(2, 3)) - 1;
			int ring2Index = Integer.parseInt(setup.substring(3, 4)) - 1;

			shop.buyWeapon(weapons.get(weaponIndex), game.getPlayer());

			if (armorIndex >= 0)
				shop.buyArmor(armor.get(armorIndex), game.getPlayer());

			if (ring1Index >= 0)
				shop.buyRing(rings.get(ring1Index), game.getPlayer());

			if (ring2Index >= 0)
				shop.buyRing(rings.get(ring2Index), game.getPlayer());

			if (game.play() == Winner.PLAYER) {
				minGoldSpentAndWon = Math.min(minGoldSpentAndWon, shop.getGoldSpent());
			} else {
				maxGoldSpentAndLost = Math.max(maxGoldSpentAndLost, shop.getGoldSpent());
			}

			game.reset();
		}

		System.out.println("Min gold spent and won: " + minGoldSpentAndWon);
		System.out.println("Max gold spent and lost: " + maxGoldSpentAndLost);
	}

	public static void main(String[] args) {
		playGame();
	}

}

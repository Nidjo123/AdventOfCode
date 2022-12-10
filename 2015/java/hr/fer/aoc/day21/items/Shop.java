package hr.fer.aoc.day21.items;

import java.util.HashMap;
import java.util.Map;
import java.util.Set;

import hr.fer.aoc.day21.entities.Player;

public class Shop {
	private static Map<String, Item> weapons = new HashMap<>();
	private static Map<String, Item> armor = new HashMap<>();
	private static Map<String, Item> rings = new HashMap<>();

	static {
		addWeapon(new Item("Dagger", 8, 4, 0));
		addWeapon(new Item("Shortsword", 10, 5, 0));
		addWeapon(new Item("Warhammer", 25, 6, 0));
		addWeapon(new Item("Longsword", 40, 7, 0));
		addWeapon(new Item("Greataxe", 74, 8, 0));

		addArmor(new Item("Leather", 13, 0, 1));
		addArmor(new Item("Chainmail", 31, 0, 2));
		addArmor(new Item("Splintmail", 53, 0, 3));
		addArmor(new Item("Bandedmail", 75, 0, 4));
		addArmor(new Item("Platemail", 102, 0, 5));

		addRing(new Item("Damage +1", 25, 1, 0));
		addRing(new Item("Damage +2", 50, 2, 0));
		addRing(new Item("Damage +3", 100, 3, 0));
		addRing(new Item("Defense +1", 20, 0, 1));
		addRing(new Item("Defense +2", 40, 0, 2));
		addRing(new Item("Defense +3", 80, 0, 3));
	}

	private int goldSpent;

	public static void addWeapon(Item weapon) {
		weapons.put(weapon.getName(), weapon);
	}

	public static void addArmor(Item armorItem) {
		armor.put(armorItem.getName(), armorItem);
	}

	public static void addRing(Item ring) {
		rings.put(ring.getName(), ring);
	}

	public void buyWeapon(String weaponName, Player player) {
		Item weapon = weapons.get(weaponName);
		goldSpent += weapon.getCost();
		player.useItem(weapon);
	}

	public void buyArmor(String armorName, Player player) {
		Item armorItem = armor.get(armorName);
		goldSpent += armorItem.getCost();
		player.useItem(armorItem);
	}

	public void buyRing(String ringName, Player player) {
		Item ring = rings.get(ringName);
		goldSpent += ring.getCost();
		player.useItem(ring);
	}

	public static Set<String> getAvailableWeapons() {
		return weapons.keySet();
	}

	public static Set<String> getAvailableArmor() {
		return armor.keySet();
	}

	public static Set<String> getAvailableRings() {
		return rings.keySet();
	}

	public int getGoldSpent() {
		return goldSpent;
	}

	public void reset() {
		goldSpent = 0;
	}
}

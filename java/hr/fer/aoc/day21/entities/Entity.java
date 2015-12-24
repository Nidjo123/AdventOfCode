package hr.fer.aoc.day21.entities;

import hr.fer.aoc.day21.items.Item;

public abstract class Entity {
	private int health;
	private int armor;
	private int damage;

	public Entity(int health, int armor, int damage) {
		this.health = health;
		this.armor = armor;
		this.damage = damage;
	}

	public void useItem(Item item) {
		armor += item.getArmor();
		damage += item.getDamage();
	}

	public void attack(Entity other) {
		other.health -= Math.max(1, damage - other.armor);
	}

	public int getHealth() {
		return health;
	}

	public int getArmor() {
		return armor;
	}

	public int getDamage() {
		return damage;
	}

}

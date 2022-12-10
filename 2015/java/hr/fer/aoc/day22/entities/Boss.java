package hr.fer.aoc.day22.entities;

public class Boss extends Entity {

	public Boss(int health, int armor, int damage) {
		super(health, armor, damage, 0);
	}

	public Boss(Boss boss) {
		super(boss);
	}
}

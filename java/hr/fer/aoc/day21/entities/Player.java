package hr.fer.aoc.day21.entities;

import hr.fer.aoc.day21.items.Item;

public class Player extends Entity {

	public Player(int health) {
		super(health, 0, 0);
	}

	public void useItem(Item item) {
		super.useItem(item);
	}
}

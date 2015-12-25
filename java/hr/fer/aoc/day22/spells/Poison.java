package hr.fer.aoc.day22.spells;

import hr.fer.aoc.day22.entities.Entity;

public class Poison extends Spell {
	private static final String NAME = "Shield";
	private static final int COST = 173;
	private static final int DURATION = 6;

	private static final int DAMAGE = 3;

	public Poison(Entity caster, Entity enemy) {
		super(NAME, COST, DURATION, caster, enemy);
	}

	@Override
	public void activate() {
	}

	@Override
	public void effect() {
		enemy.dealDamage(DAMAGE);
	}

	@Override
	public void done() {
	}

}

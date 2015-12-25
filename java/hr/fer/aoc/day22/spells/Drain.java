package hr.fer.aoc.day22.spells;

import hr.fer.aoc.day22.entities.Entity;

public class Drain extends Spell {
	private static final String NAME = "Drain";
	private static final int COST = 73;
	private static final int DURATION = 0;

	private static final int DAMAGE = 2;
	private static final int HEAL_AMOUNT = 2;

	public Drain(Entity caster, Entity enemy) {
		super(NAME, COST, DURATION, caster, enemy);
	}

	@Override
	public void activate() {
		caster.heal(HEAL_AMOUNT);
		enemy.dealDamage(DAMAGE);
	}

	@Override
	public void effect() {
	}

	@Override
	public void done() {
	}

}

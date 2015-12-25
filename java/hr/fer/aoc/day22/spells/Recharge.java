package hr.fer.aoc.day22.spells;

import hr.fer.aoc.day22.entities.Entity;

public class Recharge extends Spell {
	private static final String NAME = "Recharge";
	private static final int COST = 229;
	private static final int DURATION = 5;

	private static final int AMOUNT = 101;

	public Recharge(Entity caster, Entity enemy) {
		super(NAME, COST, DURATION, caster, enemy);
	}

	@Override
	public void activate() {
	}

	@Override
	public void effect() {
		caster.changeMana(AMOUNT);
	}

	@Override
	public void done() {
	}

}

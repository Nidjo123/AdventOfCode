package hr.fer.aoc.day22.spells;

import hr.fer.aoc.day22.entities.Entity;

public class Shield extends Spell {
	private static final String NAME = "Poison";
	private static final int COST = 113;
	private static final int DURATION = 6;

	private static final int ARMOR = 7;

	public Shield(Entity caster, Entity enemy) {
		super(NAME, COST, DURATION, caster, enemy);
	}

	@Override
	public void activate() {
		caster.changeArmor(ARMOR);
	}

	@Override
	public void effect() {
	}

	@Override
	public void done() {
		caster.changeArmor(-ARMOR);
	}

}

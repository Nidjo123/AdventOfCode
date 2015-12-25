package hr.fer.aoc.day22.spells;

import hr.fer.aoc.day22.entities.Entity;

public class MagicMissile extends Spell {
	private static final String NAME = "Magic Missile";
	private static final int COST = 53;
	private static final int DURATION = 0;

	private static final int DAMAGE = 4;

	public MagicMissile(Entity caster, Entity enemy) {
		super(NAME, COST, DURATION, caster, enemy);
	}

	@Override
	public void activate() {
		enemy.dealDamage(DAMAGE);
	}

	@Override
	public void effect() {
	}

	@Override
	public void done() {
	}

}

package hr.fer.aoc.day22.spells;

import hr.fer.aoc.day22.entities.Entity;

public abstract class Spell {
	private final String name;
	private final int cost;
	private int duration;

	protected Entity caster;
	protected Entity enemy;

	public Spell(String name, int cost, int duration, Entity caster, Entity enemy) {
		this.name = name;
		this.cost = cost;
		this.duration = duration;
		this.caster = caster;
		this.enemy = enemy;
	}

	public Spell(Spell other) {
		this.name = other.name;
		this.cost = other.cost;
		this.duration = other.duration;
		this.caster = other.caster;
		this.enemy = other.enemy;
	}

	public abstract void activate();

	public abstract void effect();

	public abstract void done();

	public void tick() {
		effect();
		duration--;

		if (isDone()) {
			done();
			caster.spellToRemove(this);
		}
	}

	public boolean isDone() {
		return duration <= 0;
	}

	public int getDuration() {
		return duration;
	}

	public void setDuration(int duration) {
		this.duration = duration;
	}

	public int getCost() {
		return cost;
	}

	public void changeCaster(Entity caster) {
		this.caster = caster;
	}

	public void changeEnemy(Entity enemy) {
		this.enemy = enemy;
	}

	@Override
	public boolean equals(Object obj) {
		if (obj == null)
			return false;

		if (obj instanceof Spell) {
			Spell spell = (Spell) obj;

			return spell.name.equals(name);
		}

		return false;
	}

	@Override
	public int hashCode() {
		return name.hashCode();
	}

	@Override
	public String toString() {
		return name;
	}
}

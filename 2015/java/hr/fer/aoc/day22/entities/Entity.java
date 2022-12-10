package hr.fer.aoc.day22.entities;

import java.util.HashSet;
import java.util.Set;

import hr.fer.aoc.day22.spells.Spell;

public abstract class Entity {
	private int health;
	private int armor;
	private int damage;
	private int mana;
	private int manaSpent;

	protected Set<Spell> activeSpells = new HashSet<>();
	private Set<Spell> toRemove = new HashSet<>();

	public Entity(int health, int armor, int damage, int mana) {
		this.health = health;
		this.armor = armor;
		this.damage = damage;
		this.mana = mana;
	}

	public Entity(Entity other) {
		this.health = other.health;
		this.armor = other.armor;
		this.damage = other.damage;
		this.mana = other.mana;
		this.manaSpent = other.manaSpent;
		this.activeSpells = new HashSet<>(other.activeSpells);
	}

	public void updateSpells() {
		for (Spell spell : activeSpells) {
			spell.tick();
		}

		activeSpells.removeAll(toRemove);
		toRemove.clear();
	}

	public void spellToRemove(Spell spell) {
		toRemove.add(spell);
	}

	public void attack(Entity other) {
		other.health -= Math.max(1, damage - other.armor);
	}

	public void castSpell(Spell spell) {
		if (spell.getCost() > mana) {
			health = 0;
			return;
		}

		activeSpells.add(spell);
		mana -= spell.getCost();
		manaSpent += spell.getCost();
		spell.activate();
	}

	public boolean hasSpell(Spell spell) {
		return activeSpells.contains(spell);
	}

	public void dealDamage(int amount) {
		health -= amount;
	}

	public void heal(int amount) {
		health += amount;
	}

	public void changeArmor(int amount) {
		armor += amount;
	}

	public void changeMana(int amount) {
		mana += amount;
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

	public int getManaSpent() {
		return manaSpent;
	}

	public Set<Spell> getActiveSpells() {
		return activeSpells;
	}

}

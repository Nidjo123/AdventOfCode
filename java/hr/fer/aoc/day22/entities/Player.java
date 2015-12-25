package hr.fer.aoc.day22.entities;

import java.util.stream.Collectors;

import hr.fer.aoc.day22.spells.Spell;

public class Player extends Entity {

	public Player(int health, int mana) {
		super(health, 0, 0, mana);
	}

	public Player(Player player, Boss boss) {
		super(player);

		activeSpells = activeSpells.stream()//
				.map(spell -> {
					Spell newSpell = null;
					try {
						newSpell = spell.getClass().getDeclaredConstructor(Entity.class, Entity.class).newInstance(this,
								boss);
					} catch (Exception e) {
						e.printStackTrace();
					}
					newSpell.changeCaster(this);
					newSpell.changeEnemy(boss);
					newSpell.setDuration(spell.getDuration());

					return newSpell;
				})//
				.collect(Collectors.toSet());
	}
}

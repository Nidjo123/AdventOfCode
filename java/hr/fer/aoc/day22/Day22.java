package hr.fer.aoc.day22;

import java.lang.reflect.InvocationTargetException;
import java.util.List;
import java.util.stream.Collectors;

import hr.fer.aoc.day22.GameState.Identifier;
import hr.fer.aoc.day22.entities.Boss;
import hr.fer.aoc.day22.entities.Entity;
import hr.fer.aoc.day22.entities.Player;
import hr.fer.aoc.day22.spells.Drain;
import hr.fer.aoc.day22.spells.MagicMissile;
import hr.fer.aoc.day22.spells.Poison;
import hr.fer.aoc.day22.spells.Recharge;
import hr.fer.aoc.day22.spells.Shield;
import hr.fer.aoc.day22.spells.Spell;

public class Day22 {
	private static final int PLAYER_HEALTH = 50;
	private static final int PLAYER_MANA = 500;

	private static final int BOSS_HEALTH = 55;
	private static final int BOSS_DAMAGE = 8;

	private static int minManaWon = Integer.MAX_VALUE;

	private static boolean hard = false;

	private static Class[] spellClasses = { MagicMissile.class, Drain.class, Poison.class, Recharge.class,
			Shield.class };

	private static void setManaIfLess(int mana) {
		if (mana < minManaWon) {
			minManaWon = mana;
		}
	}

	private static void nextTurn(GameState state) throws InstantiationException, IllegalAccessException,
			IllegalArgumentException, NoSuchMethodException, SecurityException, InvocationTargetException {
		Player player = state.getPlayer();
		Boss boss = state.getBoss();
		Identifier lastPlayed = state.getLastPlayed();

		if (player.getManaSpent() > minManaWon)
			return;

		List<Class> castSpellsClasses = player.getActiveSpells().stream()//
				.map((spell) -> spell.getClass())//
				.collect(Collectors.toList());

		if (hard && lastPlayed == Identifier.BOSS) {
			// if hard difficulty and player's turn, deal one damage to him
			player.dealDamage(1);
			if (player.getHealth() <= 0)
				return;
		}

		// every turn spells should be updated
		player.updateSpells();

		// check if player has won
		if (state.isDone()) {
			setManaIfLess(player.getManaSpent());

			return; // done
		}

		if (lastPlayed == Identifier.BOSS) {
			// then it's player's turn
			// he must cast a spell
			for (Class spellClass : spellClasses) {
				Boss tmpBoss = new Boss(boss);
				Player tmpPlayer = new Player(player, tmpBoss);

				Spell spell = (Spell) spellClass.getDeclaredConstructor(Entity.class, Entity.class)
						.newInstance(tmpPlayer, tmpBoss);

				if (tmpPlayer.hasSpell(spell))
					continue;

				tmpPlayer.castSpell(spell);

				if (tmpPlayer.getHealth() <= 0)
					// probably out of mana
					return;

				// go to next turn, last played is player
				nextTurn(new GameState(tmpPlayer, tmpBoss, Identifier.PLAYER));
			}
		} else if (lastPlayed == Identifier.PLAYER) {
			// boss should attack player

			boss.attack(player);

			if (state.isDone())
				return; // done

			Boss tmpBoss = new Boss(boss);
			Player tmpPlayer = new Player(player, tmpBoss);

			nextTurn(new GameState(tmpPlayer, tmpBoss, Identifier.BOSS));
		}
	}

	public static void main(String[] args) throws InstantiationException, IllegalAccessException,
			IllegalArgumentException, InvocationTargetException, NoSuchMethodException, SecurityException {
		GameState initialState = new GameState(new Player(PLAYER_HEALTH, PLAYER_MANA),
				new Boss(BOSS_HEALTH, 0, BOSS_DAMAGE), Identifier.BOSS);
		nextTurn(initialState);

		System.out.println("Difficulty: easy");
		System.out.println("Minimum mana needed to win: " + minManaWon);

		hard = true;
		minManaWon = Integer.MAX_VALUE;
		initialState = new GameState(new Player(PLAYER_HEALTH, PLAYER_MANA), new Boss(BOSS_HEALTH, 0, BOSS_DAMAGE),
				Identifier.BOSS);
		nextTurn(initialState);

		System.out.println("Difficulty: hard");
		System.out.println("Minimum mana needed to win: " + minManaWon);
	}

}

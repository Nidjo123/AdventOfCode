package hr.fer.aoc.day22;

import hr.fer.aoc.day22.entities.Player;
import hr.fer.aoc.day22.entities.Boss;

public class GameState {
	private Player player;
	private Boss boss;
	private Identifier lastPlayed;

	private Identifier winner = null;

	public enum Identifier {
		PLAYER, BOSS
	}

	public GameState(Player player, Boss boss, Identifier lastPlayed) {
		this.player = player;
		this.boss = boss;
		this.lastPlayed = lastPlayed;
	}

	public void nextTurn() {
		if (isDone()) {
			return;
		}

		player.updateSpells();

		if (isDone()) {
			winner = Identifier.PLAYER;
		}

		System.out.println("--- Boss' turn ---");
		System.out.println(String.format("Boss has %d health", boss.getHealth()));
		player.updateSpells();

		if (isDone()) {
			winner = Identifier.PLAYER;
		}

		boss.attack(player);

		if (isDone()) {
			winner = Identifier.BOSS;
		}
	}

	public boolean isDone() {
		return player.getHealth() <= 0 || boss.getHealth() <= 0;
	}

	public Identifier getWinner() {
		return winner;
	}

	public Player getPlayer() {
		return player;
	}

	public Boss getBoss() {
		return boss;
	}

	public Identifier getLastPlayed() {
		return lastPlayed;
	}

}

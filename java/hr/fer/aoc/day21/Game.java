package hr.fer.aoc.day21;

import hr.fer.aoc.day21.entities.Boss;
import hr.fer.aoc.day21.entities.Player;
import hr.fer.aoc.day21.items.Shop;

public class Game {
	private Player player;
	private Boss boss;
	private Shop shop;

	private static final int PLAYER_HEALTH = 100;

	private static final int BOSS_HEALTH = 100;
	private static final int BOSS_DAMAGE = 8;
	private static final int BOSS_ARMOR = 2;

	public enum Winner {
		PLAYER, BOSS
	}

	public Game() {
		player = new Player(PLAYER_HEALTH);
		boss = new Boss(BOSS_HEALTH, BOSS_ARMOR, BOSS_DAMAGE);
		shop = new Shop();
	}

	public void reset() {
		player = new Player(PLAYER_HEALTH);
		boss = new Boss(BOSS_HEALTH, BOSS_ARMOR, BOSS_DAMAGE);
		shop.reset();
	}

	public boolean done() {
		return player.getHealth() <= 0 || boss.getHealth() <= 0;
	}

	public Winner play() {
		while (!done()) {
			player.attack(boss);
			if (done()) {
				return Winner.PLAYER;
			}
			boss.attack(player);
		}

		return Winner.BOSS;
	}

	public Shop getShop() {
		return shop;
	}

	public Player getPlayer() {
		return player;
	}

}

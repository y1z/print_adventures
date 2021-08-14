use pra_enemy::enemy;
use pra_player::player;
use vector2;

/// functions that every character should have.
pub trait CommonCharacterBahavoir {
  fn get_name(&self) -> String;
  fn get_stats_string(&self) -> String;
  fn get_position(&self) -> vector2i;
  fn get_size(&self) -> vector2i;
}

/// Trait any character capable of attacking the player
pub trait AttackPlayer {
  fn attack_player(&self, _player: &mut pra_player::player);
}

/// Trait any character capable of attacking an enemy
pub trait AttackEnemy {
  fn attack_enemy(&self, _enemy: &mut pra_enemy::enemy);
}

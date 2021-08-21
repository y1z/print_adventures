use crate::pra_enemy::enemy;
use crate::pra_player::player;
use crate::vector2::Vector2i;

/// functions that every character should have.
pub trait CommonCharacterBahavoir {
  fn get_name(&self) -> String;
  fn get_stats_string(&self) -> String;
  fn get_position(&self) -> Vector2i;
  fn get_size(&self) -> Vector2i;
}

/// Trait any character capable of attacking the player
pub trait AttackPlayer {
  fn attack_player(&self, _player: &mut player);
}

/// Trait any character capable of attacking an enemy
pub trait AttackEnemy {
  fn attack_enemy(&self, _enemy: &mut enemy);
}

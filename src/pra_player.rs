use crate::common_character_data::commonCharacterData;
use crate::common_character_trait::*;
use crate::pra_enemy;
use crate::vector2::Vector2i;

/// Represent the player
#[derive(Debug, Clone)]
pub struct player {
  pub m_data: commonCharacterData,
}

impl player {
  /// creates a custom instance of a player
  pub fn create(character_data: commonCharacterData) -> player {
    player {
      m_data: character_data,
    }
  }

  /// creates a default instance of a player
  pub fn new() -> player {
    player::create(commonCharacterData::new())
  }

  /// calculates the damage the player will do
  pub fn calculate_damage(&self, enemy: &mut pra_enemy::enemy) -> i32 {
    self.m_data.m_attack - enemy.m_data.m_defense
  }
}

impl AttackEnemy for player {
  fn attack_enemy(&self, _enemy: &mut pra_enemy::enemy) {
    let damage = self.calculate_damage(_enemy);
    if damage > 0 {
      _enemy.m_data.m_health = _enemy.m_data.m_health - damage;
    }
  }
}

impl CommonCharacterBahavoir for player {
  fn get_name(&self) -> String {
    self.m_data.m_name.clone()
  }

  fn get_stats_string(&self) -> String {
    String::from(format!(
      "health : {} \nattack : {}\ndefence : {}\n",
      self.m_data.m_health, self.m_data.m_attack, self.m_data.m_defense
    ))
  }

  fn get_position(&self) -> Vector2i {
    self.m_data.m_pos
  }

  fn get_size(&self) -> Vector2i {
    self.m_data.m_size
  }
}

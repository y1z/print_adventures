use crate::common_character_data::commonCharacterData;
use crate::common_character_trait::AttackEnemy;
use crate::pra_enemy;

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

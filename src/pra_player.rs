use common_character_data;
use common_character_trait::AttackEnemy;
use pra_enemy;
/// Represent the player
struct player {
  m_data: common_character_data,
}

impl player {
  /// calculates the damage the player will do
  pub fn calculate_damage(enemy: &pra_enemy::enemy) -> i32 {
    self.m_data.m_attack - nemy.m_data.m_defense
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

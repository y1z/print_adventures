use common_character_data;
use common_character_trait::AttackEnemy;
use pra_enemy;
/// Represent the player
struct player {
  m_data: common_character_data,
}

impl AttackEnemy for player {
  fn attack_enemy(&self, _enemy: &mut pra_enemy::enemy) {
    let damage = self.m_data.m_attack - _enemy.m_data.m_defense;
    if damage > 0 {
      _enemy.m_data.m_health = _enemy.m_data.m_health - damage;
    }
  }
}

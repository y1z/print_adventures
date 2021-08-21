use crate::vector2::Vector2i;
/// represents data that every character (player and npc's) contains in the game.
#[derive(Debug, Clone)]
pub struct commonCharacterData {
  pub m_name: String,
  pub m_pos: Vector2i,
  pub m_size: Vector2i,
  pub m_health: i32,
  pub m_attack: i32,
  pub m_defense: i32,
}

impl commonCharacterData {
  /// creates a custom instance of player data
  pub fn create(
    name: String,
    position: Vector2i,
    size: Vector2i,
    health: i32,
    attack: i32,
    defense: i32,
  ) -> commonCharacterData {
    commonCharacterData {
      m_name: name,
      m_pos: position,
      m_size: size,
      m_health: health,
      m_attack: attack,
      m_defense: defense,
    }
  }

  /// create the default instance
  pub fn new() -> commonCharacterData {
    commonCharacterData::create(
      String::from("defualt"),
      Vector2i { x: 0, y: 0 },
      Vector2i { x: 1, y: 1 },
      100,
      12,
      6,
    )
  }
}

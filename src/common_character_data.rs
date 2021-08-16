use crate::vector2::Vector2i;
/// represents data that every character (player and npc's) contains in the game.
#[derive(Clone)]
struct commonCharacterData {
  m_name: String,
  m_pos: Vector2i,
  m_size: Vector2i,
  m_health: i32,
  m_attack: i32,
  m_defense: i32,
}

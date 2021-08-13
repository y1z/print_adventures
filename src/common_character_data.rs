use crate::vector2::vector2i;
/// represents data that every character (player and npc's) contains in the game.
struct commonCharacterData {
  m_name: String,
  m_pos: vector2i,
  m_size: vector2i,
  m_health: i32,
  m_attack: i32,
  m_defense: i32,
}

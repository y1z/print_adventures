use pra_player;
use vector2;

pub trait Character {
  fn get_name(&self) -> String;
  fn get_stats_string(&self) -> String;
  fn get_position(&self) -> vector2i;
  fn get_size(&self) -> vector2i;
}

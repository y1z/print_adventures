mod error_types;
mod util {
  pub mod util_traits;
}
pub mod common_character_data;
pub mod common_character_trait;
mod game_grid;
mod pra_enemy;
mod pra_game_data;
mod pra_player;
mod vector2;
use error_types::{errorTypes, mainReturn};
use game_grid::gameGrid;
use pra_game_data::gameData;
use pra_player::player;

const INTRO_TEXT: &'static str = "hello and welcome to the world of print ventures";
const FILE_NAME: &'static str = "../resource/test_level";
const DEFAULT_WIDTH: u16 = 21;
const DEFAULT_HEIGHT: u16 = 10;

fn main() -> mainReturn {
  let possible_error = run();
  if possible_error.is_err() {
    let a = possible_error.unwrap_err();
    print!("{}", a.desc());
    return Err(a);
  }
  return possible_error;
}

fn run() -> mainReturn {
  println!("\n{}", INTRO_TEXT);
  let mut game_data = init();
  game_data.m_grid.insert_player(&game_data.m_player);
  game_data.m_grid.print_grid();
  Ok(())
}

fn init() -> gameData {
  let grid = gameGrid::create(String::from(""), DEFAULT_WIDTH, DEFAULT_HEIGHT);
  let _player = player::new();
  let mut game_data = gameData::create(grid, _player);
  let _test_file_path = std::fs::File::create(FILE_NAME).unwrap();
  game_data.init(DEFAULT_WIDTH, DEFAULT_HEIGHT, _test_file_path);
  game_data
}

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
use std::io::Read;
use std::path::Path;

const INTRO_TEXT: &'static str = "hello and welcome to the world of print ventures";
const DIRECTORY: &'static str = "resource/";
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

  let mut input = std::io::stdin();
  let mut buffer = String::new();
  loop {
    buffer.clear();
    input.read_line(&mut buffer).unwrap();
    game_data.m_grid.print_grid();

    println!("buffer = [ {} ]", buffer);
    if buffer.find("q").is_some() || buffer.find("Q").is_some() {
      println!("breaking out of loop");
      break;
    }
  }

  Ok(())
}

#[allow(dead_code)]
fn print_every_directory_path() {
  let every_entry = std::fs::read_dir(".");
  if every_entry.is_ok() {
    for entry in every_entry.unwrap() {
      let dir = entry.unwrap();
      println!("{:?}", dir.path());
    }
  }
}

fn init() -> gameData {
  let grid = gameGrid::create(String::from(""), DEFAULT_WIDTH, DEFAULT_HEIGHT);
  let player = player::new();
  let mut game_data = gameData::create(grid, player);
  let path = Path::new(DIRECTORY).join("test_level");
  let _file = std::fs::File::open(path).unwrap();
  game_data.init(DEFAULT_WIDTH, DEFAULT_HEIGHT, _file);
  game_data
}

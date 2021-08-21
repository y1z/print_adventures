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
use pra_game_data::gameData;

const INTRO_TEXT: &str = "hello and welcome to the world of print ventures";

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
    return Ok(());
}

fn init() -> gameData {
    return gameData::new();
}

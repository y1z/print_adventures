mod error_types;
mod util {
    pub mod util_traits;
}
mod common_character_data;
mod vector2;
use error_types::{errorTypes, PRINT_DEBUG_INFO};
use terminal_size::{terminal_size, Height, Width};
use vector2::{Vector2f, Vector2i};

type mainReturn = Result<(), errorTypes>;

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

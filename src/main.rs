//use error_types::{errorTypes, PRINT_DEBUG_INFO};
mod error_types;
use error_types::{errorTypes, PRINT_DEBUG_INFO};
use terminal_size::{terminal_size, Height, Width};

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
    let term_size = terminal_size();

    let terminal_width = 0;
    let terminal_height = 0;
    if let Some((Width(terminal_width), Height(terminal_height))) = term_size {
        println!(
            "Your terminal is {} cols wide and {} lines tall",
            terminal_width, terminal_height
        );
        assert_ne!(terminal_width, 0);
        assert_ne!(terminal_height, 0);
    } else {
        println!("Unable to get terminal size");
    }
    return Ok(());
}

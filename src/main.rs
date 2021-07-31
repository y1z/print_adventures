const INTRO_TEXT: &str = "hello and welcome to the world of print ventures";
mod grid;
use terminal_size::{terminal_size, Height, Width};

fn main() {
    let size = terminal_size();
    let a = grid::Grid::new();
    if let Some((Width(term_width), Height(term_height))) = size {
        println!(
            "Your terminal is {} cols wide and {} lines tall",
            term_width, term_height
        );
    } else {
        println!("Unable to get terminal size");
    }
}

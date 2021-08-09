mod error_types;
use terminal_size::{terminal_size, Height, Width};
/// represents a grid in the game
pub struct gameGrid {
  m_grid: String,
  m_width: u16,
  m_height: u16,
}

impl gameGrid {
  // used to create a grid with custom parameters.
  pub fn create(string: String, width: u16, height: u16) -> gameGrid {
    return gameGrid {
      m_grid: string,
      m_width: 0,
      m_height: 0,
    };
  }

  /// used for making a new empty grid.
  pub fn new() -> gameGrid {
    return gameGrid::create(String::from(""), 0, 0);
  }

  /// updates the grid.
  pub fn update(&self){
    update_terminal_size(self);
  }

  /// Finds out what the size of the users terminal is.
  fn update_terminal_size(&self)  {
    let term_size = terminal_size();

    if let Some((Width(self.m_width), Height(self.m_height))) = term_size {
      println!(
        "Your terminal is {} cols wide and {} lines tall",
        self.m_width, self.m_height
      );
      assert_ne!(self.m_width, 0);
      assert_ne!(self.m_height, 0);
    } 
    error_types::PRINT_DEBUG_INFO!();
  }
}
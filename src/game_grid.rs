use crate::error_types;
use terminal_size::{terminal_size, Height, Width};
/// represents a grid in the game
#[derive(Debug, Clone)]
pub struct gameGrid {
  m_grid: String,
  m_width: u16,
  m_height: u16,
}

macro_rules! FILL_FORMAT {
  () => {
    "{:%<-spaces$}"
  };
}

impl gameGrid {
  /// used to create a grid with custom parameters.
  pub fn create(string: String, width: u16, height: u16) -> gameGrid {
    return gameGrid {
      m_grid: string,
      m_width: width,
      m_height: height,
    };
  }

  /// used for making a new empty grid.
  pub fn new() -> gameGrid {
    return gameGrid::create(String::from(""), 0, 0);
  }

  /// updates the grid.
  pub fn update(&mut self) {
    self.update_terminal_size();
  }

  /// Finds out what the size of the users terminal is.
  fn update_terminal_size(&mut self) {
    let term_size = terminal_size();
    if let Some((Width(temp_width), Height(temp_height))) = term_size {
      println!(
        "Your terminal is {} cols wide and {} lines tall",
        temp_width, temp_height
      );
      self.m_width = temp_width;
      self.m_height = temp_height;
      assert_ne!(self.m_width, 0);
      assert_ne!(self.m_height, 0);
    }
    error_types::PRINT_DEBUG_INFO!();
  }

  /// fills the grid with '%' characters .
  pub fn fill_grid(&mut self) {
    const FILL_ENTIRE_GRID_FORMAT: &'static str = "{:%<-spaces$}";
    self.update_terminal_size();
    let total_spaces = self.m_width * self.m_height;
    self.m_grid = format!(FILL_FORMAT!(), "", spaces = total_spaces as usize);
  }
}

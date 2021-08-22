use crate::error_types;
use crate::vector2::Vector2u;
use terminal_size::{terminal_size, Height, Width};

/// represents the data from the terminal
#[derive(Debug, Clone)]
pub struct terminalInfo {
  m_terminal_width: u16,
  m_terminal_height: u16,
}

/// represents a grid in the game
#[derive(Debug, Clone)]
pub struct gameGrid {
  m_grid: String,
  m_width: u16,
  m_height: u16,
  m_term: terminalInfo,
}

macro_rules! FILL_FORMAT {
  () => {
    "{:%<-spaces$}"
  };
}

impl terminalInfo {
  fn new() -> terminalInfo {
    terminalInfo {
      m_terminal_width: 0,
      m_terminal_height: 0,
    }
  }
  /// Finds out what the size of the users terminal is.
  fn get_terminal_size() -> (u16, u16) {
    let term_size = terminal_size();
    let mut result: (u16, u16) = (0, 0);
    if let Some((Width(result_width), Height(result_height))) = term_size {
      assert_ne!(result_width, 0);
      assert_ne!(result_height, 0);
      result = (result_width, result_height);
    }
    result
  }

  /// Set's the terminal size
  fn set_terminal_size(&mut self, new_term_size: (u16, u16)) {
    self.m_terminal_height = new_term_size.0;
    self.m_terminal_width = new_term_size.1;
  }

  /// Initalizes the 'terminalInfo' struct
  pub fn init(&mut self) {
    self.set_terminal_size(terminalInfo::get_terminal_size());
  }
}

impl gameGrid {
  /// used to create a grid with custom parameters.
  pub fn create(string: String, width: u16, height: u16) -> gameGrid {
    let mut terminal_info = terminalInfo::new();
    terminal_info.init();
    return gameGrid {
      m_grid: string,
      m_width: width,
      m_height: height,
      m_term: terminal_info,
    };
  }

  /// used for making a new empty grid.
  pub fn new() -> gameGrid {
    return gameGrid::create(String::from(""), 0, 0);
  }

  /// initalizes the game grid
  pub fn init(&mut self, width: u16, height: u16) {
    self.m_grid.reserve((width * height) as usize);
    self.fill_grid();
  }

  /// fills the grid with '%' characters .
  pub fn fill_grid(&mut self) {
    let total_spaces = self.m_width * self.m_height;
    self.m_grid = format!(FILL_FORMAT!(), "", spaces = total_spaces as usize);
  }
}

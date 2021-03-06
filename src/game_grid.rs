use crate::error_types::{PANIC_WITH_DEBUG_INFO, PRINT_DEBUG_INFO};
use crate::player;
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
    let mut result: (u16, u16) = (0, 0);
    let term_size = terminal_size();
    if let Some((Width(w), Height(h))) = term_size {
      println!("Your terminal is {} cols wide and {} lines tall", w, h);
      result.0 = w;
      result.1 = h;
    } else {
      println!("Unable to get terminal size");
      PRINT_DEBUG_INFO!();
    }
    result
  }

  /// Set's the terminal size
  fn set_terminal_size(&mut self, new_term_width: u16, new_term_height: u16) {
    self.m_terminal_height = new_term_height;
    self.m_terminal_width = new_term_width;
  }

  /// Initalizes the 'terminalInfo' struct
  pub fn init(&mut self) {
    let new_term_size = terminalInfo::get_terminal_size();
    self.set_terminal_size(new_term_size.0, new_term_size.1);
  }
}

impl gameGrid {
  /// used to create a grid with custom parameters.
  pub fn create(string: String, width: u16, height: u16) -> gameGrid {
    return gameGrid {
      m_grid: string,
      m_width: width,
      m_height: height,
      m_term: terminalInfo::new(),
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
    self.m_term.init();
  }

  /// fills the grid with '%' characters .
  pub fn fill_grid(&mut self) {
    let total_spaces = self.m_width * self.m_height;
    self.m_grid = format!(FILL_FORMAT!(), "", spaces = total_spaces as usize);
  }

  pub fn insert_player(&mut self, the_player: &player) {
    let player_pos = &the_player.m_data.m_pos;
    if player_pos.x >= self.m_width as i32 || player_pos.y >= self.m_height as i32 {
      PANIC_WITH_DEBUG_INFO!("out of bounds error");
    }
    let grid_index = ((player_pos.y * self.m_width as i32) + player_pos.x) as usize;
    let char_to_replace_index = self.m_grid.char_indices().nth(grid_index).unwrap().0;
    self
      .m_grid
      .replace_range(char_to_replace_index..char_to_replace_index, "p");
  }

  pub fn print_grid(&self) {
    let how_many_lines_to_print = (self.m_height * self.m_width) / self.m_width;
    let is_odd = self.m_term.m_terminal_width % 2 == 1;
    let offset = ((self.m_term.m_terminal_width / 2) - self.m_width) as usize;
    for x in 0..how_many_lines_to_print {
      let start_silce = (x * self.m_width) as usize;
      let end_silce = start_silce + self.m_width as usize;
      let silce = &self.m_grid[start_silce..end_silce];
      print!("{:<space$}", "", space = offset);
      print!("{}", silce);
      print!(
        "{:<space$}",
        "",
        space = offset + self.m_width as usize + is_odd as usize
      );
    }
  }
}

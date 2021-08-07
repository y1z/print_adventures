use terminal_info; 
/// represents a grid in the game
pub struct gameGrid {
  m_grid: String,
  m_terminal: terminalData,
}

impl gameGrid {
  // used to create a grid
  pub fn create(string: String) -> gameGrid {
    return gameGrid { m_grid: string ,terminalData::new() };
  }

  /// used for making a new empty grid.
  pub fn new() -> gameGrid {
    return gameGrid::create(String::from(""));
  }
}

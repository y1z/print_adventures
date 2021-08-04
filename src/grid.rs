/// represents a grid in the game
pub struct Grid {
  m_grid: String,
  m_xsize: u32,
  m_ysize: u32,
}

impl Grid {
  // used to create a grid
  pub fn create(string: String, height: u32, width: u32) -> Grid {
    return Grid {
      m_grid: string,
      m_ysize: height,
      m_xsize: width,
    };
  }

  /// used for making a new empty grid.
  pub fn new() -> Grid {
    return Grid::create(String::from(""), 0, 0);
  }
}

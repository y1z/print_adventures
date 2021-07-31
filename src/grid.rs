/// represents a grid
pub struct Grid {
  m_grid: String,
  m_xsize: u32,
  m_ysize: u32,
}

impl Grid {
  /// used for making a new empty grid.
  pub fn new() -> Grid {
    return Grid {
      m_grid: String::from(""),
      m_xsize: 0,
      m_ysize: 0,
    };
  }
}

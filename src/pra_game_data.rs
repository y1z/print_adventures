mod game_data {
  mod game_grid;
  use game_grid::gameGrid;

  /// Represent the data that the game needs to function.
  #[derive(Debug)]
  pub struct gameData {
    m_grid: gameGrid,
  }

  impl gameData {
    pub fn init(&self) {
      self.m_grid = gameGrid::new();
    }
  }
}

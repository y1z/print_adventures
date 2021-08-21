use crate::game_grid::gameGrid;
use crate::pra_player::player;

/// Represent the data that the game needs to function.
#[derive(Debug, Clone)]
pub struct gameData {
  m_grid: gameGrid,
  m_player: player,
}

impl gameData {
  /// creates a custom instance of 'gameData'
  pub fn create(grid: gameGrid, in_player: player) -> gameData {
    gameData {
      m_grid: grid,
      m_player: in_player,
    }
  }

  /// creates a default instance of 'gameData'
  pub fn new() -> gameData {
    gameData::create(gameGrid::new(), player::new())
  }

  pub fn init(&mut self) {
    self.m_grid = gameGrid::new();
  }
}

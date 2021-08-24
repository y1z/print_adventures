use crate::game_grid::gameGrid;
use crate::pra_player::player;
use std::fs;

/// Represent the data that the game needs to function.
#[derive(Debug, Clone)]
pub struct gameData {
  pub m_grid: gameGrid,
  pub m_player: player,
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

  pub fn init(&mut self, width: u16, height: u16, _file: fs::File) {
    self.m_grid.init(width, height);
  }
}

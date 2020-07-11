use gdnative::*;

/// Game state struct.
#[derive(NativeClass)]
#[inherit(Node)]
pub struct GameState {
  score: u16,
}

#[methods]
impl GameState {
  /// Initialize game state.
  fn _init(_owner: gdnative::Node) -> Self {
    GameState { score: 0 }
  }

  /// Return the player score.
  #[export]
  fn score(&self, _owner: gdnative::Node) -> u16 {
    self.score
  }

  /// Increment player score.
  #[export]
  fn increment_score(&mut self, _owner: gdnative::Node) {
    self.score += 1;
  }

  /// Reset the game state.
  #[export]
  fn reset(&mut self, _owner: gdnative::Node) {
    self.score = 0;
  }
}

use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(Node)]
/// Game state struct.
pub struct GameState {
  score: u16,
}

#[methods]
impl GameState {
  /// Create a game state.
  fn new(_owner: &Node) -> Self {
    GameState { score: 0 }
  }

  #[export]
  /// Return the player score.
  fn score(&self, _owner: &Node) -> u16 {
    self.score
  }

  #[export]
  /// Increment player score by one point.
  fn increment_score(&mut self, _owner: &Node) {
    self.score += 1;
  }

  #[export]
  /// Reset the game state.
  fn reset(&mut self, _owner: &Node) {
    self.score = 0;
  }
}

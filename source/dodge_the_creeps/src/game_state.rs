use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(Node)]
/// Game state struct.
pub struct GameState {
  score: u16,
}

#[methods]
impl GameState {
  /// Initialize game state.
  fn new(_owner: &Node) -> Self {
    GameState { score: 0 }
  }

  #[export]
  /// Return the player score.
  pub fn score(&self, _owner: &Node) -> u16 {
    self.score
  }

  #[export]
  /// Increment player score by one point.
  pub fn increment_score(&mut self, _owner: &Node) {
    self.score += 1;
  }

  #[export]
  /// Reset the game state.
  pub fn reset(&mut self, _owner: &Node) {
    self.score = 0;
  }
}

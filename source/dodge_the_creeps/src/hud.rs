use crate::extensions::NodeExt as _;
use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(CanvasLayer)]
#[user_data(user_data::ArcData<HUD>)]
#[register_with(Self::register_hud)]
/// Game state struct.
pub struct HUD;

#[methods]
impl HUD {
  /// Create a new HUD.
  fn new(_owner: &CanvasLayer) -> Self {
    HUD
  }

  /// Register HUD.
  fn register_hud(builder: &ClassBuilder<Self>) {
    // This signal tells that the start button has been pressed
    builder.add_signal(Signal { name: "start_game", args: &[] });
  }

  #[export]
  /// Called every time the node is added to the scene.
  pub fn _ready(&self, owner: &CanvasLayer) {
    // Hide score label
    unsafe { owner.get_typed_node::<Label, _>("ScoreLabel") }.hide();
  }

  #[export]
  /// Show a message.
  pub fn show_message(&self, owner: &CanvasLayer, text: String) {
    // Get message label
    let label = unsafe { owner.get_typed_node::<Label, _>("MessageLabel") };
  	label.set_text(text);
  	label.show();
    // Start message timer
    unsafe { owner.get_typed_node::<Timer, _>("MessageTimer") }.start(0.0);
  }

  #[export]
  /// Show the game over message.
  pub fn show_game_over(&self, owner: &CanvasLayer) {
    // Show message
    self.show_message(owner, "Game Over".into());
    // Stop message timer
    unsafe { owner.get_typed_node::<Timer, _>("MessageTimer") }.stop();
    // Show start button
    unsafe { owner.get_typed_node::<Button, _>("StartButton") }.show();
    // Show game title
    let label = unsafe { owner.get_typed_node::<Label, _>("MessageLabel") };
  	label.set_text("Dodge the Creeps!");
  	label.show();
  }

  #[export]
  pub fn update_score(&self, owner: &CanvasLayer, score: u16) {
    // Update score label
    unsafe { owner.get_typed_node::<Label, _>("ScoreLabel") }
      .set_text(score.to_string());
  }

  #[export]
  /// Start the game.
  pub fn on_start_button_pressed(&self, owner: &CanvasLayer) {
    // Hide start button
    unsafe { owner.get_typed_node::<Button, _>("StartButton") }.hide();
    // Show score label
    unsafe { owner.get_typed_node::<Label, _>("ScoreLabel") }.show();
    // Emit start game signal
    owner.emit_signal("start_game", &[]);
  }

  #[export]
  /// Start the game.
  pub fn on_message_timer_timeout(&self, owner: &CanvasLayer) {
    // Hide start button
    unsafe { owner.get_typed_node::<Label, _>("MessageLabel") }.hide();
  }
}

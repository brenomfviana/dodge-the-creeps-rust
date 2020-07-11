use crate::extensions::NodeExt as _;
use gdnative::api::{AnimatedSprite, Area2D, CollisionShape2D, PhysicsBody2D};
use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(Area2D)]
#[user_data(user_data::MutexData<Player>)]
#[register_with(Self::register_player)]
/// Player struct.
pub struct Player {
  #[property(default = 400.0)]
  speed: f32,
  screen_size: Vector2,
  can_move: bool,
}

#[methods]
impl Player {
  /// Create a new player.
  pub fn new(_owner: &Area2D) -> Self {
    Player {
      speed: 400.0, screen_size: Vector2::new(0.0, 0.0), can_move: false
    }
  }

  #[export]
  /// Allow the player to move in the level.
  pub fn allow_move(&mut self, _owner: &Area2D) {
    self.can_move = true;
  }

  /// Register the player.
  pub fn register_player(builder: &ClassBuilder<Self>) {
    // The player emit this signal when it collides with an enemy
    builder.add_signal(Signal { name: "hit", args: &[] });
  }

  #[export]
  /// Prepare the player.
  pub fn _ready(&mut self, owner: &Area2D) {
    let viewport = unsafe { owner.get_viewport().unwrap().assume_safe() };
    self.screen_size = viewport.size();
    owner.hide();
  }

  #[export]
  /// Run the player commands.
  pub fn _process(&mut self, owner: &Area2D, delta: f32) {
    // Check if the player can move
    if self.can_move {
      // Get the player animated sprite
      let anim_sprite = unsafe {
        owner.get_typed_node::<AnimatedSprite, _>("AnimatedSprite")
      };
      // Get input handler
      let input = Input::godot_singleton();
      // Initialize movement variables
      let mut velocity = Vector2::new(0.0, 0.0);
      // Check which keys were pressed and apply the respective move
      if Input::is_action_pressed(&input, GodotString::from_str("ui_right")) {
        velocity.x += 1.0
      }
      if Input::is_action_pressed(&input, GodotString::from_str("ui_left")) {
        velocity.x -= 1.0
      }
      if Input::is_action_pressed(&input, GodotString::from_str("ui_down")) {
        velocity.y += 1.0
      }
      if Input::is_action_pressed(&input, GodotString::from_str("ui_up")) {
        velocity.y -= 1.0
      }
      // Check if the player is not stopped
      if velocity.length() > 0.0 {
        // Moving player animation
        velocity = velocity.normalize() * self.speed;
        let animation;
        if velocity.x != 0.0 {
          animation = "right";
          anim_sprite.set_flip_v(false);
          anim_sprite.set_flip_h(velocity.x < 0.0)
        } else {
          animation = "up";
          anim_sprite.set_flip_v(velocity.y > 0.0)
        }
        anim_sprite.play(GodotString::from_str(animation), false);
      } else {
        // Stopped player animation
        anim_sprite.stop();
      }
      // Fix velocity
      let change = velocity * delta;
      // Update player position
      let position = (owner.global_position() + change)
        .clamp(Vector2::new(0.0, 0.0), self.screen_size);
      owner.set_global_position(position);
    }
  }

  #[export]
  /// Reset the player for a new game.
  pub fn start(&self, owner: &Area2D, pos: Vector2) {
    // Reset to default initial position
    owner.set_global_position(pos);
    owner.show();
    // Enable collision
    unsafe { owner.get_typed_node::<CollisionShape2D, _>("CollisionShape2D") }
      .set_disabled(false);
  }

  #[export]
  /// The player was hitted and lost the game.
  pub fn on_player_body_entered(&mut self, owner: &Area2D,
    _body: Ref<PhysicsBody2D>) {
      // Player disappears after being hit
      self.can_move = false;
      owner.emit_signal("hit", &[]);
      owner.hide();
      // Disable collision
      unsafe { owner.get_typed_node::<CollisionShape2D, _>("CollisionShape2D") }
        .set_deferred("disabled", true);
  }
}

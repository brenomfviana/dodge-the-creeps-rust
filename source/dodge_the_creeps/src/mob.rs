use crate::extensions::NodeExt;
use gdnative::api::{AnimatedSprite, RigidBody2D};
use gdnative::prelude::*;
use rand::seq::SliceRandom;

#[derive(Copy, Clone, Debug)]
/// Mob types.
enum MobType {
  Walk,
  Swim,
  Fly,
}

impl MobType {
  /// Return the mob type name.
  pub fn to_string(self) -> String {
    format!("{:?}", self).to_lowercase()
  }
}

/// List of mob types
const MOB_TYPES: [MobType; 3] = [MobType::Walk, MobType::Swim, MobType::Fly];

#[derive(NativeClass)]
#[inherit(RigidBody2D)]
#[user_data(user_data::MutexData<Mob>)]
/// Mob struct.
pub struct Mob {
  #[property(default = 150.0)]
  min_speed: f32,
  #[property(default = 250.0)]
  max_speed: f32,
}

#[methods]
impl Mob {
  /// Create a Mob.
  pub fn new(_owner: &RigidBody2D) -> Self {
    Mob { min_speed: 150.0, max_speed: 250.0 }
  }

  /// Return the mob min speed.
  pub fn min_speed(&self, _owner: &RigidBody2D) -> f32 {
    self.min_speed
  }

  /// Return the mob max speed.
  pub fn max_speed(&self, _owner: &RigidBody2D) -> f32 {
    self.max_speed
  }

  #[export]
  /// Prepare the mob.
  pub fn _ready(&self, owner: &RigidBody2D) {
    // Initialize rand generator
    let mut rng = rand::thread_rng();
    // Set mob animation
    if let Some(anim) = MOB_TYPES.choose(&mut rng) {
      unsafe { owner.get_typed_node::<AnimatedSprite, _>("AnimatedSprite") }
        .set_animation(anim.to_string());
    } else {
      panic!("The chosen Mob type does not exist.");
    }
  }

  #[export]
  /// Free the Mob when it exits the screen.
  pub fn _on_visibility_screen_exited(&self, owner: &RigidBody2D) {
    unsafe { owner.assume_unique().queue_free(); }
  }

  #[export]
  /// Free the remaining mobs of the last game when the game start.
  pub fn on_start_game(&self, owner: &RigidBody2D) {
    unsafe { owner.assume_unique().queue_free(); }
  }
}

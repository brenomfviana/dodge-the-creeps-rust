use crate::extensions::NodeExt as _;
use crate::hud;
use crate::mob;
use crate::player;
use gdnative::api::{Area2D, AudioStreamPlayer, PathFollow2D, Position2D,
  RigidBody2D};
use gdnative::prelude::*;
use rand::*;
use std::f64::consts::PI;

#[derive(NativeClass)]
#[inherit(Node)]
#[user_data(user_data::LocalCellData<GameHandler>)]
pub struct GameHandler {
  #[property]
  mob: Ref<PackedScene>,
  score: u16,
}

#[methods]
impl GameHandler {
  /// Create a game handler.
  pub fn new(_owner: &Node) -> Self {
    GameHandler { mob: PackedScene::new().into_shared(), score: 0 }
  }

  #[export]
  /// Prepare main scene.
  pub fn _ready(&self, owner: &Node) {
    unsafe { owner.get_typed_node::<AudioStreamPlayer, _>("Music") }.play(0.0);
  }

  #[export]
  /// Run the game over.
  pub fn game_over(&self, owner: &Node) {
    // Stop mob and score timers
    unsafe { owner.get_typed_node::<Timer, _>("MobTimer") }.stop();
    unsafe { owner.get_typed_node::<Timer, _>("ScoreTimer") }.stop();
    // Get HUD node and show the game over message
    let hud_node = unsafe { owner.get_typed_node::<CanvasLayer, _>("HUD") };
    hud_node.cast_instance::<hud::HUD>()
      .and_then(|hud| hud.map(|x, o| x.show_game_over(&*o)).ok())
      .unwrap_or_else(|| godot_print!("Unable to get hud"));
    // Run death sound
    unsafe { owner.get_typed_node::<AudioStreamPlayer, _>("Music") }.stop();
    unsafe { owner.get_typed_node::<AudioStreamPlayer, _>("Death") }
      .play(0.0);
  }

  #[export]
  /// Prepare the game for a new game.
  fn new_game(&mut self, owner: &Node) {
    // Get default start position
    let start_position = unsafe {
      owner.get_typed_node::<Position2D, _>("StartPosition")
    };
    // Get the player
    let player = unsafe { owner.get_typed_node::<Area2D, _>("Player") };
    // Reset the player position
    player.cast_instance::<player::Player>()
      .and_then(|player| {
        player.map_mut(|x, o| x.start(&*o, start_position.position())).ok()
      }).unwrap_or_else(|| godot_print!("Unable to get player"));
    // Get HUD
    let hud_node = unsafe { owner.get_typed_node::<CanvasLayer, _>("HUD") };
    // Print 'Get Ready' message
    hud_node.cast_instance::<hud::HUD>()
      .and_then(|hud| { hud.map(|x, o| {
          x.update_score(&*o, self.score);
          x.show_message(&*o, "Get Ready".into()
        )}).ok()
      }).unwrap_or_else(|| godot_print!("Unable to get hud"));
    // Start the start timer
    unsafe { owner.get_typed_node::<Timer, _>("StartTimer") }.start(0.0);
    // Run main music
    unsafe { owner.get_typed_node::<AudioStreamPlayer, _>("Music") }.play(0.0);
    unsafe { owner.get_typed_node::<AudioStreamPlayer, _>("Death") }.stop();
  }

  #[export]
  /// Reset the mob and score timers.
  fn on_start_timer_timeout(&self, owner: &Node) {
    unsafe { owner.get_typed_node::<Timer, _>("MobTimer") }.start(0.0);
    unsafe { owner.get_typed_node::<Timer, _>("ScoreTimer") }.start(0.0);
    // Get the player
    let player = unsafe { owner.get_typed_node::<Area2D, _>("Player") };
    // Reset the player position
    player.cast_instance::<player::Player>()
      .and_then(|player| {
        player.map_mut(|x, o| {
          x.allow_move(&*o)
        }).ok()
      }).unwrap_or_else(|| godot_print!("Unable to get player"));
  }

  #[export]
  /// Update the player score.
  fn on_score_timer_timeout(&mut self, owner: &Node) {
    // Increase the player score
    self.score += 1;
    // Get HUD
    let hud_node = unsafe { owner.get_typed_node::<CanvasLayer, _>("HUD") };
    // Update score
    hud_node.cast_instance::<hud::HUD>()
      .and_then(|hud| hud.map(|x, o| x.update_score(&*o, self.score)).ok())
      .unwrap_or_else(|| godot_print!("Unable to get hud"));
  }

  #[export]
  fn on_mob_timer_timeout(&self, owner: &Node) {
    // Get mob spawn location
    let mob_spawn_location = unsafe {
      owner.get_typed_node::<PathFollow2D, _>("MobPath/MobSpawnLocation")
    };
    // Create a new mob
    let mob: Ref<RigidBody2D, _> = instance_scene(&self.mob);
    // Spwan mobs randomly
    let mut rng = rand::thread_rng();
    let offset = rng.gen_range(std::u32::MIN, std::u32::MAX);
    mob_spawn_location.set_offset(offset.into());
    // Set mob direction
    let mut direction = mob_spawn_location.rotation() + PI / 2.0;
    // Set mob position
    mob.set_position(mob_spawn_location.position());
    // Fix direction
    direction += rng.gen_range(-PI / 4.0, PI / 4.0);
    mob.set_rotation(direction);
    let d = direction as f32;
    // Add mob in the game
    let mob = unsafe { mob.into_shared().assume_safe() };
    owner.add_child(mob, false);
    // Convert the scene into mob struct
    let mob = mob.cast_instance::<mob::Mob>().unwrap();
    // For each mob (x) and mob_owner
    mob.map(|x, mob_owner| {
      // Get mob min and max speed
      let (min, max) = (x.min_speed(&*mob_owner), x.max_speed(&*mob_owner));
      // Set a random mob velocity
      mob_owner.set_linear_velocity(Vector2::new(rng.gen_range(min, max), 0.0));
      mob_owner.set_linear_velocity(mob_owner.linear_velocity()
        .rotated(Angle { radians: d }));
      // Get HUD
      unsafe { owner.get_typed_node::<CanvasLayer, _>("HUD") }
        .cast_instance::<hud::HUD>().unwrap()
        // Connect the `start_game` signal to delete old mobs
        .map(|_, o| {
          o.connect("start_game", mob_owner, "on_start_game",
            VariantArray::new_shared(), 0 ).unwrap();
        }).unwrap();
    }).unwrap();
  }
}

/// Root here is needs to be the same type (or a parent type) of the node that
/// you put in the child scene as the root. For instance Spatial is used for
/// this example.
fn instance_scene<Root>(scene: &Ref<PackedScene, Shared>) -> Ref<Root, Unique>
where Root: gdnative::GodotObject<RefKind = ManuallyManaged> + SubClass<Node> {
  // Get the scene
  let scene = unsafe { scene.assume_safe() };
  // Create a new instance of the scene
  let instance = scene.instance(PackedScene::GEN_EDIT_STATE_DISABLED)
    .expect("should be able to instance scene");
  let instance = unsafe { instance.assume_unique() };
  // Return the instance of the scene
  instance.try_cast::<Root>().expect("root node type should be correct")
}

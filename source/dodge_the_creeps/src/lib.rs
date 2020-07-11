use gdnative::prelude::*;
mod extensions;
mod game_state;
mod hud;

/// Expose all Rust code for Godot.
fn init(handle: InitHandle) {
  handle.add_class::<game_state::GameState>();
  handle.add_class::<hud::HUD>();
}

godot_gdnative_init!();
godot_nativescript_init!(init);
godot_gdnative_terminate!();

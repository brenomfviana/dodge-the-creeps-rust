use gdnative::prelude::*;
mod game_state;

/// Expose all Rust code for Godot.
fn init(handle: InitHandle) {
  handle.add_class::<game_state::GameState>();
}

godot_gdnative_init!();
godot_nativescript_init!(init);
godot_gdnative_terminate!();

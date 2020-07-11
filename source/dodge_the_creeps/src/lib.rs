use gdnative::*;
mod game_state;

fn init(handle: gdnative::init::InitHandle) {
  handle.add_class::<game_state::GameState>();
}

godot_gdnative_init!();
godot_nativescript_init!(init);
godot_gdnative_terminate!();

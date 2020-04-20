use crate::*;

/// This is the main game loop that runs the rest of the game.
/// Try to keep it slim and trim.
pub fn game_loop() {
  time_hack("loop starting!");

  for room in game::rooms::values() {
    manage_room(room);
  }

  let time = screeps::game::time();

  if time % 32 == 3 {
    info!("running memory cleanup");
    cleanup_memory()
      .expect("expected Memory.creeps format to be a regular memory object");
  }

  time_hack("Loop done!");
}

/// This is a quick and easy way to get a time-hack at any point in the program.
pub fn time_hack(msg: &str) {
  let _time = screeps::game::cpu::get_used();
  info!("{} CPU: {}", msg, _time);
}

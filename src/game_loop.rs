use crate::*;

/// This is the main game loop that runs the rest of the game.
/// Try to keep it slim and trim.
pub fn game_loop() {
  time_hack("==============loop starting!==============");

  // initialize the director
  let mut director = Director::default();

  let time = screeps::game::time();

  // update the director
  director.update();

  if time % 32 == 3 {
    info!("running memory cleanup");
    cleanup_memory()
      .expect("expected Memory.creeps format to be a regular memory object");
  }

  // save the director
  if director.save() {
    trace!("Director successfully saved to memory");
  }

  time_hack("Loop done!");
}

/// This is a quick and easy way to get a time-hack at any point in the program.
pub fn time_hack(msg: &str) {
  let _time = game::cpu::get_used();
  info!("{} CPU: {}", msg, _time);
}

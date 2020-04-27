use crate::*;

/// This initializes logging and anything else that needs it only once.
pub fn init() {
  logging::setup_logging(logging::Info);

  // set my username as a memory object
  if let Ok(Some(_)) = root().string("Username") {
    ()
  } else {
    if let Some(spawn) = game::spawns::values().get(0) {
      let username = spawn.owner_name().unwrap();
      root().set("Username", username);
    }
  }
}

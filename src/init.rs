use crate::*;

/// This initializes logging and anything else that needs it only once.
pub fn init() {
  logging::setup_logging(logging::Trace);

  // set my username as a memory object
  let root = screeps::memory::root();
  if let Some(Values::Username(_)) = root.get_value(Keys::Username) {
    ()
  } else {
    if let Some(spawn) = game::spawns::values().get(0) {
      let username = spawn.owner_name().unwrap();
      root.set_value(Values::Username(username));
    }
  }
}

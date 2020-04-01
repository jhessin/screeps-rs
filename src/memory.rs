use crate::*;

/// This cleans up any memory references that are no longer needed. creeps, etc.
pub fn cleanup_memory() -> std::result::Result<(), Box<dyn std::error::Error>> {
  // Get all of the creeps that are still alive
  let alive_creeps: HashSet<String> =
    screeps::game::creeps::keys().into_iter().collect();

  // Get all of the creeps in the memory.
  let screeps_memory = match screeps::memory::root().dict("creeps")? {
    Some(v) => v,
    None => {
      warn!("not cleaning game creep memory: no Memory.creeps dict");
      return Ok(());
    }
  };

  // remove the ones that are dead.
  for mem_name in screeps_memory.keys() {
    if !alive_creeps.contains(&mem_name) {
      debug!("cleaning up creep memory of dead creep {}", mem_name);
      screeps_memory.del(&mem_name);
    }
  }

  Ok(())
}

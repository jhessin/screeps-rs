//! The HasMiner trait tells me if a source has a miner assigned to it.
use crate::*;

/// This lets me know if a source has a miner assigned to it.
pub trait HasCreep {
  /// Returns true if the source has a miner assigned to it.
  fn has_creep(&self) -> bool;
}

impl HasCreep for Source {
  fn has_creep(&self) -> bool {
    for creep in game::creeps::values() {
      let mut creep = Creeper::new(creep);
      if creep.role == Role::miner() {
        if let Some(Target::Source(source)) = creep.data().source() {
          if source.id() == self.id() {
            return true;
          }
        }
      }
    }
    false
  }
}

impl HasCreep for Resource {
  fn has_creep(&self) -> bool {
    for creep in game::creeps::values() {
      let mut creep = Creeper::new(creep);
      if let Some(Target::Resource(source)) = creep.data().source() {
        if source.id() == self.id() && !creep.working() {
          return true;
        }
      }
    }
    false
  }
}

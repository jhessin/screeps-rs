//! The HasMiner trait tells me if a source has a miner assigned to it.
use crate::*;

/// This lets me know if a source has a miner assigned to it.
pub trait HasMiner {
  /// Returns true if the source has a miner assigned to it.
  fn has_miner(&self) -> bool;
}

impl HasMiner for Source {
  fn has_miner(&self) -> bool {
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

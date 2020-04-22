//! The HasMiner trait tells me if a source has a miner assigned to it.
use crate::*;

/// This lets me know if a source has a miner assigned to it.
pub trait HasCreep {
  /// Returns true if the source has a miner assigned to it.
  fn has_creep(&self) -> bool;
}

impl<T: HasId + RoomObjectProperties> HasCreep for T {
  fn has_creep(&self) -> bool {
    for creep in self.room().find(find::MY_CREEPS) as Vec<Creep> {
      let mut creep = Creeper::new(creep);
      if creep.role == Role::Miner {
        if let Some(Target::Source(source)) = creep.data().source() {
          if source.id().to_string() == self.id().to_string() {
            return true;
          }
        }
      }
    }
    false
  }
}

//! The HasMiner trait tells me if a source has a miner assigned to it.
use crate::*;

/// This lets me know if a source has a miner assigned to it.
pub trait HasCreep {
  /// Returns true if the source has a miner assigned to it.
  fn has_creep(&self) -> bool;
}

impl<T: HasId + RoomObjectProperties> HasCreep for T {
  fn has_creep(&self) -> bool {
    let room = if let Some(r) = self.room() { r } else { return false };
    for creep in room.find(find::MY_CREEPS) as Vec<Creep> {
      if let Some(Values::TargetId(id)) =
        creep.memory().get_value(Keys::TargetId)
      {
        if id == self.id().to_string() {
          return true;
        }
      }
    }
    false
  }
}

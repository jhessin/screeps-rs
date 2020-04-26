//! The HasMiner trait tells me if a source has a miner assigned to it.
use crate::*;

/// This lets me know if a source has a miner assigned to it.
pub trait HasCreep {
  /// Returns true if the source has a miner assigned to it.
  fn has_creep(&self) -> bool;

  /// Returns if a creep with a specific role is assigned
  fn has_creep_with_role(&self, role: Role) -> bool;
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

  fn has_creep_with_role(&self, role: Role) -> bool {
    let room = if let Some(r) = self.room() { r } else { return false };
    let creeps = room.find(find::MY_CREEPS);
    trace!("Searching through {} Creeps", creeps.len());
    for creep in creeps {
      if let (Some(Values::Role(r)), Some(Values::TargetId(id))) = (
        creep.memory().get_value(Keys::Role),
        creep.memory().get_value(Keys::TargetId),
      ) {
        trace!("Creep with role: {} and target: {}", r, id);
        trace!(
          "Searching for role: {} and target: {}",
          role,
          self.id().to_string()
        );
        if role == r && id == self.id().to_string() {
          return true;
        }
      }
    }
    false
  }
}

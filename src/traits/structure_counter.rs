use crate::*;

/// Get the structures of a specific type from a room
pub trait StructureCounter {
  /// Returns all the structures of a specific type
  fn get_structures(&self, t: StructureType) -> Vec<Structure>;
}

impl StructureCounter for Room {
  fn get_structures(&self, t: StructureType) -> Vec<Structure> {
    self
      .find(find::STRUCTURES)
      .into_iter()
      .filter(|s| s.structure_type() == t)
      .collect()
  }
}

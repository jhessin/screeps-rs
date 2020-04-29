use crate::*;

/// This serializes all the info about a structure
#[derive(Serialize, Deserialize, Hash, Eq, PartialEq)]
pub struct StructureData {
  pos: Position,
  structure_type: StructureType,
}

impl Display for StructureData {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    write!(f, "{:?} @ {}", self.structure_type, self.pos)
  }
}

impl StructureData {
  /// Generates a new structure data from a structure
  pub fn new(s: Structure) -> Self {
    let structure_type = s.structure_type();
    let pos = s.pos();
    StructureData { structure_type, pos }
  }
}

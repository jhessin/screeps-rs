use crate::*;

/// This serializes all the info about a structure
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct RuinData {
  pos: Position,
  structure_type: StructureType,
  resources: HashMap<ResourceType, u32>,
}

impl Display for RuinData {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    writeln!(
      f,
      "{:?} @ ({}, {})",
      self.structure_type,
      self.pos.x(),
      self.pos.y()
    )?;

    for (r, amount) in &self.resources {
      writeln!(f, "{:?}: {}", r, amount)?;
    }

    Ok(())
  }
}

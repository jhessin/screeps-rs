use crate::*;

/// Holds all essential data for construction
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct ConstructionData {
  pos: Position,
  id: ObjectId<ConstructionSite>,
  progress: u32,
  progress_total: u32,
  structure_type: StructureType,
}

impl Display for ConstructionData {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    writeln!(
      f,
      "{:?}: x:{}, y:{}",
      self.structure_type,
      self.pos.x(),
      self.pos.y()
    )?;
    writeln!(f, "{} of {} completed", self.progress, self.progress_total)
  }
}

impl ConstructionData {
  /// Create ConstructionData from a ConstructionSite
  pub fn new(site: ConstructionSite) -> Self {
    let pos = site.pos();
    let id = site.id();
    let progress = site.progress();
    let progress_total = site.progress_total();
    let structure_type = site.structure_type();

    ConstructionData { pos, id, progress, progress_total, structure_type }
  }
}

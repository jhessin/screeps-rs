use crate::*;

/// Holds all essential data for construction
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct ConstructionData {
  pos: CommonData,
  id: ObjectId<ConstructionSite>,
  progress: u32,
  progress_total: u32,
  structure_type: StructureType,
}

impl Display for ConstructionData {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    writeln!(f, "{:?}: {}", self.structure_type, self.pos,)?;
    writeln!(f, "{} of {} completed", self.progress, self.progress_total)
  }
}

impl Deref for ConstructionData {
  type Target = CommonData;

  fn deref(&self) -> &Self::Target {
    &self.pos
  }
}

impl From<ConstructionSite> for ConstructionData {
  fn from(site: ConstructionSite) -> Self {
    let id = site.id();
    let progress = site.progress();
    let progress_total = site.progress_total();
    let structure_type = site.structure_type();
    let pos = site.into();

    ConstructionData { pos, id, progress, progress_total, structure_type }
  }
}

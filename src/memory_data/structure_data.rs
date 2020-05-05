use crate::*;

/// This serializes all the info about a structure
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct StructureData {
  pos: CommonData,
  structure_type: StructureType,
  resources: HashMap<ResourceType, u32>,
}

impl HasPosition for StructureData {
  fn pos(&self) -> Position {
    self.pos.pos()
  }
}

impl Display for StructureData {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    writeln!(f, "{:?} @ {}", self.structure_type, self.pos,)?;

    for (r, amount) in &self.resources {
      writeln!(f, "{:?}: {}", r, amount)?;
    }

    Ok(())
  }
}

impl From<Structure> for StructureData {
  fn from(s: Structure) -> Self {
    let structure_type = s.structure_type();
    let pos = s.pos().into();
    let mut resources = HashMap::<ResourceType, u32>::new();

    if let Some(store) = s.as_has_store() {
      for r in store.store_types() {
        resources.insert(r, store.store_of(r));
      }
    }

    StructureData { structure_type, pos, resources }
  }
}
impl StructureData {
  /// Returns the Structure that this StructureData refers to
  /// PANICS if the structure isn't there
  pub fn structure(&self) -> Option<Structure> {
    self
      .pos
      .pos()
      .find_in_range(find::STRUCTURES, 0)
      .into_iter()
      .filter(|s| s.structure_type() == self.structure_type)
      .next()
  }

  /// Determines if the room this structure is in is visible
  pub fn is_visible(&self) -> bool {
    game::rooms::get(self.pos.pos().room_name()).is_some()
  }
}

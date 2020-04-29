use crate::*;

/// This serializes all the info about a structure
#[derive(Serialize, Deserialize, Eq, PartialEq)]
pub struct StructureData {
  pos: Position,
  structure_type: StructureType,
  resources: HashMap<ResourceType, u32>,
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
    let mut resources = HashMap::<ResourceType, u32>::new();

    if let Some(store) = s.as_has_store() {
      for r in store.store_types() {
        resources.insert(r, store.store_of(r));
      }
    }

    StructureData { structure_type, pos, resources }
  }

  /// Returns the Structure that this StructureData refers to
  /// PANICS if the structure isn't there
  pub fn structure(&self) -> Structure {
    self
      .pos
      .find_in_range(find::STRUCTURES, 0)
      .into_iter()
      .filter(|s| s.structure_type() == self.structure_type)
      .next()
      .unwrap()
  }

  /// Determines if the room this structure is in is visible
  pub fn is_visible(&self) -> bool {
    game::rooms::get(self.pos.room_name()).is_some()
  }
}

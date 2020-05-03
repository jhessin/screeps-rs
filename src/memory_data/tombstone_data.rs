use crate::*;

/// This serializes all the info about a structure
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct TombstoneData {
  pos: CommonData,
  resources: HashMap<ResourceType, u32>,
}

impl HasPosition for TombstoneData {
  fn pos(&self) -> Position {
    self.pos.pos()
  }
}

impl Display for TombstoneData {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    writeln!(f, "Tombstone @ {}", self.pos)?;

    for (r, amount) in &self.resources {
      writeln!(f, "{:?}: {}", r, amount)?;
    }

    Ok(())
  }
}

impl From<Tombstone> for TombstoneData {
  fn from(ts: Tombstone) -> Self {
    let mut resources = HashMap::new();

    for r in ts.store_types() {
      resources.insert(r, ts.store_of(r));
    }

    let pos = ts.into();

    TombstoneData { pos, resources }
  }
}

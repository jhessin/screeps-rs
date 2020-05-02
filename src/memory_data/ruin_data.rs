use crate::*;

/// This serializes all the info about a structure
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct RuinData {
  pos: CommonData,
  id: ObjectId<Ruin>,
  resources: HashMap<ResourceType, u32>,
}

impl Display for RuinData {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    writeln!(f, "Ruin @ {}", self.pos)?;

    for (r, amount) in &self.resources {
      writeln!(f, "{:?}: {}", r, amount)?;
    }

    Ok(())
  }
}

impl From<Ruin> for RuinData {
  fn from(ruin: Ruin) -> Self {
    let id = ruin.id();
    let mut resources = HashMap::new();

    for r in ruin.store_types() {
      resources.insert(r, ruin.store_of(r));
    }
    let pos = ruin.into();

    RuinData { pos, id, resources }
  }
}

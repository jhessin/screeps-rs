use crate::*;

/// This serializes and wraps creeps
#[derive(Serialize, Deserialize, Eq, PartialEq)]
pub struct CreepData {
  name: String,
  hits: u32,
  max_hits: u32,
  store: HashMap<ResourceType, u32>,
  parts: HashSet<Part>,
  // role: Role,
}

impl Display for CreepData {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    // TODO Add role to the Display
    writeln!(f, "{}: RoleHere - {:?}", self.name, self.parts)?;

    writeln!(f, "{} of {} HP", self.hits, self.max_hits)?;

    for (r, amount) in &self.store {
      writeln!(f, "{:?}: {}", r, amount)?;
    }

    Ok(())
  }
}

impl CreepData {
  /// Generate CreepData from a creep
  pub fn new(creep: Creep) -> Self {
    let name = creep.name();
    let hits = creep.hits();
    let max_hits = creep.hits_max();
    let mut store = HashMap::<ResourceType, u32>::new();
    let mut parts = HashSet::<Part>::new();
    // TODO add role as well as task data

    // fill parts and store
    for part in creep.body() {
      parts.insert(part.part);
    }

    for r in creep.store_types() {
      store.insert(r, creep.store_of(r));
    }

    CreepData { name, hits, max_hits, store, parts }
  }
}

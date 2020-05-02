use crate::*;

/// This serializes and wraps creeps
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct CommonCreepData {
  pos: CommonData,
  name: String,
  id: ObjectId<Creep>,
  hits: u32,
  max_hits: u32,
  parts: HashSet<Part>,
  store: HashMap<ResourceType, u32>,
  // role: Role,
}

impl Deref for CommonCreepData {
  type Target = CommonData;

  fn deref(&self) -> &Self::Target {
    &self.pos
  }
}

impl Display for CommonCreepData {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    // TODO update Display
    writeln!(f, "{}: {:?}", self.name, self.parts)?;

    writeln!(f, "{} of {} HP", self.hits, self.max_hits)?;

    for (r, amount) in &self.store {
      writeln!(f, "{:?}: {}", r, amount)?;
    }

    Ok(())
  }
}

impl From<Creep> for CommonCreepData {
  fn from(c: Creep) -> Self {
    let pos = c.pos().into();
    let name = c.name();
    let id = c.id();
    let hits = c.hits();
    let max_hits = c.hits_max();
    let mut store = HashMap::<ResourceType, u32>::new();
    let mut parts = HashSet::<Part>::new();
    // TODO add role as well as task data

    for part in c.body() {
      parts.insert(part.part);
    }

    for r in c.store_types() {
      store.insert(r, c.store_of(r));
    }

    CommonCreepData { pos, name, id, hits, max_hits, store, parts }
  }
}

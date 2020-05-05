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
}

impl HasPosition for CommonCreepData {
  fn pos(&self) -> Position {
    self.pos.pos()
  }
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

    for part in c.body() {
      parts.insert(part.part);
    }

    for r in c.store_types() {
      store.insert(r, c.store_of(r));
    }

    CommonCreepData { pos, name, id, hits, max_hits, store, parts }
  }
}

impl From<PowerCreep> for CommonCreepData {
  fn from(c: PowerCreep) -> Self {
    let pos = c.pos().into();
    let name = c.name();
    let id = c.id().into_type();
    let hits = c.hits();
    let max_hits = c.hits_max();
    let mut store = HashMap::<ResourceType, u32>::new();
    let parts = HashSet::<Part>::new();

    for r in c.store_types() {
      store.insert(r, c.store_of(r));
    }

    CommonCreepData { pos, name, id, hits, max_hits, store, parts }
  }
}

impl CommonCreepData {
  /// Determine if this is a power creep by looking at it's parts
  pub fn is_power_creep(&self) -> bool {
    self.parts.is_empty()
  }

  /// Returns the creep if this is a creep
  pub fn creep(&self) -> Option<Creep> {
    if self.is_power_creep() {
      None
    } else if let Ok(creep) = game::get_object_typed(self.id) {
      creep
    } else {
      None
    }
  }

  /// Returns the power creep if this is a power creep
  pub fn power_creep(&self) -> Option<PowerCreep> {
    if !self.is_power_creep() {
      return None;
    }

    let creep: ObjectId<PowerCreep> = self.id.into_type();
    if let Ok(creep) = game::get_object_typed(creep) {
      creep
    } else {
      None
    }
  }

  /// Get Harvesting Power of this creep
  pub fn harvesting_power(&self) -> u32 {
    if self.is_power_creep() {
      return 0;
    }

    if let Some(creep) = self.creep() {
      let parts = creep.get_active_bodyparts(Work);
      return parts * HARVEST_POWER;
    }

    error!("Creep has an invalid id!");
    0
  }

  /// Get Harvest Mineral Power
  pub fn harvest_mineral_power(&self) -> u32 {
    if self.is_power_creep() {
      return 0;
    }

    if let Some(creep) = self.creep() {
      let parts = creep.get_active_bodyparts(Work);
      return parts * HARVEST_MINERAL_POWER;
    }

    error!("Creep has an invalid id!");
    0
  }

  /// Get Build Power
  pub fn build_power(&self) -> u32 {
    if self.is_power_creep() {
      return 0;
    }

    if let Some(creep) = self.creep() {
      let parts = creep.get_active_bodyparts(Work);
      return parts * BUILD_POWER;
    }

    error!("Creep has an invalid id!");
    0
  }

  /// Get the attack power
  pub fn attack_power(&self) -> u32 {
    if let Some(creep) = self.creep() {
      let parts = creep.get_active_bodyparts(Attack);

      return parts * ATTACK_POWER;
    }

    error!("Creep has an invalid id!");
    0
  }

  /// Get ranged attack power
  pub fn ranged_attack_power(&self) -> u32 {
    if let Some(creep) = self.creep() {
      let parts = creep.get_active_bodyparts(RangedAttack);

      return parts * RANGED_ATTACK_POWER;
    }

    error!("Creep has an invalid id!");
    0
  }

  /// Get carry capacity
  pub fn carry_capacity(&self) -> u32 {
    if let Some(creep) = self.creep() {
      let parts = creep.get_active_bodyparts(Carry);

      return parts * CARRY_CAPACITY;
    }

    error!("Creep has an invalid id!");
    0
  }

  /// Get dismantle power
  pub fn dismantle_power(&self) -> u32 {
    if let Some(creep) = self.creep() {
      let parts = creep.get_active_bodyparts(Work);

      return parts * DISMANTLE_POWER;
    }

    error!("Creep has an invalid id!");
    0
  }

  /// Get heal power
  pub fn heal_power(&self) -> u32 {
    if let Some(creep) = self.creep() {
      let parts = creep.get_active_bodyparts(Heal);

      return parts * HEAL_POWER;
    }

    error!("Creep has an invalid id!");
    0
  }

  /// Get Ranged Heal Power
  pub fn ranged_heal_power(&self) -> u32 {
    if let Some(creep) = self.creep() {
      let parts = creep.get_active_bodyparts(Heal);

      return parts * RANGED_HEAL_POWER;
    }

    error!("Creep has an invalid id!");
    0
  }

  /// Repair power
  pub fn repair_power(&self) -> u32 {
    if let Some(creep) = self.creep() {
      let parts = creep.get_active_bodyparts(Work);

      return parts * REPAIR_POWER;
    }

    error!("Creep has an invalid id!");
    0
  }

  /// Upgrade power
  pub fn upgrade_power(&self) -> u32 {
    if let Some(creep) = self.creep() {
      let parts = creep.get_active_bodyparts(Work);

      return parts * UPGRADE_CONTROLLER_POWER;
    }

    error!("Creep has an invalid id!");
    0
  }
}

impl CommonCreepData {
  /// Update this creeps data
  pub fn update(&mut self, creep: Creep) {
    if self.id != creep.id() {
      warn!("Attempting to update creep with invalid data");
      return;
    }

    // Update position
    self.pos = creep.pos().into();

    // update hits
    self.hits = creep.hits();

    // update store
    for r in creep.store_types() {
      self.store.insert(r, creep.store_of(r));
    }
  }

  /// Get the parts
  pub fn parts(&self) -> HashSet<Part> {
    self.parts.clone()
  }

  /// Get the creeps name
  pub fn name(&self) -> &str {
    &self.name
  }
}

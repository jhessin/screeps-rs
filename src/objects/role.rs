//! The role is the role that a creep will take.
use crate::*;

/// This is an enum that lists the different roles
#[derive(Serialize, Deserialize, Clone)]
pub enum Role {
  /// Harvest energy and place it into Extensions, Spawns, Towers, Storage
  /// fallback: -> Upgrader
  Harvester(RoleData),
  /// Mine from source and drop on the ground on into a container.
  Miner(RoleData),
  /// Upgrade the room controller
  Upgrader(RoleData),
  /// Builds anything it finds
  /// fallback: -> Repair -> Upgrader
  Builder(RoleData),
  /// Repairs anything damaged except walls
  /// fallback: -> Upgrader
  Repairer(RoleData),
  /// Repairs walls in a tiered system by the percentage of health it has.
  /// fallback: -> Upgrader
  WallRepairer(RoleData),
  /// Ferries resources from containers or the ground and places it in
  /// Extensions, Spawns, Towers, or Storage
  /// fallback: -> Repair -> Upgrader
  Lorry(RoleData),
  /// Ferries resources between two specific locations.
  /// fallback: -> Repair -> Upgrader
  Specialist(RoleData),
}

impl PartialEq for Role {
  fn eq(&self, other: &Self) -> bool {
    match self {
      Role::Harvester(_) => {
        if let Role::Harvester(_) = other {
          true
        } else {
          false
        }
      }
      Role::Miner(_) => {
        if let Role::Miner(_) = other {
          true
        } else {
          false
        }
      }
      Role::Upgrader(_) => {
        if let Role::Upgrader(_) = other {
          true
        } else {
          false
        }
      }
      Role::Builder(_) => {
        if let Role::Builder(_) = other {
          true
        } else {
          false
        }
      }
      Role::Repairer(_) => {
        if let Role::Repairer(_) = other {
          true
        } else {
          false
        }
      }
      Role::WallRepairer(_) => {
        if let Role::WallRepairer(_) = other {
          true
        } else {
          false
        }
      }
      Role::Lorry(_) => {
        if let Role::Lorry(_) = other {
          true
        } else {
          false
        }
      }
      Role::Specialist(_) => {
        if let Role::Specialist(_) = other {
          true
        } else {
          false
        }
      }
    }
  }
}

/// This gives me to_string functionality for serialization
/// as well as easy debugging
impl Display for Role {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    match self {
      Role::Harvester(_) => write!(f, "{}", HARVESTER),
      Role::Miner(_) => write!(f, "{}", MINER),
      Role::Upgrader(_) => write!(f, "{}", UPGRADER),
      Role::Builder(_) => write!(f, "{}", BUILDER),
      Role::Repairer(_) => write!(f, "{}", REPAIRER),
      Role::WallRepairer(_) => write!(f, "{}", WALL_REPAIRER),
      Role::Lorry(_) => write!(f, "{}", LORRY),
      Role::Specialist(_) => write!(f, "{}", SPECIALIST),
    }
  }
}

const KEY: &str = "role";

/// Serialization
impl Role {
  /// Returns a MemoryReference of the current role
  pub fn memory(&self) -> MemoryReference {
    let mem = MemoryReference::new();
    if let Ok(role) = to_string(self) {
      mem.set(KEY, role);
    }
    mem
  }
}

/// Get generics of each variant
impl Role {
  /// A generic harvester
  pub fn harvester() -> Self {
    Role::Harvester(RoleData::default())
  }

  /// A generic Miner
  pub fn miner() -> Self {
    Role::Miner(RoleData::default())
  }

  /// A generic Upgrader
  pub fn upgrader() -> Self {
    Role::Upgrader(RoleData::default())
  }

  /// A generic Builder
  pub fn builder() -> Self {
    Role::Builder(RoleData::default())
  }

  /// A generic Repairer
  pub fn repairer() -> Self {
    Role::Repairer(RoleData::default())
  }

  /// A generic WallRepairer
  pub fn wall_repairer() -> Self {
    Role::WallRepairer(RoleData::default())
  }

  /// A generic Lorry
  pub fn lorry() -> Self {
    Role::Lorry(RoleData::default())
  }

  /// A generic Specialist
  pub fn specialist() -> Self {
    Role::Specialist(RoleData::default())
  }
}

/// General helper methods
impl Role {
  /// Returns a list of Roles for counting
  pub fn list() -> Vec<Role> {
    let d = RoleData::default();
    vec![
      Role::Harvester(d.clone()),
      //      Role::Miner(d.clone()),
      Role::Upgrader(d.clone()),
      Role::Builder(d.clone()),
      Role::Repairer(d.clone()),
      Role::WallRepairer(d.clone()),
      Role::Lorry(d.clone()),
      Role::Specialist(d),
    ]
  }

  /// Returns the appropriate body for this role as well as if it should be expanded.
  pub fn body(&self) -> (Vec<Part>, bool) {
    use Part::*;
    match self {
      Role::Harvester(_) => (vec![Work, Carry, Move, Move], true),
      Role::Miner(_) => (vec![Work, Work, Work, Work, Work, Move], false),
      Role::Upgrader(_) => (vec![Work, Carry, Move, Move], true),
      Role::Builder(_) => (vec![Work, Carry, Move, Move], true),
      Role::Repairer(_) => (vec![Work, Carry, Move, Move], true),
      Role::WallRepairer(_) => (vec![Work, Carry, Move, Move], true),
      Role::Lorry(_) => (vec![Carry, Carry, Move, Move], true),
      Role::Specialist(_) => (vec![Carry, Carry, Move, Move], true),
    }
  }

  /// Determines if a source has a miner attached to it.
  /// TODO: Move this to another object possibly spawner?
  pub fn has_miner(source: &Source) -> bool {
    for creep in game::creeps::values() {
      let creep = Creeper::new(creep);
      if creep.role == Role::Miner(RoleData::default()) {
        return true;
      }
    }
    false
  }

  // Runs the creep role
  // TODO: Move this to Creeper
  // pub fn run(&self, creep: &Creep) -> ReturnCode {
  //   let working = Self::is_working(&creep);
  //
  //   match self {
  //     Role::Harvester(_) => {
  //       if working {
  //         deliver_energy(creep)
  //       } else {
  //         harvest_energy(creep)
  //       }
  //     },
  //     Role::Miner(data) => {
  //       if let Some(source) = data.source() {
  //         mine(creep, &source)
  //       } else {
  //         ReturnCode::NotFound
  //       }
  //     },
  //     Role::Upgrader(_) => {
  //       if working {
  //         upgrade_controller(creep)
  //       } else {
  //         gather_energy(creep)
  //       }
  //     },
  //     Role::Builder(_) => {
  //       if working {
  //         build_nearest(creep)
  //       } else {
  //         gather_energy(creep)
  //       }
  //     },
  //     Role::Repairer(_) => {
  //       if working {
  //         repair_nearest(creep)
  //       } else {
  //         gather_energy(creep)
  //       }
  //     },
  //     Role::WallRepairer(data) => {
  //       let ratio = if let Some(ratio) = data.ratio {
  //         ratio
  //       } else {
  //         // default minimum ratio
  //         0.0001
  //       };
  //       if working {
  //         repair_wall(creep, ratio)
  //       } else {
  //         gather_energy(creep)
  //       }
  //     },
  //     Role::Lorry => {
  //       if working {
  //         deliver_energy(creep)
  //       } else {
  //         gather_energy(creep)
  //       }
  //     },
  //     Role::Specialist (data) => {
  //       let from = if let Some(from) = data.source_structure() {
  //         from
  //       } else {
  //         panic!("Specialist: {} has no source structure to harvest from", creep.name())
  //       };
  //       let to = if let Some(target) = data.target() { target } else {
  //         panic!("Specialist: {} has no destination structure", creep.name())
  //       };
  //       if working {
  //         withdraw(creep, from.as_withdrawable().unwrap())
  //       } else {
  //         transfer(creep, to.as_transferable().unwrap())
  //       }
  //     },
  //   }
  // }
}

/// Helper functions go here

/// This function handles return codes from actions
/// TODO Move to Creeper
fn handle_code<T>(creep: &Creep, code: ReturnCode, target: &T) -> ReturnCode
where
  T: RoomObjectProperties + ?Sized,
{
  if code == ReturnCode::NotInRange {
    creep.move_to(target);
    code
  } else if code != ReturnCode::Ok {
    error!("Trouble with creep action: {}: code: {:?}", creep.name(), code);
    code
  } else {
    code
  }
}

/// This is for the HARVESTER ONLY - it gathers energy directly from the source.
/// TODO Move to Creeper
fn harvest_energy(creep: &Creep) -> ReturnCode {
  // FIND the source
  let source = js! {
  let creep = @{creep};
  creep.findNearestByPath(FIND_SOURCES)
  };
  if let Some(source) = source.into_reference() {
    if let Some(source) = source.downcast::<Source>() {
      // call mine on the source
      info!("Successfully harvesting from nearest source!");
      return mine(creep, &source);
    }
  }
  ReturnCode::NotFound
}

/// This gathers any loose energy it can find
/// Every creep will use this except miner, or specialist
/// TODO Move to Creeper
fn gather_energy(creep: &Creep) -> ReturnCode {
  // prioritize targets
  // pickup: from dropped resources first
  let targets = creep.room().find(find::DROPPED_RESOURCES);
  let targets: Vec<&Resource> = targets.iter().collect();
  if !targets.is_empty() {
    if let Some(target) = _find_nearest(creep.pos(), targets) {
      let code = pickup(creep, target);
      return handle_code(creep, code, target);
    }
  }

  // withdraw: from tombstones (check store)
  let targets: Vec<Tombstone> = creep
    .room()
    .find(find::TOMBSTONES)
    .into_iter()
    .filter(|t| t.store_used_capacity(Some(ResourceType::Energy)) > 0)
    .collect();
  let targets: Vec<&Tombstone> = targets.iter().collect();
  if !targets.is_empty() {
    if let Some(target) = _find_nearest(creep.pos(), targets) {
      let code = withdraw(creep, target);
      return handle_code(creep, code, target);
    }
  }

  // withdraw: from ruins (check store)
  let targets: Vec<Ruin> = creep
    .room()
    .find(find::RUINS)
    .into_iter()
    .filter(|r| r.store_used_capacity(Some(ResourceType::Energy)) > 0)
    .collect();
  let targets: Vec<&Ruin> = targets.iter().collect();
  if !targets.is_empty() {
    if let Some(target) = _find_nearest(creep.pos(), targets) {
      let code = withdraw(creep, target);
      return handle_code(creep, code, target);
    }
  }

  // withdraw: from containers, links, storage (whatever is closer)
  let targets: Vec<Structure> = creep
    .room()
    .find(find::STRUCTURES)
    .into_iter()
    .filter(|s| {
      if let Some(store) = s.as_has_store() {
        if store.store_used_capacity(Some(ResourceType::Energy)) > 0 {
          return true;
        }
      }
      false
    })
    .collect();
  let targets: Vec<&Structure> = targets.iter().collect();
  if !targets.is_empty() {
    if let Some(target) = _find_nearest(creep.pos(), targets) {
      if let Some(target) = target.as_withdrawable() {
        let code = withdraw(creep, target);
        return handle_code(creep, code, target);
      }
    }
  }
  ReturnCode::NotFound
}

/// This will deliver the energy to the needed spots
/// TODO Move to Creeper
fn deliver_energy(creep: &Creep) -> ReturnCode {
  // prioritize targets
  // towers
  let targets: Vec<StructureTower> = creep
    .room()
    .find(find::STRUCTURES)
    .into_iter()
    .filter_map(|s| {
      if let Structure::Tower(t) = s {
        if t.store_free_capacity(Some(ResourceType::Energy)) > 0 {
          return Some(t);
        }
      }
      None
    })
    .collect();
  let targets: Vec<&StructureTower> = targets.iter().collect();
  if !targets.is_empty() {
    if let Some(target) = _find_nearest(creep.pos(), targets) {
      let code = withdraw(creep, target);
      return handle_code(creep, code, target);
    }
  }
  // extensions, spawn
  let targets: Vec<Structure> = creep
    .room()
    .find(find::STRUCTURES)
    .into_iter()
    .filter(|s| {
      match s {
        Structure::Extension(s) => {
          if s.store_free_capacity(Some(ResourceType::Energy)) > 0 {
            return true;
          }
        }
        Structure::Spawn(s) => {
          if s.store_free_capacity(Some(ResourceType::Energy)) > 0 {
            return true;
          }
        }
        _ => {}
      }
      false
    })
    .collect();
  let targets: Vec<&Structure> = targets.iter().collect();
  if !targets.is_empty() {
    if let Some(target) = _find_nearest(creep.pos(), targets) {
      let code = withdraw(creep, target.as_withdrawable().unwrap());
      return handle_code(creep, code, target);
    }
  }
  // links, storage, etc. everything else
  let targets: Vec<Structure> = creep
    .room()
    .find(find::STRUCTURES)
    .into_iter()
    .filter(|s| {
      if let Some(s) = s.as_has_store() {
        if s.store_free_capacity(Some(ResourceType::Energy)) > 0 {
          return true;
        }
      }
      false
    })
    .collect();
  let targets: Vec<&Structure> = targets.iter().collect();
  if !targets.is_empty() {
    if let Some(target) = _find_nearest(creep.pos(), targets) {
      let code = withdraw(creep, target.as_withdrawable().unwrap());
      return handle_code(creep, code, target);
    }
  }
  ReturnCode::NotFound
}

/// This will find and repair the nearest damaged structure
/// TODO Move to Creeper
fn repair_nearest(creep: &Creep) -> ReturnCode {
  // find the nearest damaged structure
  // exclude walls
  // call repair() on it.
  // TODO
  unimplemented!()
}

/// This repairs the nearest wall using the assigned ratio
/// TODO Move to Creeper
fn repair_wall(creep: &Creep, ratio: f64) -> ReturnCode {
  // use a time cycle to check for new walls (reset the ratio)
  // otherwise just search for walls within the current ratio
  // if none are found increase the ratio (check for 1.0 value)
  // find the nearest and call repair() on it.
  // TODO
  unimplemented!()
}

/// This builds the nearest construction site
/// TODO Move to Creeper
fn build_nearest(creep: &Creep) -> ReturnCode {
  // Just find the nearest construction site and call build() on it.
  // TODO
  unimplemented!()
}

/// This picks up dropped resources
/// TODO Move to Creeper
fn pickup(creep: &Creep, resource: &Resource) -> ReturnCode {
  let code = creep.pickup(resource);
  handle_code(creep, code, resource)
}

/// This gathers the energy from a given source
/// TODO Move to Creeper
fn mine(creep: &Creep, source: &Source) -> ReturnCode {
  let code = creep.harvest(source);
  handle_code(creep, code, source)
}

/// This will withdraw energy from a specific source
/// TODO Move to Creeper
fn withdraw<T>(creep: &Creep, target: &T) -> ReturnCode
where
  T: RoomObjectProperties + Withdrawable + ?Sized,
{
  let code = creep.withdraw_all(target, ResourceType::Energy);
  handle_code(creep, code, target)
}

/// This will transfer energy to a target structure
/// TODO Move to Creeper
fn transfer<T>(creep: &Creep, target: &T) -> ReturnCode
where
  T: RoomObjectProperties + Transferable + ?Sized,
{
  let code = creep.transfer_all(target, ResourceType::Energy);
  handle_code(creep, code, target)
}

/// This will repair a target structure
/// TODO Move to Creeper
fn repair<T>(creep: &Creep, target: &T) -> ReturnCode
where
  T: StructureProperties,
{
  let code = creep.repair(target);
  handle_code(creep, code, target)
}

/// This will build a construction site
/// TODO Move to Creeper
fn build(creep: &Creep, target: &ConstructionSite) -> ReturnCode {
  let code = creep.build(target);
  handle_code(creep, code, target)
}

/// This will upgrade the controller
/// TODO Move to Creeper
fn upgrade_controller(creep: &Creep) -> ReturnCode {
  let controller = creep.room().controller().unwrap();
  let code = creep.upgrade_controller(&controller);
  handle_code(creep, code, &controller)
}

/// This is a utility that helps me find the nearest object in any array of StructureProperties
/// TODO Move to Finder trait on Vec<Target>
fn _find_nearest<T>(_: Position, _: Vec<&T>) -> Option<&T>
where
  T: RoomObjectProperties + ?Sized,
{
  unimplemented!()
}

//! The role is the role that a creep will take.
use crate::*;

/// This is an enum that lists the different roles
pub enum Role {
  /// Harvest energy and place it into Extensions, Spawns, Towers, Storage
  /// fallback: -> Upgrader
  Harvester,
  /// Mine from source and drop on the ground on into a container.
  Miner(Source),
  /// Upgrade the room controller
  Upgrader,
  /// Builds anything it finds
  /// fallback: -> Repair -> Upgrader
  Builder,
  /// Repairs anything damaged except walls
  /// fallback: -> Upgrader
  Repairer,
  /// Repairs walls in a tiered system by the percentage of health it has.
  /// fallback: -> Upgrader
  WallRepairer(f64),
  /// Ferries resources from containers or the ground and places it in
  /// Extensions, Spawns, Towers, or Storage
  /// fallback: -> Repair -> Upgrader
  Lorry,
  /// Ferries resources between two specific locations.
  /// fallback: -> Repair -> Upgrader
  Specialist {
    /// from: The RawObjectId that the specialist is withdrawing from
    from: Structure,
    /// to: The RawObjectId that the specialist is depositing to
    to: Structure,
  },
}

impl Display for Role {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    match self {
      Role::Harvester => write!(f, "{}", HARVESTER),
      Role::Miner(_) => write!(f, "{}", MINER),
      Role::Upgrader => write!(f, "{}", UPGRADER),
      Role::Builder => write!(f, "{}", BUILDER),
      Role::Repairer => write!(f, "{}", REPAIRER),
      Role::WallRepairer(_) => write!(f, "{}", WALL_REPAIRER),
      Role::Lorry => write!(f, "{}", LORRY),
      Role::Specialist { .. } => write!(f, "{}", SPECIALIST),
    }
  }
}

const KEY: &str = "role";
const MINER_SRC_KEY: &str = "sourceId";
const SPEC_FROM_KEY: &str = "fromId";
const SPEC_TO_KEY: &str = "toId";
const WALL_RATIO_KEY: &str = "ratio";

impl Role {
  fn wall_ratio_key() -> &'static str {
    "ratio"
  }

  /// Returns a MemoryReference of the current role
  pub fn memory(&self) -> MemoryReference {
    let mem = MemoryReference::new();
    match self {
      Role::Miner(s) => mem.set(MINER_SRC_KEY, s.id().to_string()),
      Role::Specialist { from, to } => {
        mem.set(SPEC_FROM_KEY, from.id().to_string());
        mem.set(SPEC_TO_KEY, to.id().to_string());
      }
      _ => (),
    }
    mem.set(KEY, self.to_string());
    mem
  }

  /// Generate a role from a creeps memory
  pub fn from_creep(creep: &Creep) -> Self {
    let default = Role::Upgrader;
    if let Ok(Some(string)) = creep.memory().string(KEY) {
      match string.as_str() {
        HARVESTER => return Role::Harvester,
        MINER => {
          if let Ok(Some(source_id)) =
            creep.memory().string(MINER_SRC_KEY)
          {
            if let Ok(source_id) = ObjectId::<Source>::from_str(&source_id) {
              if let Some(source) = source_id.resolve() {
                return Role::Miner(source);
              }
            }
          }
        }
        BUILDER => return Role::Builder,
        REPAIRER => return Role::Repairer,
        WALL_REPAIRER => {
          if let Ok(Some(ratio)) = creep.memory().f64(WALL_RATIO_KEY) {
            return Role::WallRepairer(ratio);
          }
        }
        LORRY => return Role::Lorry,
        SPECIALIST => {
          if let Ok(Some(to_id)) = creep.memory().string(SPEC_TO_KEY) {
            if let Ok(Some(from_id)) =
              creep.memory().string(SPEC_FROM_KEY)
            {
              if let Ok(to_id) = ObjectId::<Structure>::from_str(&to_id) {
                if let Some(to) = to_id.resolve() {
                  if let Ok(from_id) = ObjectId::<Structure>::from_str(&from_id)
                  {
                    if let Some(from) = from_id.resolve() {
                      return Role::Specialist { from, to };
                    }
                  }
                }
              }
            }
          }
        }
        UPGRADER => return Role::Upgrader,
        _ => return default,
      }
    }

    default
  }
}

/// General helper methods and run()
impl Role {

  /// Returns the appropriate body for this role as well as if it should be expanded.
  pub fn body(&self) -> (Vec<Part>, bool) {
    use Part::*;
    match self {
      Role::Harvester => (vec![Work, Carry, Move, Move], true),
      Role::Miner(_) => (vec![Work, Work, Work, Work, Work, Move], false),
      Role::Upgrader => (vec![Work, Carry, Move, Move], true),
      Role::Builder => (vec![Work, Carry, Move, Move], true),
      Role::Repairer => (vec![Work, Carry, Move, Move], true),
      Role::WallRepairer(_) => (vec![Work, Carry, Move, Move], true),
      Role::Lorry => (vec![Carry, Carry, Move, Move], true),
      Role::Specialist { .. } => (vec![Carry, Carry, Move, Move], true),
    }
  }

  /// Determines if a source has a miner attached to it.
  pub fn has_miner(source: &Source) -> bool {
    let id = source.id().to_string();
    for creep in game::creeps::values() {
      if let Ok(Some(source)) = creep.memory().string(MINER_SRC_KEY) {
       if source == id {
         return true;
       }
      }
    }
    false
  }

  /// Is this creep working
  pub fn is_working(creep: &Creep) -> bool {
    let working = creep.memory().bool("working");

    if working && creep.store_used_capacity(Some(ResourceType::Energy)) == 0 {
      creep.memory().set("working", false);
      false
    } else if !working && creep.store_free_capacity(Some(ResourceType::Energy)) == 0 {
      creep.memory().set("working", true);
      true
    } else {
      working
    }
  }

  /// Runs the creep role
  pub fn run(&self, creep: &Creep) -> ReturnCode {
    let working = Self::is_working(&creep);

    match self {
      Role::Harvester => {
        if working {
          deliver_energy(creep)
        } else {
          harvest_energy(creep)
        }
      },
      Role::Miner(source) => {
        mine(creep, source)
      },
      Role::Upgrader => {
        if working {
          upgrade_controller(creep)
        } else {
          gather_energy(creep)
        }
      },
      Role::Builder => {
        if working {
          build_nearest(creep)
        } else {
          gather_energy(creep)
        }
      },
      Role::Repairer => {
        if working {
          repair_nearest(creep)
        } else {
          gather_energy(creep)
        }
      },
      Role::WallRepairer(ratio) => {
        if working {
          repair_wall(creep, *ratio)
        } else {
          gather_energy(creep)
        }
      },
      Role::Lorry => {
        if working {
          deliver_energy(creep)
        } else {
          gather_energy(creep)
        }
      },
      Role::Specialist { from, to } => {
        if working {
          withdraw(creep, from.as_withdrawable().unwrap())
        } else {
          transfer(creep, to.as_transferable().unwrap())
        }
      },
    }
  }
}

/// Helper functions go here

/// This function handles return codes from actions
fn handle_code<T>(creep: &Creep, code: ReturnCode, target: &T) -> ReturnCode
where T: RoomObjectProperties + ?Sized
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
fn harvest_energy(creep: &Creep) -> ReturnCode {
  // FIND the source
  let source = js! {
  let creep = @{creep};
  creep.findNearestByPath(FIND_SOURCES)
  };
  if let Some(source)= source.into_reference() {
    if let Some(source) = source.downcast::<Source>() {
      // call mine on the source
      return mine(creep, &source);
    }
  }
  ReturnCode::NotFound
}

/// This gathers any loose energy it can find
/// Every creep will use this except miner, or specialist
fn gather_energy(creep: &Creep) -> ReturnCode {
  // prioritize targets
  // pickup: from dropped resources first
  // withdraw: from tombstones (check store)
  // withdraw: from ruins (check store)
  // withdraw: from containers, links, storage (whatever is closer)
  // TODO
  unimplemented!()
}

/// This will deliver the energy to the needed spots
fn deliver_energy(creep: &Creep) -> ReturnCode {
  // prioritize targets
  // towers
  // extensions, spawn
  // links, storage, etc
  // TODO
  unimplemented!()
}

/// This will find and repair the nearest damaged structure
fn repair_nearest(creep: &Creep) -> ReturnCode {
  // find the nearest damaged structure
  // exclude walls
  // call repair() on it.
  // TODO
  unimplemented!()
}

/// This repairs the nearest wall using the assigned ratio
fn repair_wall(creep: &Creep, ratio: f64) -> ReturnCode {
  // use a time cycle to check for new walls (reset the ratio)
  // otherwise just search for walls within the current ratio
  // if none are found increase the ratio (check for 1.0 value)
  // find the nearest and call repair() on it.
  // TODO
  unimplemented!()
}

/// This builds the nearest construction site
fn build_nearest(creep: &Creep) -> ReturnCode {
  // Just find the nearest construction site and call build() on it.
  // TODO
  unimplemented!()
}

/// This picks up dropped resources
fn pickup(creep: &Creep, resource: &Resource) -> ReturnCode {
  let code = creep.pickup(resource);
  handle_code(creep, code, resource)
}

/// This gathers the energy from a given source
fn mine(creep: &Creep, source: &Source) -> ReturnCode {
  let code = creep.harvest(source);
  handle_code(creep, code, source)
}

/// This will withdraw energy from a specific source
fn withdraw<T>(creep: &Creep, target: &T) -> ReturnCode
where T: RoomObjectProperties + Withdrawable + ?Sized {
  let code = creep.withdraw_all(target, ResourceType::Energy);
  handle_code(creep, code, target)
}

/// This will transfer energy to a target structure
fn transfer<T>(creep: &Creep, target: &T) -> ReturnCode
where T: RoomObjectProperties + Transferable + ?Sized{
  let code = creep.transfer_all(target, ResourceType::Energy);
  handle_code(creep, code, target)
}

/// This will repair a target structure
fn repair<T>(creep: &Creep, target: &T) -> ReturnCode
where T: StructureProperties {
  let code = creep.repair(target);
  handle_code(creep, code, target)
}

/// This will build a construction site
fn build(creep: &Creep, target: &ConstructionSite) -> ReturnCode {
  let code = creep.build(target);
  handle_code(creep, code, target)
}

/// This will upgrade the controller
fn upgrade_controller(creep: &Creep) -> ReturnCode {
  let controller = creep.room().controller().unwrap();
  let code = creep.upgrade_controller(&controller);
  handle_code(creep, code, &controller)
}

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
}

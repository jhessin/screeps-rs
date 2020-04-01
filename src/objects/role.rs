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

impl Role {
  /// Returns the key where this is stored into the memory of a creep
  fn key() -> &'static str {
    "role"
  }

  /// Returns the key for the miner source
  fn miner_source_key() -> &'static str {
    "sourceId"
  }

  /// Returns the key for the from ID
  fn spec_from_key() -> &'static str {
    "fromId"
  }

  /// Returns the key for the to ID
  fn spec_to_key() -> &'static str {
    "toId"
  }

  fn wall_ratio_key() -> &'static str {
    "ratio"
  }

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

  /// Returns a MemoryReference of the current role
  pub fn memory(&self) -> MemoryReference {
    let mem = MemoryReference::new();
    match self {
      Role::Miner(s) => mem.set(Self::miner_source_key(), s.id().to_string()),
      Role::Specialist { from, to } => {
        mem.set(Self::spec_from_key(), from.id().to_string());
        mem.set(Self::spec_to_key(), to.id().to_string());
      }
      _ => (),
    }
    mem.set(Self::key(), self.to_string());
    mem
  }

  /// Generate a role from a creeps memory
  pub fn from_creep(creep: &Creep) -> Self {
    let default = Role::Upgrader;
    if let Ok(Some(string)) = creep.memory().string(Self::key()) {
      match string.as_str() {
        HARVESTER => return Role::Harvester,
        MINER => {
          if let Ok(Some(source_id)) =
            creep.memory().string(Self::miner_source_key())
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
          if let Ok(Some(ratio)) = creep.memory().f64(Self::wall_ratio_key()) {
            return Role::WallRepairer(ratio);
          }
        }
        LORRY => return Role::Lorry,
        SPECIALIST => {
          if let Ok(Some(to_id)) = creep.memory().string(Self::spec_to_key()) {
            if let Ok(Some(from_id)) =
              creep.memory().string(Self::spec_from_key())
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

//! The role is the role that a creep will take.
use crate::*;

/// This is an enum that lists the different roles
#[derive(
  Serialize, Deserialize, Clone, Copy, Eq, PartialEq, Ord, PartialOrd,
)]
pub enum Role {
  /// Harvest energy and place it into Extensions, Spawns, Towers, Storage
  /// fallback: -> Upgrader
  Harvester,
  /// Mine from source and drop on the ground on into a container.
  Miner,
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
  WallRepairer,
  /// Ferries resources from containers or the ground and places it in
  /// Extensions, Spawns, Towers, or Storage
  /// fallback: -> Repair -> Upgrader
  Lorry,
  /// Ferries resources between two specific locations.
  /// fallback: -> Repair -> Upgrader
  Specialist,
  /// This is a claimer to claim new rooms
  Claimer,
}

/// This gives me to_string functionality for serialization
/// as well as easy debugging
impl Display for Role {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    write!(f, "{}", to_string(self).unwrap())
  }
}

/// General helper methods
impl Role {
  /// Returns the appropriate body for this role as well as if it should be expanded.
  pub fn body(&self) -> (Vec<Part>, bool) {
    use Part::*;
    match self {
      Role::Harvester => (vec![Work, Carry, Move, Move], true),
      Role::Miner => (vec![Work, Work, Work, Work, Work, Move], false),
      Role::Upgrader => (vec![Work, Carry, Move, Move], true),
      Role::Builder => (vec![Work, Carry, Move, Move], true),
      Role::Repairer => (vec![Work, Carry, Move, Move], true),
      Role::WallRepairer => (vec![Work, Carry, Move, Move], true),
      Role::Lorry => (vec![Carry, Move], true),
      Role::Specialist => (vec![Carry, Carry, Move, Move], true),
      Role::Claimer => (vec![Claim, Move], false),
    }
  }

  /// This works the role
  pub fn run(&self, creep: &Creep) -> ReturnCode {
    todo!("Run each role here")
  }
}

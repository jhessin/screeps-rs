use crate::*;

/// The roles of a creep
#[derive(Serialize, Deserialize, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum Role {
  /// Categorized by equal parts Work, Carry, and Move
  Worker,
  /// Only has a single Carry part for repairs of containers
  Miner,
  /// Only has Carry and Move parts
  Hauler,
  /// Only a single Move part
  Scout,
  /// MMO inspired combat
  /// Tough, Move, Attack
  Tank,
  /// Move, Attack, RangedAttack
  Ranger,
  /// Move, RangedAttack, Heal
  Cleric,
}

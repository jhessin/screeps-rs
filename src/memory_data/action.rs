use crate::*;

/// This enumerates all possible actions a creep can take
#[derive(Serialize, Deserialize, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum Action {
  /// Harvest a Resource, Mineral, or Deposit
  /// Requires Work part
  Harvest,
  /// Attack or RangedAttack a Creep, PowerCreep, or Structure
  /// Requires Attack or RangedAttack part
  Attack,
  /// Attack a Controller,
  /// Requires Claim part
  AttackController,
  /// Build a Construction Site
  /// Requires Work part
  Build,
  /// Claim a Controller
  /// Requires Claim part
  Claim,
  /// Dismantle a Structure
  /// Requires Work part
  Dismantle,
  /// Generate Safe Mode on a Controller
  /// Requires Carry part and 1000 Ghodium
  GenerateSafeMode,
  /// Heal a target Creep or PowerCreep
  /// Requires Heal part
  Heal,
  /// Pickup a Dropped Resource
  /// Requires Carry part
  Pickup,
  /// Pull a Creep or PowerCreep target
  /// Requires Move part
  Pull,
  /// Repair a Structure
  /// Requires Work part as well as a Carry part
  Repair,
  /// Reserve a Controller
  /// Requires a Claim part
  Reserve,
  /// Transfer to a Structure
  /// Requires a Work part as well as a Carry part
  Transfer,
  /// Withdraw from a Structure, Tombstone, or Deposit
  /// Requires a Carry part
  Withdraw,
}

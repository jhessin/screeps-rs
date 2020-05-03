use crate::*;

/// This enumerates all possible actions a creep can take
#[derive(Serialize, Deserialize, Hash, Debug, Eq, PartialEq)]
pub enum Action {
  /// Harvest a Resource, Mineral, or Deposit
  /// Requires Work part
  Harvest,
  /// Mine a Resource
  Mine,
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
  /// Simply scout out a target and add roads
  /// Requires a Move part
  Scout,
  /// Upgrade a Controller
  /// Requires Work, Carry, Move
  Upgrade,
}

impl Action {
  /// Get the required body parts for an action
  pub fn req_parts(&self) -> HashSet<Part> {
    match self {
      Action::Harvest => vec![Work, Carry, Move],
      Action::Mine => vec![Work, Carry, Move],
      Action::Attack => vec![Attack, RangedAttack, Move],
      Action::AttackController => vec![Claim, Move],
      Action::Build => vec![Carry, Work, Move],
      Action::Claim => vec![Claim, Move],
      Action::Dismantle => vec![Work, Move],
      Action::GenerateSafeMode => vec![Carry, Move],
      Action::Heal => vec![Heal, Move],
      Action::Pickup => vec![Carry, Move],
      Action::Repair => vec![Work, Carry, Move],
      Action::Reserve => vec![Claim, Move],
      Action::Transfer => vec![Carry, Move],
      Action::Withdraw => vec![Carry, Move],
      Action::Scout => vec![Move],
      Action::Upgrade => vec![Work, Carry, Move],
    }
    .into_iter()
    .collect()
  }

  /// Get the ticks required for this action
  pub fn ticks_req(&self, creep: &CommonCreepData, target: &Target) -> u32 {
    // TODO
    match self {
      Action::Harvest => {
        let harvest_power = creep.harvesting_power();
        if harvest_power == 0 {
          std::u32::MAX
        } else {
          creep.carry_capacity() / harvest_power
            + creep.pos().get_range_to(target)
        }
      }
      Action::Mine => std::u32::MAX,
      Action::Attack => creep.attack_power(),
      Action::AttackController => 0,
      Action::Build => 0,
      Action::Claim => 0,
      Action::Dismantle => 0,
      Action::GenerateSafeMode => 0,
      Action::Heal => 0,
      Action::Pickup => 0,
      Action::Repair => 0,
      Action::Reserve => 0,
      Action::Transfer => 0,
      Action::Withdraw => 0,
      Action::Scout => 0,
      Action::Upgrade => 0,
    }
  }
}

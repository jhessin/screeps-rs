use crate::*;

/// All variants that hold a transferable RoomObject
pub enum TransferTarget {
  /// Structures
  Structure(Structure),
  /// Creeps
  Creep(Creep),
  /// PowerCreeps
  PowerCreep(PowerCreep),
}

/// All variants that can be Withdrawn
pub enum WithdrawTarget {
  /// Structures
  Structure(Structure),
  /// Tombstones
  Tombstone(Tombstone),
  /// Ruins
  Ruin(Ruin),
}

/// All variants that can be Harvested
pub enum HarvestTarget {
  /// Sources
  Source(Source),
  /// Minerals
  Mineral(Mineral),
  /// Deposits
  Deposit(Deposit),
}

type AttackTarget = TransferTarget;

/// All variants that a soldier can rally to
pub enum RallyTarget {
  /// The rally flag
  Flag(Flag),
  /// My Rampart structure
  Rampart(StructureRampart),
}

/// This is the finder trait for implementing methods on the Position type
pub trait Finder {
  /// This is the missing method from typescript
  fn find_closest_by_path(&self, targets: Vec<Target>) -> Option<Target>;

  /// These methods use up energy
  /// A repair target
  fn find_repair_target(&self) -> Option<Structure>;
  /// A build target
  fn find_build_target(&self) -> Option<ConstructionSite>;
  /// A transferable target we should fill up first
  fn find_transfer_target_primary(
    &self,
    resource: Option<ResourceType>,
  ) -> Option<TransferTarget>;
  /// A transferable target we should fill up last
  fn find_transfer_target_secondary(
    &self,
    resource: Option<ResourceType>,
  ) -> Option<TransferTarget>;

  /// These things give us energy or other resources
  /// A target to dismantle
  fn find_dismantle_target(&self) -> Option<Structure>;
  /// A harvest target
  fn find_harvest_target(
    &self,
    resource: Option<ResourceType>,
  ) -> Option<HarvestTarget>;
  /// A pickup target
  fn find_pickup_target(
    &self,
    resource: Option<ResourceType>,
  ) -> Option<Resource>;
  /// A withdraw target we should pull from first
  fn find_withdraw_target_primary(
    &self,
    resource: Option<ResourceType>,
  ) -> Option<WithdrawTarget>;
  /// A withdraw target we should only pull from last
  fn find_withdraw_target_secondary(
    &self,
    resource: Option<ResourceType>,
  ) -> Option<WithdrawTarget>;

  /// Things that require a Claim part
  /// Claiming of course
  fn find_claim_target(&self) -> Option<StructureController>;
  /// reserving also requires at least 1 claim part.
  fn find_reserve_target(&self) -> Option<StructureController>;

  /// Things that require Attack or Ranged Attack part
  /// Attacking
  fn find_attack_target(&self) -> Option<AttackTarget>;
  /// Should we use a ranged_mass_attack?
  fn should_mass_attack(&self) -> bool;
  /// Find a good rally position
  fn find_rally_point(&self) -> Option<RallyTarget>;

  /// Things that require a heal part
  fn find_heal_target(&self) -> Option<AttackTarget>;

  /// Other things
  fn find_pull_target(&self) -> Option<Creep>;
  /// Find a target to sign
  fn find_sign_target(&self) -> Option<StructureController>;
}

impl Finder for Position {
  fn find_closest_by_path(&self, targets: Vec<Target>) -> Option<Target> {
    if targets.is_empty() {
      return None;
    }

    let mut nearest: Option<Target> = None;
    let mut nearest_cost = std::u32::MAX;

    for target in targets {
      let pos = match &target {
        Target::Source(t) => t.pos(),
        Target::Structure(t) => t.pos(),
        Target::Tombstone(t) => t.pos(),
        Target::Ruin(t) => t.pos(),
        Target::Resource(t) => t.pos(),
        Target::ConstructionSite(t) => t.pos(),
        Target::Creep(t) => t.pos(),
      };

      let result = search(self, &pos, std::u32::MAX, SearchOptions::default());
      if !result.incomplete && result.cost < nearest_cost {
        nearest_cost = result.cost;
        nearest = Some(target);
      }
    }

    nearest
  }

  fn find_repair_target(&self) -> Option<Structure> {
    todo!()
  }

  fn find_build_target(&self) -> Option<ConstructionSite> {
    todo!()
  }

  fn find_transfer_target_primary(
    &self,
    resource: Option<ResourceType>,
  ) -> Option<TransferTarget> {
    todo!()
  }

  fn find_transfer_target_secondary(
    &self,
    resource: Option<ResourceType>,
  ) -> Option<TransferTarget> {
    todo!()
  }

  fn find_dismantle_target(&self) -> Option<Structure> {
    todo!()
  }

  fn find_harvest_target(
    &self,
    resource: Option<ResourceType>,
  ) -> Option<HarvestTarget> {
    todo!()
  }

  fn find_pickup_target(
    &self,
    resource: Option<ResourceType>,
  ) -> Option<Resource> {
    todo!()
  }

  fn find_withdraw_target_primary(
    &self,
    resource: Option<ResourceType>,
  ) -> Option<WithdrawTarget> {
    todo!()
  }

  fn find_withdraw_target_secondary(
    &self,
    resource: Option<ResourceType>,
  ) -> Option<WithdrawTarget> {
    todo!()
  }

  fn find_claim_target(&self) -> Option<StructureController> {
    todo!()
  }

  fn find_reserve_target(&self) -> Option<StructureController> {
    todo!()
  }

  fn find_attack_target(&self) -> Option<AttackTarget> {
    todo!()
  }

  fn should_mass_attack(&self) -> bool {
    todo!()
  }

  fn find_rally_point(&self) -> Option<RallyTarget> {
    todo!()
  }

  fn find_heal_target(&self) -> Option<AttackTarget> {
    todo!()
  }

  fn find_pull_target(&self) -> Option<Creep> {
    todo!()
  }

  fn find_sign_target(&self) -> Option<StructureController> {
    todo!()
  }
}

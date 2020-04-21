use crate::Keys::TargetId;
use crate::*;
use screeps::Part::Move;

const ROOM_SIZE: u32 = 79;

/// All variants that hold a transferable RoomObject
pub enum TransferTarget {
  /// Structures
  Structure(Structure),
  /// Creeps
  Creep(Creep),
  /// PowerCreeps
  PowerCreep(PowerCreep),
}

impl TransferTarget {
  pub fn into_target(self) -> Target {
    match self {
      TransferTarget::Structure(t) => Target::Structure(t),
      TransferTarget::Creep(t) => Target::Creep(t),
      TransferTarget::PowerCreep(t) => Target::PowerCreep(t),
    }
  }

  pub fn as_transferable(&self) -> Option<&dyn Transferable> {
    match self {
      TransferTarget::Structure(t) => t.as_transferable(),
      TransferTarget::Creep(t) => t,
      TransferTarget::PowerCreep(t) => t,
    }
  }
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

impl WithdrawTarget {
  pub fn into_target(self) -> Target {
    match self {
      WithdrawTarget::Structure(t) => Target::Structure(t),
      WithdrawTarget::Tombstone(t) => Target::Tombstone(t),
      WithdrawTarget::Ruin(t) => Target::Ruin(t),
    }
  }

  pub fn as_withdrawable(&self) -> Option<&dyn Withdrawable> {
    match self {
      WithdrawTarget::Structure(t) => t.as_withdrawable(),
      WithdrawTarget::Tombstone(t) => t,
      WithdrawTarget::Ruin(t) => t,
    }
  }
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

impl HarvestTarget {
  fn into_target(self) -> Target {
    match self {
      HarvestTarget::Source(t) => Target::Source(t),
      HarvestTarget::Mineral(t) => Target::Mineral(t),
      HarvestTarget::Deposit(t) => Target::Deposit(t),
    }
  }

  pub fn as_harvestable(&self) -> &dyn Harvestable {
    match self {
      HarvestTarget::Source(t) => t,
      HarvestTarget::Mineral(t) => t,
      HarvestTarget::Deposit(t) => t,
    }
  }
}

impl Target {
  fn into_transfer_target(self) -> Option<TransferTarget> {
    match self {
      Target::Structure(t) => Some(TransferTarget::Structure(t)),
      Target::Creep(t) => Some(TransferTarget::Creep(t)),
      Target::PowerCreep(t) => Some(TransferTarget::PowerCreep(t)),
      _ => None,
    }
  }

  fn into_withdraw_target(self) -> Option<WithdrawTarget> {
    match self {
      Target::Structure(t) => Some(WithdrawTarget::Structure(t)),
      Target::Ruin(t) => Some(WithdrawTarget::Ruin(t)),
      Target::Tombstone(t) => Some(WithdrawTarget::Tombstone(t)),
      _ => None,
    }
  }

  fn into_harvest_target(self) -> Option<HarvestTarget> {
    match self {
      Target::Source(t) => Some(HarvestTarget::Source(t)),
      Target::Mineral(t) => Some(HarvestTarget::Mineral(t)),
      Target::Deposit(t) => Some(HarvestTarget::Deposit(t)),
      _ => None,
    }
  }

  fn into_rally_target(self) -> Option<RallyTarget> {
    match self {
      Target::Structure(Structure::Rampart(t)) => Some(RallyTarget::Rampart(t)),
      Target::Flag(t) => Some(RallyTarget::Flag(t)),
      _ => None,
    }
  }
}

type AttackTarget = TransferTarget;

/// All variants that a soldier can rally to
pub enum RallyTarget {
  /// The rally flag
  Flag(Flag),
  /// My Rampart structure
  Rampart(StructureRampart),
}

impl RallyTarget {
  fn into_target(self) -> Target {
    match self {
      RallyTarget::Flag(t) => Target::Flag(t),
      RallyTarget::Rampart(t) => Target::Structure(Structure::Rampart(t)),
    }
  }
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
        Target::Mineral(t) => t.pos(),
        Target::Deposit(t) => t.pos(),
        Target::PowerCreep(t) => t.pos(),
        Target::Flag(t) => t.pos(),
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
    let targets: Vec<Target> = self
      .find_in_range(find::CONSTRUCTION_SITES, ROOM_SIZE)
      .into_iter()
      .map(|s| Target::ConstructionSite(s))
      .collect();

    if let Some(Target::ConstructionSite(target)) =
      self.find_closest_by_path(targets)
    {
      return Some(target);
    }
    None
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
    const DISMANTLE_PATH: &str = "dismantle";

    // First add targets using flags
    if let Some(flag) = game::flags::get(DISMANTLE_PATH) {
      if let Some(s) = flag.pos().find_in_range(find::STRUCTURES, 0).get(0)
        as Option<Structure>
      {
        let mut arr = if let Ok(Some(arr)) =
          screeps::memory::root().path_arr(DISMANTLE_PATH)
        {
          arr as Vec<String>
        } else {
          vec![]
        };
        arr.push(s.id().to_string());
        screeps::memory::root().path_set(DISMANTLE_PATH, arr);
      }
    }

    // Get the closest target
    let mut targets: Vec<Target> = vec![];
    if let Some(mut target_ids) =
      screeps::memory::root().path_arr(DISMANTLE_PATH) as Option<Vec<String>>
    {
      for id in target_ids {
        if let Ok(target) = ObjectId::<Structure>::from_str(&id) {
          if let Some(target) = target.resolve() {
            if !target.has_creep() {
              targets.push(Target::Structure(target));
            }
          }
        } else {
          // invalid target remove from memory
          target_ids.remove(id.parse().unwrap());
          screeps::memory::root().path_set(DISMANTLE_PATH, target_ids.clone());
        }
      }
    }

    if let Some(Target::Structure(target)) = self.find_closest_by_path(targets)
    {
      return Some(target);
    }
    None
  }

  fn find_harvest_target(
    &self,
    resource: Option<ResourceType>,
  ) -> Option<HarvestTarget> {
    let room = game::rooms::get(self.room_name()).unwrap();
    match resource {
      Some(ResourceType::Energy) => {
        let sources: Vec<Target> = room
          .find(find::SOURCES_ACTIVE)
          .into_iter()
          .filter_map(|s| {
            if s.has_creep() {
              None
            } else {
              Some(Target::Source(s))
            }
          })
          .collect();
        if let Some(target) = self.find_closest_by_path(sources) {
          target.into_harvest_target()
        } else {
          None
        }
      }
      Some(resource) => {
        let mut targets: Vec<Target> = room
          .find(find::MINERALS)
          .into_iter()
          .filter_map(|s: Mineral| {
            if s.mineral_type() == ResourceType && !s.has_creep() {
              Some(Target::Mineral(s))
            } else {
              None
            }
          })
          .collect();
        for deposit in room.find(find::DEPOSITS) as Vec<Deposit> {
          if deposit.deposit_type() == resource && !deposit.has_creep() {
            targets.push(Target::Deposit(deposit));
          }
        }

        if let Some(target) = self.find_closest_by_path(targets) {
          target.into_harvest_target()
        } else {
          None
        }
      }
      None => {
        let mut targets: Vec<Target> = room
          .find(find::SOURCES_ACTIVE)
          .into_iter()
          .filter_map(|s: Source| {
            if s.has_creep() {
              None
            } else {
              Some(Target::Source(s))
            }
          })
          .collect();

        for m in room.find(find::MINERALS) as Vec<Mineral> {
          if !m.has_creep() {
            targets.push(Target::Mineral(m));
          }
        }

        for d in room.find(find::DEPOSITS) as Vec<Deposit> {
          if !d.has_creep() {
            targets.push(Target::Deposit(d));
          }
        }

        if let Some(target) = self.find_closest_by_path(targets) {
          target.into_harvest_target()
        } else {
          None
        }
      }
    }
  }

  fn find_pickup_target(
    &self,
    resource: Option<ResourceType>,
  ) -> Option<Resource> {
    let room = game::rooms::get(self.room_name()).unwrap();
    let targets: Vec<Target> = if let Some(resource) = resource {
      room
        .find(find::DROPPED_RESOURCES)
        .into_iter()
        .filter_map(|s: Resource| {
          if s.resource_type() == resource && !s.has_creep() {
            Some(Target::Resource(s))
          } else {
            None
          }
        })
        .collect()
    } else {
      room.find(find::DROPPED_RESOURCES).into_iter().filter_map(|s| {
        if s.has_creep() {
          None
        } else {
          Some(Target::Resource(s))
        }
      })
    };

    if let Some(Target::Resource(t)) = self.find_closest_by_path(targets) {
      Some(t)
    } else {
      None
    }
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
    for room in game::rooms::values() {
      if let Some(ctrl) = room.controller() as Option<StructureController> {
        if !ctrl.my() && !ctrl.has_owner() {
          if let Some(res) = ctrl.reservation() {
            // TODO find a better way to do this
            if res.username
              == game::spawns::get("Spawn1").unwrap().owner_name().unwrap()
            {
              return Some(ctrl);
            }
          } else {
            return Some(ctrl);
          }
        }
      }
    }
    None
  }

  fn find_reserve_target(&self) -> Option<StructureController> {
    todo!()
  }

  fn find_attack_target(&self) -> Option<AttackTarget> {
    let room = game::rooms::get(self.room_name()).unwrap();
    let mut targets = room
      .find(find::HOSTILE_CREEPS)
      .into_iter()
      .map(|s| Target::Creep(s))
      .collect() as Vec<Target>;

    for target in room
      .find(find::HOSTILE_POWER_CREEPS)
      .into_iter()
      .map(|s| Target::PowerCreep(s))
    {
      targets.push(target);
    }

    for target in room
      .find(find::HOSTILE_STRUCTURES)
      .into_iter()
      .map(|s| Target::Structure(s))
    {
      targets.push(target);
    }

    if let Some(target) = self.find_closest_by_path(targets) {
      target.into_transfer_target()
    } else {
      None
    }
  }

  fn should_mass_attack(&self) -> bool {
    self.find_in_range(find::HOSTILE_CREEPS, 3).len()
      + self.find_in_range(find::HOSTILE_POWER_CREEPS, 3).len()
      + self.find_in_range(find::HOSTILE_STRUCTURES, 3).len()
      > 1
  }

  fn find_rally_point(&self) -> Option<RallyTarget> {
    todo!()
  }

  fn find_heal_target(&self) -> Option<AttackTarget> {
    todo!()
  }

  fn find_pull_target(&self) -> Option<Creep> {
    let room = game::rooms::get(self.room_name()).unwrap();
    let targets = room
      .find(find::MY_CREEPS)
      .into_iter()
      .filter_map(|s: Creep| {
        if (s.has_creep() || s.get_active_bodyparts(Move) > 0) {
          None
        } else {
          Some(Target::Creep(s))
        }
      })
      .collect() as Vec<Target>;
    if let Some(Target::Creep(c)) = self.find_closest_by_path(targets) {
      Some(c)
    } else {
      None
    }
  }

  fn find_sign_target(&self) -> Option<StructureController> {
    todo!()
  }
}

use crate::*;
use screeps::Part::Move;

const ROOM_SIZE: u32 = 79;

/// This is the finder trait for implementing methods on the Position type
pub trait Finder {
  /// This simply finds anything in the room
  fn find<T: find::FindConstant>(&self, c: T) -> Vec<T::Item>;

  /// This is the missing method from typescript
  fn find_closest_by_path<T: SizedRoomObject + HasPosition + ?Sized>(
    &self,
    targets: Vec<T>,
  ) -> Option<T>;

  /// These methods use up energy
  /// A repair target
  fn find_repair_target(&self) -> Option<Structure>;
  /// A build target
  fn find_build_target(&self) -> Option<ConstructionSite>;
  /// A transferable target we should fill up first
  fn find_transfer_target_primary(
    &self,
    resource: Option<ResourceType>,
  ) -> Option<Reference>;
  /// A transferable target we should fill up last
  fn find_transfer_target_secondary(
    &self,
    resource: Option<ResourceType>,
  ) -> Option<Reference>;

  /// These things give us energy or other resources
  /// A target to dismantle
  fn find_dismantle_target(&self) -> Option<Structure>;
  /// A harvest target
  fn find_harvest_target(
    &self,
    resource: Option<ResourceType>,
  ) -> Option<Reference>;
  /// A pickup target
  fn find_pickup_target(
    &self,
    resource: Option<ResourceType>,
  ) -> Option<Resource>;
  /// A withdraw target we should pull from first
  fn find_withdraw_target_primary(
    &self,
    resource: Option<ResourceType>,
  ) -> Option<Reference>;
  /// A withdraw target we should only pull from last
  fn find_withdraw_target_secondary(
    &self,
    resource: Option<ResourceType>,
  ) -> Option<Reference>;

  /// Things that require a Claim part
  /// Claiming of course
  fn find_claim_target(&self) -> Option<StructureController>;
  /// reserving also requires at least 1 claim part.
  fn find_reserve_target(&self) -> Option<StructureController>;

  /// Things that require Attack or Ranged Attack part
  /// Attacking
  fn find_attack_target(&self) -> Option<Reference>;
  /// Should we use a ranged_mass_attack?
  fn should_mass_attack(&self) -> bool;
  /// Find a good rally position
  fn find_rally_point(&self) -> Option<Reference>;

  /// Things that require a heal part
  fn find_heal_target(&self) -> Option<Reference>;

  /// Other things
  fn find_pull_target(&self) -> Option<Creep>;
  /// Find a target to sign
  fn find_sign_target(&self) -> Option<StructureController>;
}

impl Finder for Position {
  fn find<T: find::FindConstant>(&self, c: T) -> Vec<T::Item> {
    self.find_in_range(c, ROOM_SIZE)
  }

  fn find_closest_by_path<T: SizedRoomObject + HasPosition + ?Sized>(
    &self,
    targets: Vec<T>,
  ) -> Option<T> {
    if targets.is_empty() {
      return None;
    }

    let mut nearest: Option<T> = None;
    let mut nearest_cost = std::u32::MAX;

    for target in targets {
      let result =
        search(self, &target, std::u32::MAX, SearchOptions::default());
      if !result.incomplete && result.cost < nearest_cost {
        nearest_cost = result.cost;
        nearest = Some(target);
      }
    }

    nearest
  }

  fn find_repair_target(&self) -> Option<Structure> {
    todo!("repair_target")
  }

  fn find_build_target(&self) -> Option<ConstructionSite> {
    let targets: Vec<ConstructionSite> =
      self.find_in_range(find::CONSTRUCTION_SITES, ROOM_SIZE);

    if let Some(target) = self.find_closest_by_path(targets) {
      return Some(target);
    }
    None
  }

  fn find_transfer_target_primary(
    &self,
    resource: Option<ResourceType>,
  ) -> Option<Reference> {
    todo!("transfer_primary")
  }

  fn find_transfer_target_secondary(
    &self,
    resource: Option<ResourceType>,
  ) -> Option<Reference> {
    todo!("transfer_secondary")
  }

  fn find_dismantle_target(&self) -> Option<Structure> {
    const DISMANTLE_PATH: &str = "dismantle";

    // First add targets using flags
    if let Some(flag) = game::flags::get(DISMANTLE_PATH) {
      if let Some(s) = flag.pos().find_in_range(find::STRUCTURES, 0).get(0) {
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
    let mut targets: Vec<Structure> = vec![];
    if let Ok(Some(mut target_ids)) =
      screeps::memory::root().path_arr::<String>(DISMANTLE_PATH)
    {
      for id in target_ids.clone() {
        if let Ok(target) = ObjectId::<Structure>::from_str(&id) {
          if let Some(target) = target.resolve() {
            if !target.has_creep() {
              targets.push(target);
            }
          }
        } else {
          // invalid target remove from memory
          target_ids.remove(id.parse().unwrap());
          screeps::memory::root().path_set(DISMANTLE_PATH, target_ids.clone());
        }
      }
    }

    self.find_closest_by_path(targets)
  }

  fn find_harvest_target(
    &self,
    resource: Option<ResourceType>,
  ) -> Option<Reference> {
    match resource {
      Some(ResourceType::Energy) => {
        let sources: Vec<RoomObject> = self
          .find(find::SOURCES_ACTIVE)
          .into_iter()
          .filter_map(|s| {
            if s.has_creep() {
              None
            } else {
              s.as_ref().clone().downcast()
            }
          })
          .collect();
        if let Some(target) = self.find_closest_by_path(sources) {
          Some(target.as_ref().clone())
        } else {
          None
        }
      }
      Some(resource) => {
        let mut targets: Vec<RoomObject> = self
          .find(find::MINERALS)
          .into_iter()
          .filter_map(|s: Mineral| {
            if s.mineral_type() == resource && !s.has_creep() {
              s.as_ref().clone().downcast()
            } else {
              None
            }
          })
          .collect();
        for deposit in self.find(find::DEPOSITS) as Vec<Deposit> {
          if deposit.deposit_type() == resource && !deposit.has_creep() {
            if let Some(d) = deposit.as_ref().clone().downcast() {
              targets.push(d);
            }
          }
        }

        if let Some(target) = self.find_closest_by_path(targets) {
          Some(target.as_ref().clone())
        } else {
          None
        }
      }
      None => {
        let mut targets: Vec<RoomObject> = self
          .find(find::SOURCES_ACTIVE)
          .into_iter()
          .filter_map(|s: Source| {
            if s.has_creep() {
              None
            } else {
              s.as_ref().clone().downcast()
            }
          })
          .collect();

        for m in self.find(find::MINERALS) as Vec<Mineral> {
          if !m.has_creep() {
            if let Some(m) = m.as_ref().clone().downcast() {
              targets.push(m);
            }
          }
        }

        for d in self.find(find::DEPOSITS) as Vec<Deposit> {
          if !d.has_creep() {
            if let Some(d) = d.as_ref().clone().downcast() {
              targets.push(d);
            }
          }
        }

        if let Some(target) = self.find_closest_by_path(targets) {
          Some(target.as_ref().clone())
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
    let targets: Vec<Resource> = if let Some(resource) = resource {
      self
        .find(find::DROPPED_RESOURCES)
        .into_iter()
        .filter(|s| s.resource_type() == resource && !s.has_creep())
        .collect()
    } else {
      self
        .find(find::DROPPED_RESOURCES)
        .into_iter()
        .filter(|s| !s.has_creep())
        .collect()
    };

    if let Some(t) = self.find_closest_by_path(targets) {
      Some(t)
    } else {
      None
    }
  }

  fn find_withdraw_target_primary(
    &self,
    resource: Option<ResourceType>,
  ) -> Option<Reference> {
    todo!("withdraw_primary")
  }

  fn find_withdraw_target_secondary(
    &self,
    resource: Option<ResourceType>,
  ) -> Option<Reference> {
    todo!("withdraw_secondary")
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
    todo!("reserve_target")
  }

  fn find_attack_target(&self) -> Option<Reference> {
    let mut targets = self
      .find(find::HOSTILE_CREEPS)
      .into_iter()
      .filter_map(|s| s.as_ref().clone().downcast())
      .collect::<Vec<RoomObject>>();

    for target in self
      .find(find::HOSTILE_POWER_CREEPS)
      .into_iter()
      .filter_map(|s| s.as_ref().clone().downcast())
    {
      targets.push(target);
    }

    for target in self.find(find::HOSTILE_STRUCTURES) as Vec<OwnedStructure> {
      if let Some(t) = target.as_ref().clone().downcast() {
        targets.push(t);
      }
    }

    if let Some(target) = self.find_closest_by_path(targets) {
      Some(target.as_ref().clone())
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

  fn find_rally_point(&self) -> Option<Reference> {
    todo!("rally_target")
  }

  fn find_heal_target(&self) -> Option<Reference> {
    let mut targets = self
      .find(find::MY_CREEPS)
      .into_iter()
      .filter_map(|s| {
        if s.hits() < s.hits_max() && !s.has_creep() {
          // Some((*s.as_ref()).into_expected_type().unwrap())
          s.as_ref().clone().downcast()
        } else {
          None
        }
      })
      .collect::<Vec<RoomObject>>();

    // Power creeps don't work for some reason
    for structure in self.find(find::STRUCTURES) {
      if let Some(s) = structure.as_attackable() {
        if s.hits() < s.hits_max() && !structure.has_creep() {
          if let Some(s) = structure.as_ref().clone().downcast() {
            targets.push(s)
          }
        }
      }
    }
    if let Some(target) = self.find_closest_by_path(targets) {
      Some(target.as_ref().clone())
    } else {
      None
    }
  }

  fn find_pull_target(&self) -> Option<Creep> {
    let targets = self
      .find(find::MY_CREEPS)
      .into_iter()
      .filter_map(|s| {
        if s.has_creep() || s.get_active_bodyparts(Move) > 0 {
          None
        } else {
          Some(s)
        }
      })
      .collect::<Vec<Creep>>();
    self.find_closest_by_path(targets)
  }

  fn find_sign_target(&self) -> Option<StructureController> {
    todo!("sign_target")
  }
}

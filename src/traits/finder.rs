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
  /// A wall repair target
  fn find_wall_repair_target(&self) -> Option<Structure>;
  /// A build target
  fn find_build_target(&self) -> Option<ConstructionSite>;
  /// A transferable target we should fill up first
  fn find_transfer_target_primary(
    &self,
    resource: Option<ResourceType>,
  ) -> Option<Structure>;
  /// A transferable target we should fill up last
  fn find_transfer_target_secondary(
    &self,
    resource: Option<ResourceType>,
  ) -> Option<Structure>;

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
  ) -> Option<Structure>;

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
    let targets: Vec<Structure> = self
      .find(find::MY_STRUCTURES)
      .into_iter()
      .filter_map(|s| {
        let s = s.as_structure();
        if let Some(atk) = s.as_attackable() {
          if atk.hits() < atk.hits_max() {
            return Some(s);
          }
        }
        None
      })
      .collect();

    self.find_closest_by_path(targets)
  }

  fn find_wall_repair_target(&self) -> Option<Structure> {
    let mut walls: Vec<Structure> = self
      .find(find::STRUCTURES)
      .into_iter()
      .filter(|s| match s {
        Structure::Wall(_) | Structure::Rampart(_) => {
          let attack = s.as_attackable().unwrap();
          attack.hits() < attack.hits_max()
        }
        _ => false,
      })
      .collect();

    // check if there are no walls
    if walls.is_empty() {
      return None;
    }

    let mut target = walls.pop().unwrap();

    while walls.len() > 0 {
      let next = walls.pop().unwrap();
      if next.as_attackable().unwrap().hits()
        < target.as_attackable().unwrap().hits()
      {
        target = next;
      }
    }

    Some(target)
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
  ) -> Option<Structure> {
    let targets: Vec<Structure> = self
      .find(find::MY_STRUCTURES)
      .into_iter()
      .filter_map(|s| {
        let s = s.as_structure() as Structure;
        match s {
          Structure::Spawn(_)
          | Structure::Extension(_)
          | Structure::Tower(_) => {
            if let Some(store) = s.as_has_store() {
              if store.store_free_capacity(resource) > 0 {
                return Some(s);
              }
            }
          }
          _ => return None,
        }
        None
      })
      .collect();

    self.find_closest_by_path(targets)
  }

  fn find_transfer_target_secondary(
    &self,
    resource: Option<ResourceType>,
  ) -> Option<Structure> {
    let targets: Vec<Structure> = self
      .find(find::MY_STRUCTURES)
      .into_iter()
      .filter_map(|s| {
        let s = s.as_structure() as Structure;
        if let Some(store) = s.as_has_store() {
          if store.store_free_capacity(resource) > 0 {
            return Some(s);
          }
        }
        None
      })
      .collect();

    self.find_closest_by_path(targets)
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
    let mut targets: Vec<RoomObject> = self
      .find(find::TOMBSTONES)
      .into_iter()
      .filter_map(|s| {
        let s = s as Tombstone;
        if s.has_creep() {
          return None;
        }
        if s.store_used_capacity(resource) > 0 {
          if let Some(s) = s.as_ref().clone().downcast() {
            return Some(s);
          }
        }
        None
      })
      .collect();

    for target in self.find(find::RUINS) as Vec<Ruin> {
      if target.has_creep() {
        continue;
      }
      if target.has_creep() {
        continue;
      }
      if target.store_used_capacity(resource) > 0 {
        if let Some(s) = target.as_ref().clone().downcast() {
          targets.push(s);
        }
      }
    }

    for container in self.find(find::STRUCTURES) as Vec<Structure> {
      if container.has_creep() {
        continue;
      }
      if let Structure::Container(c) = container {
        if let Some(s) = c.as_ref().clone().downcast() {
          targets.push(s);
        }
      }
    }

    if let Some(target) = self.find_closest_by_path(targets) {
      Some(target.as_ref().clone())
    } else {
      None
    }
  }

  fn find_withdraw_target_secondary(
    &self,
    resource: Option<ResourceType>,
  ) -> Option<Structure> {
    let targets: Vec<Structure> = self
      .find(find::MY_STRUCTURES)
      .into_iter()
      .filter_map(|s| {
        let s = s.as_structure() as Structure;
        match s {
          Structure::Link(_) | Structure::Storage(_) => {
            if let Some(store) = s.as_has_store() {
              if store.store_used_capacity(resource) > 0 {
                return Some(s);
              }
            }
            None
          }
          _ => None,
        }
      })
      .collect();

    self.find_closest_by_path(targets)
  }

  fn find_claim_target(&self) -> Option<StructureController> {
    for room in game::rooms::values() {
      if let Some(ctrl) = room.controller() as Option<StructureController> {
        if !ctrl.my() && !ctrl.has_owner() {
          if let Some(res) = ctrl.reservation() {
            if let Some(Values::Username(username)) =
              screeps::memory::root().get_value(Keys::Username)
            {
              if res.username == username {
                return Some(ctrl);
              }
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
    let username = if let Some(Values::Username(u)) =
      screeps::memory::root().get_value(Keys::Username)
    {
      u
    } else {
      panic!("Username not found in memory!")
    };
    for room in game::rooms::values() {
      if let Some(ctrl) = room.controller() as Option<StructureController> {
        if ctrl.my() {
          return None;
        }
        if ctrl.owner_name().is_none() {
          if let Some(res) = ctrl.reservation() {
            return if res.username == username { Some(ctrl) } else { None };
          }
          return Some(ctrl);
        }
      }
    }
    None
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
    let ramparts: Vec<StructureRampart> = self
      .find(find::MY_STRUCTURES)
      .into_iter()
      .filter_map(|s| {
        let s = s.as_structure() as Structure;
        if let Structure::Rampart(s) = s {
          return if s.has_creep() { None } else { Some(s) };
        }
        None
      })
      .collect();

    if let Some(target) = self.find_closest_by_path(ramparts) {
      return Some(target.as_ref().clone());
    }

    if let Some(flag) = game::flags::get("rally") {
      return Some(flag.as_ref().clone());
    }

    None
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
    let username = if let Some(Values::Username(u)) =
      screeps::memory::root().get_value(Keys::Username)
    {
      u
    } else {
      panic!("Username not found in memory!")
    };
    for room in game::rooms::values() {
      if let Some(ctrl) = room.controller() as Option<StructureController> {
        if let Some(sign) = ctrl.sign() {
          if sign.username == username {
            // Don't resign
            return None;
          }
          if ctrl.my() {
            // Sign it if it's yours.
            return Some(ctrl);
          }
          if !ctrl.has_owner() && ctrl.reservation().is_none() {
            // Sign it if it isn't owned or reserved
            return Some(ctrl);
          }
          if let Some(res) = ctrl.reservation() {
            if res.username == username {
              return Some(ctrl);
            }
          }
        }
      }
    }
    None
  }
}

/// This is used on a room to make finding creeps with a particular role easier.
pub trait CreepFinder {
  /// Return all the creeps in a room with a particular role
  fn creeps_with_role(&self, role: Role) -> Vec<Creep>;

  /// Return all the creeps without a role
  fn idle_creeps(&self) -> Vec<Creep>;
}

impl CreepFinder for Room {
  fn creeps_with_role(&self, role: Role) -> Vec<Creep> {
    self
      .find(find::MY_CREEPS)
      .into_iter()
      .filter(|s| {
        if let Some(Values::Role(r)) = s.memory().get_value(Keys::Role) {
          r == role
        } else {
          false
        }
      })
      .collect()
  }

  fn idle_creeps(&self) -> Vec<Creep> {
    self
      .find(find::MY_CREEPS)
      .into_iter()
      .filter(|s| s.memory().get_value(Keys::Role).is_none())
      .collect()
  }
}

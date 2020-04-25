use crate::*;
use screeps::Part::Move;
use screeps::StructureType;
use std::ops::Deref;

const ROOM_SIZE: u32 = 49;

// TODO Move this into it's own struct and give Creeper an instance of it named pos?
/// This is the finder trait for implementing methods on the Position type
pub struct Finder {
  /// The position that the finder is finding paths for
  pub pos: Position,
}

impl Deref for Finder {
  type Target = Position;

  fn deref(&self) -> &Self::Target {
    &self.pos
  }
}

impl Finder {
  /// Create a new finder from a game object with a position
  pub fn new<T: HasPosition>(thing: &T) -> Self {
    let pos = thing.pos();
    Finder { pos }
  }

  /// find all the items in a room
  pub fn find<T: find::FindConstant>(&self, c: T) -> Vec<T::Item> {
    self.find_in_range(c, ROOM_SIZE)
  }

  /// find the closest within an array of id strings
  pub fn find_closest_id_by_path(
    &self,
    targets: Vec<String>,
  ) -> Option<String> {
    if targets.is_empty() {
      return None;
    }

    let mut nearest_id: Option<String> = None;
    let mut nearest_cost = std::u32::MAX;

    for target in targets {
      if let Some(t) = target.as_room_object() {
        let result =
          search(&self.pos, &t, std::u32::MAX, SearchOptions::default());
        if result.incomplete {
          trace!("Couldn't find a path! cost: {}", result.cost);
        }
        if result.cost < nearest_cost {
          nearest_cost = result.cost;
          nearest_id = Some(target);
        }
      }
    }

    nearest_id
  }

  /// find the closest item from an array of items by lowest cost path
  pub fn find_closest_by_path<T: SizedRoomObject + HasPosition + ?Sized>(
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
        search(&self.pos, &target, std::u32::MAX, SearchOptions::default());
      if result.incomplete {
        trace!("Couldn't find a path! cost: {}", result.cost);
      }
      if result.cost < nearest_cost {
        nearest_cost = result.cost;
        nearest = Some(target);
      }
    }

    nearest
  }

  /// Find a repair target
  pub fn find_repair_target(&self) -> Option<Structure> {
    let targets: Vec<Structure> = self
      .find(find::MY_STRUCTURES)
      .into_iter()
      .filter_map(|s| {
        let s = s.as_structure() as Structure;
        // Do not repair walls or ramparts
        match s {
          Structure::Wall(_) | Structure::Rampart(_) => return None,
          _ => (),
        }
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

  /// find the most damaged wall
  pub fn find_wall_repair_target(&self) -> Option<Structure> {
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

  /// find a build target
  pub fn find_build_target(
    &self,
    structure_type: Option<StructureType>,
  ) -> Option<ConstructionSite> {
    let targets: Vec<ConstructionSite> = self
      .find(find::CONSTRUCTION_SITES)
      .into_iter()
      .filter(|s| {
        if let Some(s_type) = structure_type {
          // looking for a particular structure
          s.structure_type() == s_type
        } else {
          // Do not build walls or ramparts - wall repairer should do that.
          s.structure_type() != StructureType::Rampart
            && s.structure_type() != StructureType::Wall
        }
      })
      .collect();

    if let Some(target) = self.find_closest_by_path(targets) {
      return Some(target);
    }
    None
  }

  /// find a transfer target (primary)
  pub fn find_transfer_target_primary(
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

  /// find a transfer target (secondary)
  pub fn find_transfer_target_secondary(
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

  /// find a dismantle target
  pub fn find_dismantle_target(&self) -> Option<Structure> {
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

  /// find a harvest target
  pub fn find_source_target(&self) -> Option<Source> {
    let sources: Vec<Source> = self
      .find(find::SOURCES_ACTIVE)
      .into_iter()
      .filter_map(|s| {
        if s.has_creep_with_role(Role::Miner) {
          trace!("id: {} has a miner - skipping", s.id().to_string());
          return None;
        }
        trace!("id: {} found!", s.id());
        Some(s)
      })
      .collect();
    trace!("{} sources found", sources.len());
    self.find_closest_by_path(sources)
  }

  /// find the closest mineral target
  pub fn find_mineral_target(
    &self,
    resource: Option<ResourceType>,
  ) -> Option<Mineral> {
    let targets: Vec<Mineral> = self
      .find(find::MINERALS)
      .into_iter()
      .filter_map(|s: Mineral| {
        if s.has_creep_with_role(Role::Miner) {
          return None;
        } else if let Some(resource) = resource {
          if resource == s.mineral_type() {
            if s.pos().find_in_range(find::STRUCTURES, 0).len() > 0 {
              return Some(s);
            }
          }
        }
        None
      })
      .collect();

    self.find_closest_by_path(targets)
  }

  /// Find the closest deposit target
  pub fn find_deposit_target(
    &self,
    resource: Option<ResourceType>,
  ) -> Option<Deposit> {
    trace!("Searching for a harvest target for {:?}", resource);
    let targets: Vec<Deposit> = self
      .find(find::DEPOSITS)
      .into_iter()
      .filter_map(|s: Deposit| {
        if s.has_creep_with_role(Role::Miner) {
          return None;
        }
        if let Some(resource) = resource {
          if s.deposit_type() == resource {
            return Some(s);
          }
        }
        None
      })
      .collect();

    self.find_closest_by_path(targets)
  }

  /// find a dropped resource
  pub fn find_pickup_target(
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

  /// Find a tombstone target
  pub fn find_tombstone_target(
    &self,
    resource: Option<ResourceType>,
  ) -> Option<Tombstone> {
    let targets: Vec<Tombstone> = self
      .find(find::TOMBSTONES)
      .into_iter()
      .filter_map(|s| {
        if s.has_creep() {
          return None;
        }
        if s.store_used_capacity(resource) > 0 {
          Some(s)
        } else {
          None
        }
      })
      .collect();

    self.find_closest_by_path(targets)
  }

  /// find a ruin target
  pub fn find_ruin_target(
    &self,
    resource: Option<ResourceType>,
  ) -> Option<Ruin> {
    let targets: Vec<Ruin> = self
      .find(find::RUINS)
      .into_iter()
      .filter_map(|s| {
        if s.has_creep() {
          return None;
        }
        if s.store_used_capacity(resource) > 0 {
          Some(s)
        } else {
          None
        }
      })
      .collect();

    self.find_closest_by_path(targets)
  }

  /// find a withdraw target (primary)
  pub fn find_withdraw_target_primary(
    &self,
    resource: Option<ResourceType>,
  ) -> Option<Structure> {
    let targets: Vec<Structure> = self
      .find(find::STRUCTURES)
      .into_iter()
      .filter_map(|s| {
        if let Structure::Container(c) = &s {
          if c.store_used_capacity(resource) > 0 {
            return Some(s);
          }
        }
        None
      })
      .collect();

    self.find_closest_by_path(targets)
  }

  /// find a withdraw target (secondary)
  pub fn find_withdraw_target_secondary(
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

  /// find a claimable room
  pub fn find_claim_target(&self) -> Option<StructureController> {
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

  /// find a reservable room
  pub fn find_reserve_target(&self) -> Option<StructureController> {
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

  /// find an attackable target
  pub fn find_enemy_creeps(&self) -> Option<Creep> {
    let targets = self.find(find::HOSTILE_CREEPS);
    self.find_closest_by_path(targets)
  }
  /// find an attackable power creep
  pub fn find_enemy_power_creep(&self) -> Option<PowerCreep> {
    let targets = self.find(find::HOSTILE_POWER_CREEPS);
    self.find_closest_by_path(targets)
  }

  /// find an attackable structure
  pub fn find_enemy_structure(&self) -> Option<OwnedStructure> {
    let targets = self.find(find::HOSTILE_STRUCTURES);

    self.find_closest_by_path(targets)
  }

  /// should a creep mass attack?
  pub fn should_mass_attack(&self) -> bool {
    self.find_in_range(find::HOSTILE_CREEPS, 3).len()
      + self.find_in_range(find::HOSTILE_POWER_CREEPS, 3).len()
      + self.find_in_range(find::HOSTILE_STRUCTURES, 3).len()
      > 1
  }

  /// find a rampart
  pub fn find_rampart_rally(&self) -> Option<StructureRampart> {
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

    self.find_closest_by_path(ramparts)
  }

  /// find a rally point
  pub fn find_rally_point(&self) -> Option<Flag> {
    if let Some(flag) = game::flags::get("rally") {
      Some(flag)
    } else {
      None
    }
  }

  /// find a heal target
  pub fn find_heal_target(&self) -> Option<String> {
    let mut targets = self
      .find(find::MY_CREEPS)
      .into_iter()
      .filter_map(|s| {
        if s.hits() < s.hits_max() && !s.has_creep() {
          Some(s.id().to_string())
        } else {
          None
        }
      })
      .collect::<Vec<String>>();

    for creep in self.find(find::MY_POWER_CREEPS) {
      if creep.hits() < creep.hits_max() && !creep.has_creep() {
        targets.push(creep.id().to_string())
      }
    }

    self.find_closest_id_by_path(targets)
  }

  /// find a pull target
  pub fn find_pull_target(&self) -> Option<Creep> {
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
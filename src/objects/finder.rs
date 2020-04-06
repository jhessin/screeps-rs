//! Easy path finding tool it wraps up the position of a thing
//! then provides tools to find the nearest other thing.

use screeps::ResourceType::Energy;

use crate::*;

/// Easy path management
pub struct Finder {
  /// The origin for navigation
  origin: Position,
  /// The room to find objects in.
  room: Room,
}

impl Finder {
  /// Returns a new path given anything that has a position
  pub fn new<T>(pos: T) -> Self
  where
    T: HasPosition + RoomObjectProperties,
  {
    let room = pos.room();
    let origin = pos.pos();
    Finder { origin, room }
  }

  /// This finds the nearest Target object.
  pub fn find_nearest(&self, targets: Vec<Target>) -> Option<Target> {
    if targets.is_empty() {
      return None;
    }

    let mut nearest = targets[0].clone();
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

      let result =
        search(&self.origin, &pos, std::u32::MAX, SearchOptions::default());
      if !result.incomplete && result.cost < nearest_cost {
        nearest_cost = result.cost;
        // for memory cleanup
        drop(nearest);
        nearest = target.clone();
      }
    }

    Some(nearest)
  }

  /// This finds the nearest energy for work
  pub fn find_nearest_energy_for_work(&self) -> Option<Target> {
    let mut targets: Vec<Target> = vec![];

    // Lorries that are working
    // if let Some(t) = self.find_nearest_working_lorry() {
    //   targets.push(t);
    // }

    // Dropped Resources
    if let Some(t) = self.find_nearest_dropped_resource() {
      targets.push(t);
    }

    // Tombstones
    if let Some(t) = self.find_nearest_tombstone() {
      targets.push(t);
    }

    // Ruins
    if let Some(t) = self.find_nearest_ruin() {
      targets.push(t);
    }

    // if !targets.is_empty() {
    //   return self.find_nearest(targets);
    // }

    // From any structure
    // other than Towers/Spawns/Extensions
    if let Some(t) = self.find_nearest_other_energy_source() {
      targets.push(t);
    }

    self.find_nearest(targets)
  }

  /// Used to find energy for storage.
  /// Primarily loose energy or energy from other lorries who are working.
  pub fn find_nearest_energy_to_store(&self) -> Option<Target> {
    let mut targets: Vec<Target> = vec![];

    // Other working lorries
    // if let Some(t) = self.find_nearest_working_lorry() {
    //   targets.push(t);
    // }

    // dropped resources
    if let Some(t) = self.find_nearest_dropped_resource() {
      targets.push(t);
    }

    // Tombstones
    if let Some(t) = self.find_nearest_tombstone() {
      targets.push(t);
    }

    // Ruins
    if let Some(t) = self.find_nearest_ruin() {
      targets.push(t);
    }

    // Containers
    if let Some(t) = self.find_nearest_container_with_energy() {
      targets.push(t);
    }

    self.find_nearest(targets)
  }

  /// Find the nearest energy target
  /// This is strictly for lorries
  pub fn find_nearest_energy_target(&self) -> Option<Target> {
    let mut targets = vec![];

    if let Some(t) = self.find_nearest_spawn_extension_needing_energy() {
      targets.push(t);
    }
    if let Some(t) = self.find_nearest_tower_needing_energy() {
      targets.push(t);
    }

    if !targets.is_empty() {
      return self.find_nearest(targets);
    }

    if let Some(t) = self.find_nearest_storage() {
      targets.push(t);
    }

    if !targets.is_empty() {
      return self.find_nearest(targets);
    }

    if let Some(t) = self.find_nearest_other_energy_target() {
      targets.push(t);
    }

    self.find_nearest(targets)
  }

  /// Find working lorries (or harvesters)
  pub fn find_nearest_working_lorry(&self) -> Option<Target> {
    let mut targets = vec![];

    for c in self.room.find(find::MY_CREEPS) {
      let mut creep = Creeper::new(c.clone());
      if creep.working()
        && (creep.role == Role::lorry() || creep.role == Role::harvester())
      {
        targets.push(Target::Creep(c));
      }
    }

    self.find_nearest(targets)
  }

  /// Returns the nearest structure store that doesn't use it's own energy
  pub fn find_nearest_other_energy_source(&self) -> Option<Target> {
    let mut targets: Vec<Target> = vec![];

    for s in self.room.find(find::STRUCTURES) {
      match s {
        // Exceptions: These use their own energy.
        Structure::Spawn(_) => continue,
        Structure::PowerSpawn(_) => continue,
        Structure::Extension(_) => continue,
        Structure::Tower(_) => continue,
        _ => {
          if let Some(store) = s.as_has_store() {
            if store.store_used_capacity(Some(Energy)) > 0 {
              targets.push(Target::Structure(s));
            }
          }
        }
      }
    }

    self.find_nearest(targets)
  }

  /// Returns the nearest energy target
  pub fn find_nearest_other_energy_target(&self) -> Option<Target> {
    let mut targets = vec![];

    for s in self.room.find(find::STRUCTURES) {
      if let Structure::Container(_) = s {
        continue;
      }

      if let Some(store) = s.as_has_store() {
        if store.store_free_capacity(Some(Energy)) > 0 {
          targets.push(Target::Structure(s));
        }
      }
    }

    self.find_nearest(targets)
  }

  /// find the nearest container with energy
  pub fn find_nearest_container_with_energy(&self) -> Option<Target> {
    let mut targets = vec![];

    for s in self.room.find(find::STRUCTURES) {
      if let Structure::Container(c) = &s {
        if c.store_used_capacity(Some(Energy)) > 0 {
          targets.push(Target::Structure(s));
        }
      }
    }

    self.find_nearest(targets)
  }

  /// Strictly for HARVESTERS - find the nearest active source
  pub fn find_nearest_active_source(&self) -> Option<Target> {
    let mut targets: Vec<Target> = vec![];

    // this should include dropped resources, ruins, and tombstones
    // if let Some(t) = self.find_nearest_working_lorry() {
    //   targets.push(t);
    // }

    if let Some(t) = self.find_nearest_dropped_resource() {
      targets.push(t);
    }

    if !targets.is_empty() {
      return self.find_nearest(targets);
    }

    if let Some(t) = self.find_nearest_tombstone() {
      targets.push(t);
    }

    if !targets.is_empty() {
      return self.find_nearest(targets);
    }

    if let Some(t) = self.find_nearest_ruin() {
      targets.push(t);
    }

    if !targets.is_empty() {
      return self.find_nearest(targets);
    }

    if let Some(t) = self.find_nearest_container_with_energy() {
      targets.push(t);
    }

    if !targets.is_empty() {
      return self.find_nearest(targets);
    }

    for s in self.room.find(find::SOURCES_ACTIVE) {
      targets.push(Target::Source(s));
    }

    self.find_nearest(targets)
  }

  /// Find the nearest dropped resource
  pub fn find_nearest_dropped_resource(&self) -> Option<Target> {
    let mut targets: Vec<Target> = vec![];

    for r in self.room.find(find::DROPPED_RESOURCES) {
      if !r.has_creep() {
        targets.push(Target::Resource(r));
      }
    }

    self.find_nearest(targets)
  }

  /// find the nearest tombstone
  pub fn find_nearest_tombstone(&self) -> Option<Target> {
    let mut targets: Vec<Target> = vec![];

    for t in self.room.find(find::TOMBSTONES) {
      if t.store_used_capacity(Some(Energy)) > 0 {
        targets.push(Target::Tombstone(t));
      }
    }

    self.find_nearest(targets)
  }

  /// find the nearest ruin
  pub fn find_nearest_ruin(&self) -> Option<Target> {
    let mut targets: Vec<Target> = vec![];

    for r in self.room.find(find::RUINS) {
      if r.store_used_capacity(Some(Energy)) > 0 {
        targets.push(Target::Ruin(r));
      }
    }

    self.find_nearest(targets)
  }

  /// Returns the nearest energy storage for lorries/harvesters
  /// Also finds other creeps that are not working.
  pub fn find_nearest_storage(&self) -> Option<Target> {
    let mut targets: Vec<Target> = vec![];

    for s in self.room.find(find::STRUCTURES) {
      match s {
        // Exclude containers as they are only temporary.
        Structure::Container(_) => continue,
        _ => {
          if let Some(store) = s.as_has_store() {
            if store.store_free_capacity(Some(Energy)) > 0 {
              targets.push(Target::Structure(s));
            }
          }
        }
      }
    }

    // for c in self.room.find(find::MY_CREEPS) {
    //   let mut creep = Creeper::new(c);
    //
    //   if !creep.working() && creep.role != Role::miner() {
    //     targets.push(Target::Creep(creep.creep));
    //   }
    // }

    self.find_nearest(targets)
  }

  /// Towers - this finds only towers that need energy
  pub fn find_nearest_tower_needing_energy(&self) -> Option<Target> {
    let mut targets: Vec<Target> = vec![];

    for s in self.room.find(find::STRUCTURES) {
      if let Structure::Tower(t) = &s {
        if t.store_free_capacity(Some(Energy)) > 0 {
          targets.push(Target::Structure(s));
        }
      }
    }

    self.find_nearest(targets)
  }

  /// Spawn/Extensions - this finds the nearest spawn or extension that needs energy
  pub fn find_nearest_spawn_extension_needing_energy(&self) -> Option<Target> {
    let mut targets: Vec<Target> = vec![];

    for s in self.room.find(find::STRUCTURES) {
      match &s {
        Structure::Spawn(spawn) => {
          if spawn.store_free_capacity(Some(Energy)) > 0 {
            targets.push(Target::Structure(s));
          }
        }
        Structure::Extension(x) => {
          if x.store_free_capacity(Some(Energy)) > 0 {
            targets.push(Target::Structure(s));
          }
        }
        Structure::PowerSpawn(spawn) => {
          if spawn.store_free_capacity(Some(Energy)) > 0 {
            targets.push(Target::Structure(s));
          }
        }
        _ => (),
      }
    }

    self.find_nearest(targets)
  }

  /// Construction Sites
  pub fn find_nearest_construction_site(&self) -> Option<Target> {
    let mut targets: Vec<Target> = vec![];

    for c in self.room.find(find::CONSTRUCTION_SITES) {
      targets.push(Target::ConstructionSite(c));
    }

    self.find_nearest(targets)
  }

  /// Repair targets
  pub fn find_nearest_repair_target(&self) -> Option<Target> {
    let mut targets: Vec<Target> = vec![];

    for t in self.room.find(find::STRUCTURES) {
      if let Structure::Wall(_) = t {
        continue;
      }
      if let Some(s) = t.as_attackable() {
        if s.hits() < s.hits_max() {
          targets.push(Target::Structure(t));
        }
      }
    }

    self.find_nearest(targets)
  }

  /// Finds the nearest wall repair target that is below a certain life ratio.
  pub fn find_nearest_wall_repair_target(&self, ratio: f64) -> Option<Target> {
    let mut targets: Vec<Target> = vec![];

    for s in self.room.find(find::STRUCTURES) {
      if let Structure::Tower(t) = &s {
        let hits = t.hits() as f64;
        let max = t.hits_max() as f64;
        if hits / max < ratio {
          targets.push(Target::Structure(s));
        }
      }
    }

    self.find_nearest(targets)
  }

  /// Returns true if there are any repairable walls
  pub fn has_repairable_walls(&self) -> bool {
    let walls: Vec<Structure> = self
      .room
      .find(find::STRUCTURES)
      .into_iter()
      .filter(|s| {
        if let Structure::Wall(wall) = s {
          wall.hits() < wall.hits_max()
        } else {
          false
        }
      })
      .collect();

    !walls.is_empty()
  }
}

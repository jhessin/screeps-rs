//! Holds the Spawner that wraps a spawn to make it useful.
use crate::*;

/// This wraps a structure spawn and gives it superpowers!
pub struct Spawner {
  /// the spawn that this is controlling
  spawn: StructureSpawn,
  room: Room,
}

impl Display for Spawner {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    write!(
      f,
      "Spawner: {} in Room: {}",
      self.spawn.name(),
      self.room.name_local()
    )
  }
}

impl Spawner {
  /// Returns the cost of a creep
  pub fn body_cost(body: &[Part]) -> u32 {
    body.iter().map(|p| p.cost()).sum()
  }

  /// This expands a body to fill a room or
  fn expand_body(&self, body: &[Part]) -> Vec<Part> {
    let capacity = self.room.energy_capacity_available();
    let num_parts = Self::body_cost(body) / capacity;

    let mut parts = vec![];
    for part in Vec::from(body) {
      for _ in 1..num_parts {
        parts.push(part);
      }
    }

    parts
  }

  /// This expands only as much as can currently be afforded.
  fn emergency_expand_body(&self, body: &[Part]) -> Vec<Part> {
    let capacity = self.room.energy_available();
    let num_parts = Self::body_cost(body) / capacity;

    let mut parts = vec![];
    for part in Vec::from(body) {
      for _ in 1..num_parts {
        parts.push(part);
      }
    }

    parts
  }

  /// This gets an available name
  pub fn get_available_name() -> &'static str {
    'name: for name in NAMES.iter() {
      for creep in game::creeps::keys() {
        if name == &creep {
          continue 'name;
        }
      }
      return *name;
    }
    ""
  }
  /// This spawns a creep with a given role
  pub fn spawn(&self, role: Role) -> ReturnCode {
    let (body, expand) = role.body();

    let body = if expand { self.expand_body(&body) } else { body };

    let name = Self::get_available_name();
    let opts = SpawnOptions::new().memory(role.memory());

    self.spawn.spawn_creep_with_options(&body, name, &opts)
  }

  /// Spawn a creep with whatever energy is available
  pub fn emergency_spawn(&self, role: Role) -> ReturnCode {
    let (body, expand) = role.body();
    let body = if expand { self.emergency_expand_body(&body) } else { body };
    let name = Self::get_available_name();
    let opts = SpawnOptions::new().memory(role.memory());
    self.spawn.spawn_creep_with_options(&body, name, &opts)
  }
}

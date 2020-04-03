//! Holds the Spawner that wraps a spawn to make it useful.
use crate::*;

/// This wraps a structure spawn and gives it superpowers!
pub struct Spawner {
  /// the spawn that this is controlling
  pub spawn: StructureSpawn,
  /// The room the spawn is in.
  pub room: Room,
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
  /// Get a new spawner
  pub fn new(spawn: StructureSpawn) -> Self {
    let room = spawn.room();
    Spawner { spawn, room }
  }

  /// Get the minimum for a specific role
  pub fn get_min(&self, role: &Role) -> u32 {
    // The default if there isn't one set.
    let default = 1;

    // get the value from memory
    if let Ok(role_string) = to_string(&role) {
      let path = format!("{}.{}", "roles", role_string);
      if let Ok(Some(min)) = self.spawn.memory().get_path::<u32>(&path) {
        return min;
      }
    }

    // otherwise set the default
    self.set_min(role.clone(), default);
    default
  }

  /// Sets the minimum of a particular role
  pub fn set_min(&self, role: Role, size: u32) {
    if let Ok(role_string) = to_string(&role) {
      let path = format!("{}.{}", "roles", role_string);
      self.spawn.memory().path_set(&path, size);
    }
  }

  /// Returns the cost of a creep
  pub fn body_cost(body: &[Part]) -> u32 {
    body.iter().map(|p| p.cost()).sum()
  }

  /// This expands a body to fill a room or
  fn expand_body(&self, body: &[Part]) -> Vec<Part> {
    debug!("Expanding body {:?}", body);
    debug!("Initial cost {}", Self::body_cost(body));
    let capacity = self.room.energy_capacity_available();
    debug!("Energy capacity is {}", capacity);
    let num_parts = capacity / Self::body_cost(body);
    debug!("Number of each part is {}", num_parts);

    let mut parts = vec![];
    for part in Vec::from(body) {
      for _ in 1..num_parts as u32 {
        parts.push(part);
      }
    }

    parts
  }

  /// This expands only as much as can currently be afforded.
  fn emergency_expand_body(&self, body: &[Part]) -> Vec<Part> {
    let capacity = self.room.energy_available();
    let num_parts = capacity / Self::body_cost(body);

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
    debug!("Spawning {}...", role);
    let (body, expand) = role.body();
    debug!("--with body model {:?}", body.as_slice());

    let body = if expand { self.expand_body(&body) } else { body };

    let name = Self::get_available_name();
    let opts = SpawnOptions::new().memory(role.memory());

    debug!("Spawning creep named: {}", name);
    self.spawn.spawn_creep_with_options(body.as_slice(), name, &opts)
  }

  /// Spawn a creep with whatever energy is available
  pub fn emergency_spawn(&self, role: Role) -> ReturnCode {
    let (body, expand) = role.body();
    let body = if expand { self.emergency_expand_body(&body) } else { body };
    let name = Self::get_available_name();
    let opts = SpawnOptions::new().memory(role.memory());
    self.spawn.spawn_creep_with_options(&body, name, &opts)
  }

  /// Spawn creeps as necessary
  pub fn spawn_as_needed(&self) -> ReturnCode {
    // Find determine if miners are needed and if we can afford them.
    let (miner_body, _) = Role::miner().body();
    let miner_cost = Self::body_cost(&miner_body);

    // first build a list of all the roles.
    let mut role_map: HashMap<String, Vec<Creeper>> = HashMap::new();

    // then iterate through all the creeps and get their roles.
    for creep in game::creeps::values() {
      let creeper = Creeper::new(creep);
      let entry =
        role_map.entry(creeper.role.to_string()).or_insert_with(|| vec![]);
      entry.push(creeper);
    }

    // For debugging purposes output the creeps by name
    for (key, value) in &role_map {
      if let Some(creeper) = value.get(0) {
        let role = &creeper.role;
        info!(
          "{} of {} {}: {:?}",
          value.len(),
          self.get_min(&role.clone()),
          key,
          value.iter().map(|c| c.creep.name()).collect::<Vec<String>>()
        );
      }
    }

    if self.room.energy_capacity_available() >= miner_cost {
      // self.set_min(Role::harvester(), 0);
      // self.set_min(Role::lorry(), 1);
      // building miners
      // check each source and assign a miner to each one.
      // set the number of harvesters to 0
    }

    // spawn harvesters if necessary
    let role = Role::harvester();
    if let Some(creeps) = role_map.get(&role.to_string()) {
      if creeps.len() < self.get_min(&role) as usize {
        return self.spawn(role);
      }
    } else if self.get_min(&role) > 0 {
      // spawn some because there are none.
      return self.emergency_spawn(role);
    }

    // spawn upgraders if necessary
    let role = Role::upgrader();
    if let Some(creeps) = role_map.get(&role.to_string()) {
      if creeps.len() < self.get_min(&role) as usize {
        return self.spawn(role);
      }
    } else if self.get_min(&role) > 0 {
      // spawn some because there are none.
      return self.spawn(role);
    }

    // spawn builder if necessary
    let role = Role::builder();
    if let Some(creeps) = role_map.get(&role.to_string()) {
      if creeps.len() < self.get_min(&role) as usize {
        return self.spawn(role);
      }
    } else if self.get_min(&role) > 0 {
      // spawn some because there are none.
      return self.spawn(role);
    }

    // spawn repairer if necessary
    let role = Role::repairer();
    if let Some(creeps) = role_map.get(&role.to_string()) {
      if creeps.len() < self.get_min(&role) as usize {
        return self.spawn(role);
      }
    } else if self.get_min(&role) > 0 {
      // spawn some because there are none.
      return self.spawn(role);
    }

    // spawn wall_repairer if necessary
    let role = Role::wall_repairer();
    if let Some(creeps) = role_map.get(&role.to_string()) {
      if creeps.len() < self.get_min(&role) as usize {
        return self.spawn(role);
      }
    } else if self.get_min(&role) > 0 {
      // spawn some because there are none.
      return self.spawn(role);
    }

    // spawn lorry if necessary
    let role = Role::lorry();
    if let Some(creeps) = role_map.get(&role.to_string()) {
      if creeps.len() < self.get_min(&role) as usize {
        return self.spawn(role);
      }
    } else if self.get_min(&role) > 0 {
      // spawn some because there are none.
      return self.spawn(role);
    }

    ReturnCode::Full
  }
}

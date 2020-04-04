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
  pub fn get_min(&self, role: &Role) -> usize {
    // The default if there isn't one set.
    let default = 1;

    // get the value from memory

    let path = format!("{}.{}", "roles", role.to_string());
    if let Ok(Some(min)) = self.spawn.memory().get_path::<usize>(&path) {
      return min as usize;
    }

    // otherwise set the default
    self.set_min(role.clone(), default);
    default as usize
  }

  /// Sets the minimum of a particular role
  pub fn set_min(&self, role: Role, size: u32) {
    let path = format!("{}.{}", "roles", role.to_string());
    self.spawn.memory().path_set(&path, size);
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
      for _ in 0..num_parts as u32 {
        parts.push(part);
      }
    }

    parts
  }

  /// This expands only as much as can currently be afforded.
  fn emergency_expand_body(&self, body: &[Part]) -> Vec<Part> {
    let capacity = self.room.energy_available();
    debug!("Current capacity: {}", capacity);
    let num_parts = capacity / Self::body_cost(body);
    debug!("num-parts: {}", num_parts);

    let mut parts = vec![];
    for part in Vec::from(body) {
      for _ in 0..num_parts {
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
    debug!("Expanded body: {:?}", body.as_slice());

    let name = Self::get_available_name();
    let opts = SpawnOptions::new().memory(role.memory());

    debug!("Spawning creep named: {}", name);
    self.spawn.spawn_creep_with_options(body.as_slice(), name, &opts)
  }

  /// Spawn a creep with whatever energy is available
  pub fn emergency_spawn(&self, role: Role) -> ReturnCode {
    debug!("Emergency Spawning {}...", role);
    let (body, expand) = role.body();
    let body = if expand { self.emergency_expand_body(&body) } else { body };
    debug!("Expanded body: {:?}", body.as_slice());
    let name = Self::get_available_name();
    debug!("Creep name: {}", name);
    let opts = SpawnOptions::new().memory(role.memory());
    self.spawn.spawn_creep_with_options(&body, name, &opts)
  }

  /// Spawn creeps as necessary
  pub fn spawn_as_needed(&self) -> ReturnCode {
    // Find determine if miners are needed and if we can afford them.
    let (miner_body, _) = Role::miner().body();
    let miner_cost = Self::body_cost(&miner_body);

    let miners = Creeper::names_for_role(Role::miner());
    let harvesters = Creeper::names_for_role(Role::harvester());
    let builders = Creeper::names_for_role(Role::builder());
    let upgraders = Creeper::names_for_role(Role::upgrader());
    let repairers = Creeper::names_for_role(Role::repairer());
    let wall_repairers = Creeper::names_for_role(Role::wall_repairer());
    let lorries = Creeper::names_for_role(Role::lorry());
    let specialists = Creeper::names_for_role(Role::specialist());

    // For debugging purposes output the creeps by name
    info!(
      "{} of {} Miners: {:?}",
      miners.len(),
      self.get_min(&Role::miner()),
      &miners
    );
    info!(
      "{} of {} Harvesters: {:?}",
      miners.len(),
      self.get_min(&Role::harvester()),
      &harvesters
    );
    info!(
      "{} of {} Builders: {:?}",
      miners.len(),
      self.get_min(&Role::builder()),
      &builders
    );
    info!(
      "{} of {} Upgraders: {:?}",
      miners.len(),
      self.get_min(&Role::upgrader()),
      &upgraders
    );
    info!(
      "{} of {} Repairers: {:?}",
      miners.len(),
      self.get_min(&Role::repairer()),
      &repairers
    );
    info!(
      "{} of {} Wall Repairers: {:?}",
      miners.len(),
      self.get_min(&Role::wall_repairer()),
      &wall_repairers
    );
    info!(
      "{} of {} Lorries: {:?}",
      miners.len(),
      self.get_min(&Role::lorry()),
      &lorries
    );
    info!(
      "{} of {} Specialists: {:?}",
      miners.len(),
      self.get_min(&Role::specialist()),
      &specialists
    );

    // spawn miners if possible
    if self.room.energy_available() >= miner_cost {
      self.set_min(Role::harvester(), 0);
      self.set_min(Role::lorry(), 1);
      // building miners
      debug!("Building miners");
      // check each source and assign a miner to each one.
      let sources: Vec<Source> = self
        .room
        .find(find::SOURCES)
        .into_iter()
        .filter(|s| !s.has_miner())
        .collect();

      if !sources.is_empty() {
        let miner = Role::build_miner(sources[0].clone());
        return self.spawn(miner);
      }
    }

    // spawn harvesters if necessary
    let role = Role::harvester();
    if harvesters.len() < self.get_min(&role) {
      let result = self.spawn(role.clone());
      if result != ReturnCode::Ok && harvesters.is_empty() {
        if miners.is_empty() {
          return self.emergency_spawn(role);
        } else if lorries.is_empty() {
          return self.emergency_spawn(Role::lorry());
        }
      } else {
        return result;
      }
    }

    // spawn upgraders if necessary
    let role = Role::upgrader();
    if upgraders.len() < self.get_min(&role) {
      return self.spawn(role);
    }

    // spawn builder if necessary
    let role = Role::builder();
    if builders.len() < self.get_min(&role) {
      return self.spawn(role);
    }

    // spawn repairer if necessary
    let role = Role::repairer();
    if repairers.len() < self.get_min(&role) {
      return self.spawn(role);
    }

    // spawn wall_repairer if necessary
    let role = Role::wall_repairer();
    if wall_repairers.len() < self.get_min(&role) {
      return self.spawn(role);
    }

    // spawn lorry if necessary
    let role = Role::lorry();
    if lorries.len() < self.get_min(&role) {
      return self.spawn(role);
    }

    ReturnCode::Full
  }
}

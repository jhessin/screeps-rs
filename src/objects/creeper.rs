//! Wraps up a Creep and gives it superpowers!
use screeps::ResourceType::Energy;

use crate::*;

const ROLE_KEY: &str = "role";

/// Wraps a Creep in superpowers
pub struct Creeper {
  /// The creep that this creeper controls
  pub creep: Creep,
  /// The role assigned this creeper.
  pub role: Role,
}

impl Display for Creeper {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    write!(
      f,
      "Creep: {}\nRole: {}\nworking: {}",
      self.creep.name(),
      self.role,
      self.creep.memory().bool("working")
    )
  }
}

impl Creeper {
  /// Creates a new creeper given a creep.
  pub fn new(creep: Creep) -> Self {
    // Get the role
    let role = if let Ok(Some(role)) = creep.memory().string(ROLE_KEY) {
      if let Ok(role) = from_str::<Role>(&role) {
        role
      } else {
        Role::Upgrader(RoleData::default())
      }
    } else {
      Role::Upgrader(RoleData::default())
    };

    Creeper { creep, role }
  }

  /// Get the creeps appropriate data
  pub fn data(&mut self) -> &mut RoleData {
    match &mut self.role {
      Role::Harvester(d) => d,
      Role::Miner(d) => d,
      Role::Upgrader(d) => d,
      Role::Builder(d) => d,
      Role::Repairer(d) => d,
      Role::WallRepairer(d) => d,
      Role::Lorry(d) => d,
      Role::Specialist(d) => d,
    }
  }

  /// saves the updated creep data to memory
  fn save(&mut self) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let role = to_string(&self.role)?;
    self.creep.memory().set(ROLE_KEY, role);

    Ok(())
  }

  /// Get a list of the Creep names of creeps with a particular role.
  pub fn names_for_role(role: Role) -> Vec<String> {
    game::creeps::values()
      .into_iter()
      .filter_map(|creep| {
        let creep = Creeper::new(creep);
        if creep.role == role {
          Some(creep.creep.name())
        } else {
          None
        }
      })
      .collect()
  }

  /// Is this creep working
  pub fn working(&mut self) -> bool {
    const WORKING: &str = "working";
    if self.role == Role::miner() {
      self.creep.memory().set(WORKING, false);
      return false;
    }

    let working = self.creep.memory().bool(WORKING);

    if working && self.creep.store_used_capacity(Some(Energy)) == 0 {
      self.creep.memory().set(WORKING, false);
      self.data().reset_source();
      false
    } else if !working && self.creep.store_free_capacity(Some(Energy)) == 0 {
      self.creep.memory().set(WORKING, true);
      self.data().reset_target();
      true
    } else {
      working
    }
  }

  /// Runs the creep role
  pub fn run(&mut self) -> ReturnCode {
    let working = self.working();

    let code = match self.role {
      Role::Harvester(_) => {
        if working {
          self.deliver_energy()
        } else {
          self.harvest_energy()
        }
      }
      Role::Miner(_) => self.mine(),
      Role::Upgrader(_) => {
        if working {
          self.upgrade_controller()
        } else {
          self.gather_energy()
        }
      }
      Role::Builder(_) => {
        if working {
          self.build_nearest()
        } else {
          self.gather_energy()
        }
      }
      Role::Repairer(_) => {
        if working {
          self.repair_nearest()
        } else {
          self.gather_energy()
        }
      }
      Role::WallRepairer(_) => {
        if working {
          self.repair_wall()
        } else {
          self.gather_energy()
        }
      }
      Role::Lorry(_) => {
        if working {
          self.deliver_energy()
        } else {
          self.gather_energy()
        }
      }
      Role::Specialist(_) => {
        if working {
          self.withdraw()
        } else {
          self.transfer()
        }
      }
    };

    match self.save() {
      Ok(_) => {
        debug!("Creep: {} role and data saved", self.creep.name());
      }
      Err(e) => {
        error!(
          "Creep: {} info not saved to memory: {:?}",
          self.creep.name(),
          e
        );
      }
    }

    code
  }

  /// A utility to handle traveling to a resource/target
  fn handle_code(&mut self, code: ReturnCode, msg: &'static str) -> ReturnCode {
    let target =
      if self.working() { self.data().target() } else { self.data().source() };
    let target = match target {
      Some(Target::Resource(target)) => target.pos(),
      Some(Target::Source(target)) => target.pos(),
      Some(Target::Ruin(target)) => target.pos(),
      Some(Target::Structure(target)) => target.pos(),
      Some(Target::Tombstone(target)) => target.pos(),
      Some(Target::Creep(target)) => target.pos(),
      Some(Target::ConstructionSite(target)) => target.pos(),
      None => return ReturnCode::InvalidTarget,
    };

    if code == ReturnCode::NotInRange {
      self.creep.move_to(&target);
      code
    } else if code != ReturnCode::Ok {
      error!(
        "{} is having trouble with {}: code: {:?}",
        self.creep.name(),
        msg,
        code
      );
      code
    } else {
      code
    }
  }

  /// This is for the HARVESTER ONLY - it gathers energy directly from the source.
  pub fn harvest_energy(&mut self) -> ReturnCode {
    // find the nearest source if there isn't one already
    if let Some(Target::Source(_)) = self.data().source() {
    } else {
      let sources = self.creep.room().find(find::SOURCES_ACTIVE);
      let sources: Vec<&Source> = sources.iter().collect();
      let finder = Finder::new(self.creep.clone());
      if let Some(source) = finder.find_nearest_of(sources) {
        self.data().set_source(Target::Source(source.clone()));
      } else {
        return ReturnCode::NotFound;
      }
    };
    // call mine on the source
    self.mine()
  }

  /// This gathers any loose energy it can find
  /// Every creep will use this except miner, or specialist
  pub fn gather_energy(&mut self) -> ReturnCode {
    // TODO Note there was an error here.
    // prioritize targets
    let mut object_destroyed = false;
    for event in self.creep.room().get_event_log() {
      if let EventType::ObjectDestroyed(_event) = event.event {
        debug!("ObjectDestroyed event: {}", _event.object_type);
        object_destroyed = true;
        break;
      }
    }
    // check for existing target
    // Verify validity
    if !object_destroyed {
      if let Some(source) = self.data().source() {
        match source {
          Target::Source(s) => {
            if s.energy() > 0 {
              return self.mine();
            }
          }
          Target::Structure(s) => {
            if let Some(s) = s.as_has_store() {
              if s.store_used_capacity(Some(Energy)) > 0 {
                return self.withdraw();
              }
            }
          }
          Target::Tombstone(s) => {
            if s.store_used_capacity(Some(Energy)) > 0 {
              return self.withdraw();
            }
          }
          Target::Ruin(s) => {
            if s.store_used_capacity(Some(Energy)) > 0 {
              return self.withdraw();
            }
          }
          Target::Resource(s) => {
            if s.amount() > 0 {
              return self.pickup();
            }
          }
          Target::Creep(c) => {
            let mut creeper = Creeper::new(c);
            if creeper.creep.store_used_capacity(Some(Energy)) > 0
              && creeper.working()
            {
              match creeper.role {
                Role::Lorry(_) => {
                  creeper.data().set_target(Target::Creep(self.creep.clone()));
                  return ReturnCode::Ok;
                }
                Role::Harvester(_) => {
                  creeper.data().set_target(Target::Creep(self.creep.clone()));
                  return ReturnCode::Ok;
                }
                _ => (),
              }
            }
          }
          _ => (),
        };
      }
    }

    // Dropped Resources first
    let path = Finder::new(self.creep.clone());
    let sources = self.creep.room().find(find::DROPPED_RESOURCES);
    if !sources.is_empty() {
      if let Some(source) =
        path.find_nearest_of::<Resource>(sources.iter().collect())
      {
        debug!("Resource found! Setting source.");
        self.data().set_source(Target::Resource(source.clone()));
        debug!("Calling self.pickup()");
        return self.pickup();
      }
    }

    // Tombstones next
    let targets = self.creep.room().find(find::TOMBSTONES);
    let targets: Vec<&Tombstone> = targets
      .iter()
      .filter(|t| t.store_used_capacity(Some(Energy)) > 0)
      .collect();
    if !targets.is_empty() {
      if let Some(target) = path.find_nearest_of(targets) {
        self.data().set_source(Target::Tombstone(target.clone()));
        return self.withdraw();
      }
    }

    // RUINS
    let targets = self.creep.room().find(find::RUINS);
    let targets: Vec<&Ruin> = targets
      .iter()
      .filter(|r| r.store_used_capacity(Some(Energy)) > 0)
      .collect();
    if !targets.is_empty() {
      if let Some(target) = path.find_nearest_of(targets) {
        self.data().set_source(Target::Ruin(target.clone()));
        return self.withdraw();
      }
    }

    // Everything else
    let targets = self.creep.room().find(find::STRUCTURES);
    let targets: Vec<&Structure> = targets
      .iter()
      .filter(|s| {
        match s {
          Structure::Tower(_) => return false,
          Structure::Extension(_) => return false,
          Structure::Spawn(_) => return false,
          Structure::PowerSpawn(_) => return false,
          _ => (),
        }
        if let Some(s) = s.as_has_store() {
          if s.store_used_capacity(Some(Energy)) > 0 {
            return true;
          }
        }
        false
      })
      .collect();
    if !targets.is_empty() {
      if let Some(target) = path.find_nearest_of(targets) {
        self.data().set_source(Target::Structure(target.clone()));
        return self.withdraw();
      }
    }

    self.harvest_energy()
  }

  /// This will deliver the energy to the needed spots
  /// fallback -> upgrade_controller
  pub fn deliver_energy(&mut self) -> ReturnCode {
    // prioritize targets
    // Check for existing target
    // and verify that it is valid.
    if let Some(target) = self.data().target() {
      match target {
        Target::Structure(s) => {
          if let Some(s) = s.as_has_store() {
            if s.store_free_capacity(Some(Energy)) > 0 {
              return self.transfer();
            }
          }
        }
        Target::Creep(c) => {
          if c.store_free_capacity(Some(Energy)) > 0 {
            return self.transfer();
          }
        }
        // Everything else is invalid so continue and try to find another target.
        _ => (),
      };
    }

    let path = Finder::new(self.creep.clone());

    // towers
    let targets = self.creep.room().find(find::STRUCTURES);
    let targets: Vec<&StructureTower> = targets
      .iter()
      .filter_map(|s| {
        if let Structure::Tower(t) = s {
          if t.store_free_capacity(Some(Energy)) > 0 {
            info!(
              "Found tower with {} of {} energy",
              t.store_used_capacity(Some(Energy)),
              t.store_capacity(Some(Energy))
            );
            return Some(t);
          }
        }
        None
      })
      .collect();
    if !targets.is_empty() {
      if let Some(target) = path.find_nearest_of(targets) {
        self
          .data()
          .set_target(Target::Structure(Structure::Tower(target.clone())));
        return self.transfer();
      }
    }

    // extensions, spawns
    let targets = self.creep.room().find(find::STRUCTURES);
    let targets: Vec<&Structure> = targets
      .iter()
      .filter(|s| match s {
        Structure::Extension(s) => s.store_free_capacity(Some(Energy)) > 0,
        Structure::Spawn(s) => s.store_free_capacity(Some(Energy)) > 0,
        _ => false,
      })
      .collect();
    if !targets.is_empty() {
      if let Some(target) = path.find_nearest_of(targets) {
        self.data().set_target(Target::Structure(target.clone()));
        return self.transfer();
      }
    }

    // Everything else
    let targets = self.creep.room().find(find::STRUCTURES);
    let targets: Vec<&Structure> = targets
      .iter()
      .filter(|s| {
        if let Some(s) = s.as_has_store() {
          if s.store_free_capacity(Some(Energy)) > 0 {
            return true;
          }
        }
        false
      })
      .collect();

    if let Some(target) = path.find_nearest_of(targets) {
      self.data().set_target(Target::Structure(target.clone()));
      return self.transfer();
    }

    // if all else fails just upgrade the controller.
    self.upgrade_controller()
  }

  /// This will find and repair the nearest damaged structure
  /// excluding walls
  /// fallback -> build_nearest
  pub fn repair_nearest(&mut self) -> ReturnCode {
    // Check for existing target
    if let Some(Target::Structure(s)) = self.data().target() {
      // ensure it is still valid
      if let Some(att) = s.as_attackable() {
        if att.hits() < att.hits_max() {
          return self.repair();
        }
      }
    }

    // find the nearest damaged structure
    let targets = self.creep.room().find(find::STRUCTURES);
    let targets: Vec<&Structure> = targets
      .iter()
      .filter(|s| {
        // exclude walls
        if let Structure::Wall(_) = s {
          false
        } else if let Some(att) = s.as_attackable() {
          att.hits() < att.hits_max()
        } else {
          false
        }
      })
      .collect();
    // find the nearest
    let finder = Finder::new(self.creep.clone());
    if let Some(target) = finder.find_nearest_of(targets) {
      // call self.data().set_target()
      self.data().set_target(Target::Structure(target.clone()));
      return self.repair();
    }

    self.build_nearest()
  }

  /// This repairs the nearest wall
  /// fallback -> repair_nearest
  pub fn repair_wall(&mut self) -> ReturnCode {
    const STARTING_RATIO: f64 = 0.0001;
    // Check for valid ratio
    let ratio = if let Some(ratio) = self.data().ratio {
      if ratio < 1.0 {
        ratio
      } else {
        self.data().ratio = Some(STARTING_RATIO);
        STARTING_RATIO
      }
    } else {
      self.data().ratio = Some(STARTING_RATIO);
      STARTING_RATIO
    };

    // Check for existing target
    if let Some(Target::Structure(Structure::Wall(s))) = self.data().target() {
      if s.hits() == 0 {}
      let hits = s.hits() as f64;
      let max = s.hits_max() as f64;
      // Check it is still below the threshold ratio.
      if s.hits() != 0 && hits / max < ratio {
        // valid pass on to repair task
        return self.repair();
      }
    }

    // otherwise reset ratio and loop for new target.
    let mut ratio = STARTING_RATIO;
    self.data().ratio = Some(ratio);

    // First just get all the walls
    let targets = self.creep.room().find(find::STRUCTURES);
    let targets: Vec<StructureWall> = targets
      .into_iter()
      .filter_map(|s| {
        if let Structure::Wall(wall) = s {
          if wall.hits() != 0 {
            return Some(wall);
          }
        }
        None
      })
      .collect();

    // If there are no walls save time
    if targets.is_empty() {
      return ReturnCode::NotFound;
    }

    let targets = loop {
      let targets: Vec<_> = targets
        .iter()
        .filter(|wall| {
          let hits = wall.hits() as f64;
          let max = wall.hits_max() as f64;
          // Check it is still below the threshold ratio.
          hits / max < ratio
        })
        .collect();

      if targets.is_empty() {
        ratio += 0.0001;
        if (ratio - 1.0).abs() < 0.00001 {
          return ReturnCode::Full;
        }
      } else {
        break targets;
      }
    };

    // Find the nearest
    let finder = Finder::new(self.creep.clone());
    if let Some(target) = finder.find_nearest_of(targets) {
      // Save the target.
      self
        .data()
        .set_target(Target::Structure(Structure::Wall(target.clone())));
      // Save the ratio.
      self.data().ratio = Some(ratio);
      // call self.repair()
      return self.repair();
    }

    self.repair_nearest()
  }

  /// This builds the nearest construction site
  /// fallback -> Upgrade_controller
  pub fn build_nearest(&mut self) -> ReturnCode {
    // check for existing target
    if let Some(Target::ConstructionSite(_)) = self.data().target() {
      // any construction site is valid
      return self.build();
    }

    // otherwise find the nearest
    let targets = self.creep.room().find(find::CONSTRUCTION_SITES);
    let targets: Vec<&ConstructionSite> = targets.iter().collect();
    let finder = Finder::new(self.creep.clone());
    if let Some(target) = finder.find_nearest_of(targets) {
      // set it as target
      self.data().set_target(Target::ConstructionSite(target.clone()));
      return self.build();
    }

    self.upgrade_controller()
  }

  /// This picks up dropped resources
  pub fn pickup(&mut self) -> ReturnCode {
    debug!("Inside self.pickup()");
    if let Some(Target::Resource(r)) = self.data().source() {
      debug!("Attempting to pickup resource");
      self.handle_code(self.creep.pickup(&r), "Picking up resource")
    } else {
      debug!("Trying to pickup something that isn't a Resource");
      ReturnCode::InvalidTarget
    }
  }

  /// This gathers energy from the source assigned to
  /// data.source()
  pub fn mine(&mut self) -> ReturnCode {
    // Check for container mining
    if let Some(Target::Structure(Structure::Container(container))) =
      self.data().target()
    {
      if self.creep.pos() == container.pos() {
        if let Some(Target::Source(src)) = self.data().source() {
          return self.creep.harvest(&src);
        }
      } else {
        return self.creep.move_to(&container);
      }
    }

    if let Some(Target::Source(s)) = self.data().source() {
      self.handle_code(self.creep.harvest(&s), "Harvesting source")
    } else {
      ReturnCode::InvalidTarget
    }
  }

  /// This will withdraw energy from the source provided
  /// in data().source()
  pub fn withdraw(&mut self) -> ReturnCode {
    match self.data().source() {
      Some(s) => match s {
        Target::Source(s) => {
          self.handle_code(self.creep.harvest(&s), "Harvesting source")
        }
        Target::Structure(s) => {
          if let Some(w) = s.as_withdrawable() {
            self.handle_code(
              self.creep.withdraw_all(w, Energy),
              "Withdrawing Energy",
            )
          } else {
            ReturnCode::InvalidTarget
          }
        }
        Target::Tombstone(s) => self.handle_code(
          self.creep.withdraw_all(&s, Energy),
          "Withdrawing from Tombstone",
        ),
        Target::Ruin(s) => self.handle_code(
          self.creep.withdraw_all(&s, Energy),
          "Withdrawing from Ruin",
        ),
        Target::Resource(s) => {
          self.handle_code(self.creep.pickup(&s), "Picking up resource")
        }
        Target::Creep(_) => ReturnCode::Busy,
        _ => ReturnCode::InvalidTarget,
      },
      None => ReturnCode::InvalidTarget,
    }
  }

  /// This will transfer energy to the target structure
  /// @ data.target()
  pub fn transfer(&mut self) -> ReturnCode {
    match self.data().target() {
      Some(Target::Structure(structure)) => {
        if let Some(s) = structure.as_transferable() {
          return self.handle_code(
            self.creep.transfer_all(s, Energy),
            "Transferring Energy",
          );
        }
      }
      Some(Target::Creep(c)) => {
        return self.handle_code(
          self.creep.transfer_all(&c, Energy),
          "Transferring to Creep",
        )
      }
      _ => return ReturnCode::InvalidTarget,
    }

    ReturnCode::InvalidTarget
  }

  /// This will repair the structure referred to by
  /// data.target()
  pub fn repair(&mut self) -> ReturnCode {
    match self.data().target() {
      Some(Target::Structure(s)) => {
        self.handle_code(self.creep.repair(&s), "Repairing Structure")
      }
      _ => ReturnCode::InvalidTarget,
    }
  }

  /// This will build the construction site
  /// stored as data.construction_target()
  pub fn build(&mut self) -> ReturnCode {
    match self.data().target() {
      Some(Target::ConstructionSite(s)) => {
        self.handle_code(self.creep.build(&s), "Building Construction Site")
      }
      _ => ReturnCode::InvalidTarget,
    }
  }

  /// This will upgrade the controller
  pub fn upgrade_controller(&mut self) -> ReturnCode {
    let target = self.creep.room().controller().unwrap();
    self
      .data()
      .set_target(Target::Structure(Structure::Controller(target.clone())));
    self.handle_code(
      self.creep.upgrade_controller(&target),
      "Upgrading controller",
    )
  }
}

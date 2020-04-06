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
  /// A finder for finding targets
  pub finder: Finder,
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

/// Builder Methods
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

    let finder = Finder::new(creep.clone());

    Creeper { creep, role, finder }
  }
}

/// Utility Methods
impl Creeper {
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
  /// This also updates the working state of our creep.
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
          self.deliver()
        } else {
          self.harvest()
        }
      }
      Role::Miner(_) => self.harvest(),
      Role::Upgrader(_) => {
        if working {
          self.upgrade()
        } else {
          self.gather_work()
        }
      }
      Role::Builder(_) => {
        if working {
          self.build()
        } else {
          self.gather_work()
        }
      }
      Role::Repairer(_) => {
        if working {
          self.repair()
        } else {
          self.gather_work()
        }
      }
      Role::WallRepairer(_) => {
        if working {
          self.repair_wall()
        } else {
          self.gather_work()
        }
      }
      Role::Lorry(_) => {
        if working {
          self.deliver()
        } else {
          self.gather_storage()
        }
      }
      Role::Specialist(_) => {
        if working {
          self.deliver()
        } else {
          self.gather_work()
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
  fn move_to_or(&mut self, code: ReturnCode, msg: &'static str) -> ReturnCode {
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
}

/// Gather Methods
impl Creeper {
  /// Harvest - used by miners and harvesters
  pub fn harvest(&mut self) -> ReturnCode {
    self.data().validate_gather_source();

    let source = if let Some(t) = self.data().source() {
      t
    } else if let Some(t) = self.finder.find_nearest_active_source() {
      self.data().set_source(&t);
      t
    } else {
      return ReturnCode::NotFound;
    };

    if self.role == Role::miner() {
      return if let Target::Source(s) = source {
        let target = if let Some(Target::Structure(Structure::Container(t))) =
          self.data().target()
        {
          t
        } else if let Some(t) = s.container() {
          self
            .data()
            .set_target(&Target::Structure(Structure::Container(t.clone())));
          t
        } else {
          // There is no container just mine the source
          return self.move_to_or(self.creep.harvest(&s), "Mining Source");
        };
        // There is a target container
        if self.creep.pos() == target.pos() {
          self.creep.harvest(&s)
        } else {
          self.creep.move_to(&target)
        }
      } else {
        // invalid target retry
        self.data().reset_source();
        ReturnCode::InvalidTarget
      };
    }

    // You are a harvester - just get energy.
    match source {
      Target::Source(s) => {
        self.move_to_or(self.creep.harvest(&s), "Harvesting source")
      }

      Target::Structure(s) => {
        if let Some(w) = s.as_has_store() {
          if w.store_used_capacity(Some(Energy)) > 0 {
            return if let Some(s) = s.as_withdrawable() {
              self.move_to_or(
                self.creep.withdraw_all(s, Energy),
                "Withdrawing energy",
              )
            } else {
              self.data().reset_source();
              ReturnCode::InvalidTarget
            };
          }
        }
        if let Some(s) = s.as_withdrawable() {
          self.move_to_or(
            self.creep.withdraw_all(s, Energy),
            "Withdrawing energy",
          )
        } else {
          self.data().reset_source();
          ReturnCode::InvalidTarget
        }
      }

      Target::Tombstone(s) => self.move_to_or(
        self.creep.withdraw_all(&s, Energy),
        "Withdrawing from tombstone",
      ),

      Target::Ruin(s) => self
        .move_to_or(self.creep.withdraw_all(&s, Energy), "Withdraw from ruin"),

      Target::Resource(s) => {
        self.move_to_or(self.creep.pickup(&s), "Picking up resource")
      }

      Target::ConstructionSite(_) => {
        self.data().reset_source();
        ReturnCode::InvalidTarget
      }

      Target::Creep(s) => {
        let mut creep = Creeper::new(s);
        if creep.working()
          && (creep.role == Role::harvester() || creep.role == Role::lorry())
        {
          // Only do this if the creep is a lorry or a harvester
          creep.data().set_target(&Target::Creep(self.creep.clone()));
          self.creep.move_to(&creep.creep)
        } else {
          // invalid target try again.
          self.data().reset_source();
          ReturnCode::InvalidTarget
        }
      }
    }
  }

  /// gather_storage - gather energy for storage (used by lorries)
  pub fn gather_storage(&mut self) -> ReturnCode {
    self.data().validate_gather_source();

    let target = if let Some(t) = self.data().source() {
      t
    } else if let Some(t) = self.finder.find_nearest_energy_to_store() {
      self.data().set_source(&t);
      t
    } else {
      return ReturnCode::NotFound;
    };

    match target {
      Target::Source(s) => {
        warn!("Lorry gathering from source?");
        self.move_to_or(self.creep.harvest(&s), "Harvesting from source")
      }

      Target::Structure(s) => {
        if let Some(w) = s.as_has_store() {
          if w.store_used_capacity(Some(Energy)) > 0 {
            return if let Some(s) = s.as_withdrawable() {
              self.move_to_or(
                self.creep.withdraw_all(s, Energy),
                "Withdrawing energy",
              )
            } else {
              self.data().reset_source();
              ReturnCode::InvalidTarget
            };
          }
        }
        if let Some(s) = s.as_withdrawable() {
          self.move_to_or(
            self.creep.withdraw_all(s, Energy),
            "Withdrawing energy",
          )
        } else {
          self.data().reset_source();
          ReturnCode::InvalidTarget
        }
      }

      Target::Tombstone(t) => self.move_to_or(
        self.creep.withdraw_all(&t, Energy),
        "Withdrawing from tombstone",
      ),
      Target::Ruin(r) => self.move_to_or(
        self.creep.withdraw_all(&r, Energy),
        "Withdrawing from ruin",
      ),
      Target::Resource(r) => {
        self.move_to_or(self.creep.pickup(&r), "Picking up resource")
      }
      Target::ConstructionSite(_) => {
        // invalid target try again.
        warn!("Invalid target");
        self.data().reset_source();
        ReturnCode::InvalidTarget
      }
      Target::Creep(c) => {
        // only for working lorries or harvesters
        let mut creep = Creeper::new(c);
        if creep.working()
          && (creep.role == Role::harvester() || creep.role == Role::lorry())
        {
          creep.data().set_target(&Target::Creep(self.creep.clone()));
          self.creep.move_to(&creep.creep)
        } else {
          // invalid target try again.
          warn!("Invalid target");
          self.data().reset_source();
          ReturnCode::InvalidTarget
        }
      }
    }
  }

  /// gather_work() should gather resources for work
  pub fn gather_work(&mut self) -> ReturnCode {
    self.data().validate_gather_source();

    let target = if let Some(t) = self.data().source() {
      t
    } else if let Some(t) = self.finder.find_nearest_energy_for_work() {
      self.data().set_source(&t);
      t
    } else {
      return ReturnCode::NotFound;
    };

    match target {
      Target::Source(s) => {
        warn!("Worker harvesting from source");
        self.move_to_or(self.creep.harvest(&s), "Harvesting from source")
      }
      Target::Structure(s) => {
        match s {
          Structure::Container(s) => self.move_to_or(
            self.creep.withdraw_all(&s, Energy),
            "Withdrawing from container",
          ),
          Structure::Link(s) => self.move_to_or(
            self.creep.withdraw_all(&s, Energy),
            "Withdrawing from link",
          ),
          Structure::Storage(s) => self.move_to_or(
            self.creep.withdraw_all(&s, Energy),
            "Withdrawing from storage",
          ),
          _ => {
            // Invalid source try again
            warn!("Invalid source for worker");
            self.data().reset_source();
            ReturnCode::InvalidTarget
          }
        }
      }
      Target::Tombstone(s) => self.move_to_or(
        self.creep.withdraw_all(&s, Energy),
        "Withdrawing from Tombstone",
      ),
      Target::Ruin(s) => self.move_to_or(
        self.creep.withdraw_all(&s, Energy),
        "Withdrawing from ruin",
      ),
      Target::Resource(s) => {
        self.move_to_or(self.creep.pickup(&s), "Picking up resource")
      }
      Target::Creep(c) => {
        // only for working lorries
        let mut creep = Creeper::new(c);
        if creep.working()
          && (creep.role == Role::lorry() || creep.role == Role::harvester())
        {
          creep.data().set_target(&Target::Creep(self.creep.clone()));
          self.creep.move_to(&creep.creep)
        } else {
          // Invalid source try again
          self.data().reset_source();
          ReturnCode::InvalidTarget
        }
      }
      _ => {
        // Invalid target try again
        self.data().reset_source();
        ReturnCode::InvalidTarget
      }
    }
  }
}

/// Deliver methods
impl Creeper {
  /// Deliver - should deliver resources by priority for storage/use.
  pub fn deliver(&mut self) -> ReturnCode {
    self.data().validate_deliver_target();

    let target = if let Some(t) = self.data().target() {
      t
    } else if let Some(t) = self.finder.find_nearest_energy_target() {
      self.data().set_target(&t);
      t
    } else {
      // No target found
      return ReturnCode::NotFound;
    };

    match target {
      Target::Structure(s) => {
        if let Some(s) = s.as_transferable() {
          self.move_to_or(
            self.creep.transfer_all(s, Energy),
            "Transferring to structure",
          )
        } else {
          // Invalid target try again
          self.data().reset_target();
          ReturnCode::InvalidTarget
        }
      }
      Target::Creep(c) => {
        let mut creep = Creeper::new(c);
        if !creep.working() {
          self.move_to_or(
            self.creep.transfer_all(&creep.creep, Energy),
            "Transferring to creep",
          )
        } else {
          // Invalid target try again
          self.data().reset_target();
          ReturnCode::InvalidTarget
        }
      }
      _ => {
        // Invalid target try again
        self.data().reset_target();
        ReturnCode::InvalidTarget
      }
    }
  }

  /// Build - build the nearest construction site
  pub fn build(&mut self) -> ReturnCode {
    self.data().validate_build_target();

    let target = if let Some(t) = self.data().target() {
      t
    } else if let Some(t) = self.finder.find_nearest_construction_site() {
      self.data().set_target(&t);
      t
    } else {
      // Nothing to build - try repair
      return self.repair();
    };

    if let Target::ConstructionSite(s) = target {
      self.move_to_or(self.creep.build(&s), "Building site")
    } else {
      // invalid target try again
      self.data().reset_target();
      ReturnCode::InvalidTarget
    }
  }

  /// repair - repairs the nearest damaged structure
  pub fn repair(&mut self) -> ReturnCode {
    self.data().validate_repair_target();

    let target = if let Some(t) = self.data().target() {
      t
    } else if let Some(t) = self.finder.find_nearest_repair_target() {
      self.data().set_target(&t);
      t
    } else {
      // No target start upgrading
      return self.upgrade();
    };

    if let Target::Structure(s) = target {
      if let Some(t) = s.as_attackable() {
        if t.hits() < t.hits_max() {
          return self.move_to_or(self.creep.repair(&s), "Repairing structure");
        }
      }
    }

    // Invalid target try again
    self.data().reset_target();
    ReturnCode::InvalidTarget
  }

  /// repair_wall - repairs the nearest damaged structure
  pub fn repair_wall(&mut self) -> ReturnCode {
    self.data().validate_repair_target();

    let mut ratio = if let Some(r) = self.data().ratio {
      r
    } else {
      self.data().ratio = Some(0.0001);
      0.001
    };

    if !self.finder.has_repairable_walls() {
      // No walls to repair try
      return self.repair();
    }

    let target = if let Some(t) = self.data().target() {
      if let Target::Structure(Structure::Wall(w)) = &t {
        let hits = w.hits() as f64;
        let max = w.hits_max() as f64;
        if hits / max < ratio {
          Some(t)
        } else {
          None
        }
      } else {
        None
      }
    } else {
      None
    };

    let target = if let Some(t) = target {
      t
    } else {
      loop {
        if let Some(t) = self.finder.find_nearest_wall_repair_target(ratio) {
          self.data().set_target(&t);
          break t;
        } else {
          ratio += 0.001;
          if (ratio - 1.0).abs() < 0.0001 {
            return ReturnCode::NotFound;
          }
        }
      }
    };

    if let Target::Structure(s) = target {
      self.move_to_or(self.creep.repair(&s), "Repairing wall")
    } else {
      // Invalid target
      self.data().reset_target();
      ReturnCode::InvalidTarget
    }
  }

  /// Upgrade - upgrade the room controller
  pub fn upgrade(&mut self) -> ReturnCode {
    // if !self.working() {
    //   error!("Trying to work when you are empty?");
    //   return ReturnCode::NotEnough;
    // }

    // get the room controller
    let target = if let Some(t) = self.data().target() {
      t
    } else {
      let target = self.creep.room().controller().unwrap();
      Target::Structure(Structure::Controller(target))
    };

    if let Target::Structure(Structure::Controller(c)) = target {
      self.move_to_or(self.creep.upgrade_controller(&c), "Upgrading Controller")
    } else {
      // Invalid target
      self.data().reset_target();
      ReturnCode::InvalidTarget
    }
  }
}

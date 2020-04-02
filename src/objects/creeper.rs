//! Wraps up a Creep and gives it superpowers!
use crate::*;

const ROLE_KEY: &str = "role";
const DATA_KEY: &str = "data";

/// Wraps a Creep in superpowers
pub struct Creeper {
  /// The creep that this creeper controls
  creep: Creep,
  /// The role assigned this creeper.
  pub role: Role,
  /// The data assigned
  pub data: RoleData,
}

impl Creeper {
  /// Creates a new creeper given a creep.
  pub fn new(creep: Creep) -> Self {
    // Get the role
    let role = if let Ok(Some(role)) = creep.memory().string(ROLE_KEY) {
      if let Ok(role) = from_str::<Role>(&role) {
        role
      } else {
        Role::Upgrader
      }
    } else {
      Role::Upgrader
    };

    // Get the data if possible
    let unwrap_role = || match &role {
      Role::Miner(d) => d.clone(),
      Role::WallRepairer(d) => d.clone(),
      Role::Specialist(d) => d.clone(),
      _ => RoleData::default(),
    };

    let data = if let Ok(Some(d)) = creep.memory().string(DATA_KEY) {
      if let Ok(d) = from_str::<RoleData>(&d) {
        d
      } else {
        unwrap_role()
      }
    } else {
      unwrap_role()
    };
    Creeper { creep, role, data }
  }

  /// saves the updated creep data to memory
  fn save(&mut self) -> std::result::Result<(), Box<dyn std::error::Error>> {
    // save data to appropriate roles
    self.role = match self.role.clone() {
      Role::Miner(d) => Role::Miner(self.data.clone()),
      Role::WallRepairer(d) => Role::WallRepairer(self.data.clone()),
      Role::Specialist(d) => Role::Specialist(self.data.clone()),
      _ => self.role.clone(),
    };
    let role = to_string(&self.role)?;
    let data = to_string(&self.data)?;
    self.creep.memory().set(ROLE_KEY, role);
    self.creep.memory().set(DATA_KEY, data);

    Ok(())
  }

  /// Is this creep working
  pub fn is_working(&self) -> bool {
    const WORKING: &str = "working";
    let working = self.creep.memory().bool(WORKING);

    if working
      && self.creep.store_used_capacity(Some(ResourceType::Energy)) == 0
    {
      self.creep.memory().set(WORKING, false);
      false
    } else if !working
      && self.creep.store_free_capacity(Some(ResourceType::Energy)) == 0
    {
      self.creep.memory().set(WORKING, true);
      true
    } else {
      working
    }
  }

  /// Runs the creep role
  pub fn run(&mut self) -> ReturnCode {
    let working = self.is_working();

    let code = match self.role {
      Role::Harvester => {
        if working {
          self.deliver_energy()
        } else {
          self.harvest_energy()
        }
      }
      Role::Miner(_) => self.mine(),
      Role::Upgrader => {
        if working {
          self.upgrade_controller()
        } else {
          self.gather_energy()
        }
      }
      Role::Builder => {
        if working {
          self.build_nearest()
        } else {
          self.gather_energy()
        }
      }
      Role::Repairer => {
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
      Role::Lorry => {
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
  fn handle_code(&self, code: ReturnCode, msg: &'static str) -> ReturnCode {
    let target = if let Some(t) = self.data.target() {
      t
    } else {
      error!("Target not saved to creeper data");
      return ReturnCode::NotFound;
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
    let source = if let Some(s) = self.data.source() {
      s
    } else {
      let source = js! {
        let creep = @{self.creep.clone()};
        creep.findNearestByPath(FIND_SOURCES)
      };
      if let Some(source) = source.into_reference() {
        if let Some(source) = source.downcast::<Source>() {
          info!("Successfully harvesting from nearest source!");
          self.data.set_source(&source);
          source
        } else {
          return ReturnCode::NotFound;
        }
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
    // prioritize targets
    // Dropped Resources first
    let path = Finder::new(self.creep.clone());
    let targets = self.creep.room().find(find::DROPPED_RESOURCES);
    if !targets.is_empty() {
      if let Some(target) =
        path.find_nearest_of::<Resource>(targets.iter().collect())
      {
        self.data.set_target_resource(target);
        return self.handle_code(self.pickup(), "Picking up resource");
      }
    }

    // Tombstones next
    let targets = self.creep.room().find(find::TOMBSTONES);
    let targets: Vec<&Tombstone> = targets
      .iter()
      .filter(|t| t.store_used_capacity(Some(ResourceType::Energy)) > 0)
      .collect();
    if !targets.is_empty() {
      if let Some(target) = path.find_nearest_of(targets) {
        self.data.set_source_tombstone(target);
        return self.handle_code(self.withdraw(), "Drawing from Tombstone");
      }
    }

    // RUINS
    let targets = self.creep.room().find(find::RUINS);
    let targets: Vec<&Ruin> = targets
      .iter()
      .filter(|r| r.store_used_capacity(Some(ResourceType::Energy)) > 0)
      .collect();
    if !targets.is_empty() {
      if let Some(target) = path.find_nearest_of(targets) {
        self.data.set_source_ruin(target);
        return self.handle_code(self.withdraw(), "Withdrawing from Ruin");
      }
    }
    // TODO
    // Everything else
    unimplemented!()
  }

  /// This will deliver the energy to the needed spots
  pub fn deliver_energy(&self) -> ReturnCode {
    // TODO
    unimplemented!()
  }

  /// This will find and repair the nearest damaged structure
  /// excluding walls
  pub fn repair_nearest(&self) -> ReturnCode {
    // TODO
    unimplemented!()
  }

  /// This repairs the nearest wall
  pub fn repair_wall(&self) -> ReturnCode {
    // TODO
    unimplemented!()
  }

  /// This builds the nearest construction site
  pub fn build_nearest(&self) -> ReturnCode {
    // TODO
    unimplemented!()
  }

  /// This picks up dropped resources
  /// Uses data.resource_target() to find the resource
  pub fn pickup(&self) -> ReturnCode {
    // TODO
    unimplemented!()
  }

  /// This gathers energy from the source assigned to
  /// data.source()
  pub fn mine(&self) -> ReturnCode {
    // TODO
    unimplemented!()
  }

  /// This will withdraw energy from the source provided
  /// in data.source_structure()
  pub fn withdraw(&self) -> ReturnCode {
    // TODO
    unimplemented!()
  }

  /// This will transfer energy to the target structure
  /// @ data.target()
  pub fn transfer(&self) -> ReturnCode {
    // TODO
    unimplemented!()
  }

  /// This will repair the structure referred to by
  /// data.target()
  pub fn repair(&self) -> ReturnCode {
    // TODO
    unimplemented!()
  }

  /// This will build the construction site
  /// stored as data.construction_target()
  pub fn build(&self) -> ReturnCode {
    // TODO
    unimplemented!()
  }

  /// This will upgrade the controller
  pub fn upgrade_controller(&self) -> ReturnCode {
    // TODO
    unimplemented!()
  }
}

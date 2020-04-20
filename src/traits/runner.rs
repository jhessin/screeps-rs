use crate::*;

/// This trait allows things to run!
pub trait Runner {
  /// Run the game object so it does what it is supposed to.
  fn run(&self) -> ReturnCode;
}

impl Runner for Creep {
  fn run(&self) -> ReturnCode {
    let mut creep = Creeper::new(self.clone());
    // time_hack(format!("Running creep: {}", creep.creep.name()).as_str());
    // TODO run this fully here
    creep.run()
  }
}

impl Runner for Structure {
  fn run(&self) -> ReturnCode {
    match self {
      Structure::Container(_) => ReturnCode::Ok,
      Structure::Controller(_) => ReturnCode::Ok,
      Structure::Extension(_) => ReturnCode::Ok,
      Structure::Extractor(_) => ReturnCode::Ok,
      Structure::Factory(_) => ReturnCode::Ok,
      Structure::InvaderCore(_) => ReturnCode::Ok,
      Structure::KeeperLair(_) => ReturnCode::Ok,
      Structure::Lab(_) => ReturnCode::Ok,
      Structure::Link(_) => todo!("Run Link Here"),
      Structure::Nuker(_) => ReturnCode::Ok,
      Structure::Observer(_) => ReturnCode::Ok,
      Structure::PowerBank(_) => ReturnCode::Ok,
      Structure::PowerSpawn(_) => ReturnCode::Ok,
      Structure::Portal(_) => ReturnCode::Ok,
      Structure::Rampart(_) => ReturnCode::Ok,
      Structure::Road(_) => ReturnCode::Ok,
      Structure::Spawn(spawn) => {
        debug!("running spawn {}", spawn.name());
        let spawn = Spawner::new(spawn.clone());

        let r = spawn.spawn_as_needed();
        debug!("Spawn returned: {:?}", r);
        todo!("Run this locally here.")
      }
      Structure::Storage(_) => ReturnCode::Ok,
      Structure::Terminal(_) => todo!("Run Terminal here"),
      Structure::Tower(tower) => {
        if let Some(target) =
          tower.pos().find_closest_by_range(find::HOSTILE_CREEPS)
        {
          tower.attack(&target)
        } else {
          for range in 0..79 {
            for creep in
              tower.pos().find_in_range(find::MY_CREEPS, range) as Vec<Creep>
            {
              if creep.hits() < creep.hits_max() {
                return tower.heal(&creep);
              }
            }
          }
          ReturnCode::Ok
        }
      }
      Structure::Wall(_) => ReturnCode::Ok,
    }
  }
}

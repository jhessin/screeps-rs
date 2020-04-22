use crate::*;

/// This trait allows things to run!
pub trait Runner {
  /// Run the game object so it does what it is supposed to.
  fn run(&self) -> ReturnCode;
}

impl Runner for Creep {
  fn run(&self) -> ReturnCode {
    if self.spawning() {
      return ReturnCode::Busy;
    }

    // time_hack(format!("Running creep: {}", creep.creep.name()).as_str());
    if let Some(Values::Role(role)) = self.memory().get_value(Keys::Role) {
      return role.run(self);
    }

    // INVALID ROLE!
    ReturnCode::Tired
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
      Structure::Link(link) => {
        if link.store_free_capacity(None) > 0 {
          // Don't worry about links that aren't full
          return ReturnCode::Ok;
        }

        let mut others: Vec<StructureLink> = link
          .room()
          .find(find::STRUCTURES)
          .into_iter()
          .filter_map(|s| {
            if s.id().to_string() == link.id().to_string() {
              None
            } else {
              if let Structure::Link(l) = s {
                Some(l)
              } else {
                None
              }
            }
          })
          .collect();

        if others.len() == 0 {
          return ReturnCode::Ok;
        }
        let mut target = others.pop().unwrap();

        while !others.is_empty() {
          let next = others.pop().unwrap();
          if next.store_used_capacity(None) < target.store_used_capacity(None) {
            target = next;
          }
        }
        let amount = link.store_used_capacity(None) / 2;
        link.transfer_energy(&target, Some(amount))
      }
      Structure::Nuker(_) => ReturnCode::Ok,
      Structure::Observer(_) => ReturnCode::Ok,
      Structure::PowerBank(_) => ReturnCode::Ok,
      Structure::PowerSpawn(_) => ReturnCode::Ok,
      Structure::Portal(_) => ReturnCode::Ok,
      Structure::Rampart(_) => ReturnCode::Ok,
      Structure::Road(_) => ReturnCode::Ok,
      Structure::Spawn(spawn) => todo!("Run Spawner."),
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

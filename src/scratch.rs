  /// Determines if a source has a miner attached to it.
  /// TODO: Move this to another object possibly spawner?
pub fn has_miner(source: &Source) -> bool {
  for creep in game::creeps::values() {
    let creep = Creeper::new(creep);
    if creep.role == Role::miner() {
      if creep.data().source().id() == source.id() {
        return true;
      }
    }
  }
  false
}

/// This will deliver the energy to the needed spots
/// TODO Move to Creeper
fn deliver_energy(creep: &Creep) -> ReturnCode {
  // prioritize targets
  // towers
  let targets: Vec<StructureTower> = creep
    .room()
    .find(find::STRUCTURES)
    .into_iter()
    .filter_map(|s| {
      if let Structure::Tower(t) = s {
        if t.store_free_capacity(Some(ResourceType::Energy)) > 0 {
          return Some(t);
        }
      }
      None
    })
    .collect();
  let targets: Vec<&StructureTower> = targets.iter().collect();
  if !targets.is_empty() {
    if let Some(target) = _find_nearest(creep.pos(), targets) {
      let code = withdraw(creep, target);
      return handle_code(creep, code, target);
    }
  }
  // extensions, spawn
  let targets: Vec<Structure> = creep
    .room()
    .find(find::STRUCTURES)
    .into_iter()
    .filter(|s| {
      match s {
        Structure::Extension(s) => {
          if s.store_free_capacity(Some(ResourceType::Energy)) > 0 {
            return true;
          }
        }
        Structure::Spawn(s) => {
          if s.store_free_capacity(Some(ResourceType::Energy)) > 0 {
            return true;
          }
        }
        _ => {}
      }
      false
    })
    .collect();
  let targets: Vec<&Structure> = targets.iter().collect();
  if !targets.is_empty() {
    if let Some(target) = _find_nearest(creep.pos(), targets) {
      let code = withdraw(creep, target.as_withdrawable().unwrap());
      return handle_code(creep, code, target);
    }
  }
  // links, storage, etc. everything else
  let targets: Vec<Structure> = creep
    .room()
    .find(find::STRUCTURES)
    .into_iter()
    .filter(|s| {
      if let Some(s) = s.as_has_store() {
        if s.store_free_capacity(Some(ResourceType::Energy)) > 0 {
          return true;
        }
      }
      false
    })
    .collect();
  let targets: Vec<&Structure> = targets.iter().collect();
  if !targets.is_empty() {
    if let Some(target) = _find_nearest(creep.pos(), targets) {
      let code = withdraw(creep, target.as_withdrawable().unwrap());
      return handle_code(creep, code, target);
    }
  }
  ReturnCode::NotFound
}

// Runs the creep role
// TODO: Move this to Creeper
// pub fn run(&self, creep: &Creep) -> ReturnCode {
//   let working = Self::is_working(&creep);
//
//   match self {
//     Role::Harvester(_) => {
//       if working {
//         deliver_energy(creep)
//       } else {
//         harvest_energy(creep)
//       }
//     },
//     Role::Miner(data) => {
//       if let Some(source) = data.source() {
//         mine(creep, &source)
//       } else {
//         ReturnCode::NotFound
//       }
//     },
//     Role::Upgrader(_) => {
//       if working {
//         upgrade_controller(creep)
//       } else {
//         gather_energy(creep)
//       }
//     },
//     Role::Builder(_) => {
//       if working {
//         build_nearest(creep)
//       } else {
//         gather_energy(creep)
//       }
//     },
//     Role::Repairer(_) => {
//       if working {
//         repair_nearest(creep)
//       } else {
//         gather_energy(creep)
//       }
//     },
//     Role::WallRepairer(data) => {
//       let ratio = if let Some(ratio) = data.ratio {
//         ratio
//       } else {
//         // default minimum ratio
//         0.0001
//       };
//       if working {
//         repair_wall(creep, ratio)
//       } else {
//         gather_energy(creep)
//       }
//     },
//     Role::Lorry => {
//       if working {
//         deliver_energy(creep)
//       } else {
//         gather_energy(creep)
//       }
//     },
//     Role::Specialist (data) => {
//       let from = if let Some(from) = data.source_structure() {
//         from
//       } else {
//         panic!("Specialist: {} has no source structure to harvest from", creep.name())
//       };
//       let to = if let Some(target) = data.target() { target } else {
//         panic!("Specialist: {} has no destination structure", creep.name())
//       };
//       if working {
//         withdraw(creep, from.as_withdrawable().unwrap())
//       } else {
//         transfer(creep, to.as_transferable().unwrap())
//       }
//     },
//   }
// }

/// This will find and repair the nearest damaged structure
/// TODO Move to Creeper
fn repair_nearest(creep: &Creep) -> ReturnCode {
  // find the nearest damaged structure
  // exclude walls
  // call repair() on it.
  // TODO
  unimplemented!()
}

/// This repairs the nearest wall using the assigned ratio
/// TODO Move to Creeper
fn repair_wall(creep: &Creep, ratio: f64) -> ReturnCode {
  // use a time cycle to check for new walls (reset the ratio)
  // otherwise just search for walls within the current ratio
  // if none are found increase the ratio (check for 1.0 value)
  // find the nearest and call repair() on it.
  // TODO
  unimplemented!()
}

/// This builds the nearest construction site
/// TODO Move to Creeper
fn build_nearest(creep: &Creep) -> ReturnCode {
  // Just find the nearest construction site and call build() on it.
  // TODO
  unimplemented!()
}

/// This picks up dropped resources
/// TODO Move to Creeper
fn pickup(creep: &Creep, resource: &Resource) -> ReturnCode {
  let code = creep.pickup(resource);
  handle_code(creep, code, resource)
}

/// This gathers the energy from a given source
/// TODO Move to Creeper
fn mine(creep: &Creep, source: &Source) -> ReturnCode {
  let code = creep.harvest(source);
  handle_code(creep, code, source)
}

/// This will withdraw energy from a specific source
/// TODO Move to Creeper
fn withdraw<T>(creep: &Creep, target: &T) -> ReturnCode
  where
    T: RoomObjectProperties + Withdrawable + ?Sized,
{
  let code = creep.withdraw_all(target, ResourceType::Energy);
  handle_code(creep, code, target)
}

/// This will transfer energy to a target structure
/// TODO Move to Creeper
fn transfer<T>(creep: &Creep, target: &T) -> ReturnCode
  where
    T: RoomObjectProperties + Transferable + ?Sized,
{
  let code = creep.transfer_all(target, ResourceType::Energy);
  handle_code(creep, code, target)
}

/// This will repair a target structure
/// TODO Move to Creeper
fn repair<T>(creep: &Creep, target: &T) -> ReturnCode
  where
    T: StructureProperties,
{
  let code = creep.repair(target);
  handle_code(creep, code, target)
}

/// This will build a construction site
/// TODO Move to Creeper
fn build(creep: &Creep, target: &ConstructionSite) -> ReturnCode {
  let code = creep.build(target);
  handle_code(creep, code, target)
}

/// This will upgrade the controller
/// TODO Move to Creeper
fn upgrade_controller(creep: &Creep) -> ReturnCode {
  let controller = creep.room().controller().unwrap();
  let code = creep.upgrade_controller(&controller);
  handle_code(creep, code, &controller)
}

/// This is a utility that helps me find the nearest object in any array of StructureProperties
/// TODO Move to Finder trait on Vec<Target>
fn _find_nearest<T>(_: Position, _: Vec<&T>) -> Option<&T>
  where
    T: RoomObjectProperties + ?Sized,
{
  unimplemented!()
}

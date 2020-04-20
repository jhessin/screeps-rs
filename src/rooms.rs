use crate::*;

/// Manage the room
pub fn manage_room(room: Room) -> ReturnCode {
  info!("Running room: {}", room.name_local());

  // run creeps
  for creep in room.find(find::MY_CREEPS) {
    // TODO replace this with creep.run()
    manage_creep(creep);
  }

  // run spawns
  for spawn in room
    .find(find::STRUCTURES)
    .into_iter()
    .filter_map(|s| {
      if let Structure::Spawn(s) = s {
        return Some(s);
      }
      None
    })
    .collect::<Vec<StructureSpawn>>()
  {
    // TODO replace this with spawn.run()
    manage_spawn(spawn);
  }

  // run towers
  for tower in room
    .find(find::STRUCTURES)
    .into_iter()
    .filter_map(|s| if let Structure::Tower(s) = s { Some(s) } else { None })
    .collect::<Vec<StructureTower>>()
  {
    // TODO replace this with tower.run()
    if let Some(creep) =
      tower.pos().find_closest_by_range(find::HOSTILE_CREEPS) as Option<Creep>
    {
      tower.attack(&creep);
    }
  }

  // TODO run links

  // TODO run terminals
  ReturnCode::Ok
}

fn manage_spawn(spawn: StructureSpawn) {
  debug!("running spawn {}", spawn.name());
  let spawn = Spawner::new(spawn);

  let r = spawn.spawn_as_needed();
  debug!("Spawn returned: {:?}", r);
}

fn manage_creep(creep: Creep) {
  let mut creep = Creeper::new(creep);
  // time_hack(format!("Running creep: {}", creep.creep.name()).as_str());
  creep.run();
}

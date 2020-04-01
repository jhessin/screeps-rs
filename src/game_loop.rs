use crate::*;

/// This is the main game loop that runs the rest of the game.
/// Try to keep it slim and trim.
pub fn game_loop() {
  time_hack("loop starting!");

  debug!("running spawns");
  for spawn in screeps::game::spawns::values() {
    manage_spawn(spawn);
  }

  debug!("running creeps");
  for creep in screeps::game::creeps::values() {
    let name = creep.name();
    debug!("running creep {}", name);
    if creep.spawning() {
      continue;
    }
    manage_creep(creep);
  }

  let time = screeps::game::time();

  if time % 32 == 3 {
    info!("running memory cleanup");
    cleanup_memory()
      .expect("expected Memory.creeps format to be a regular memory object");
  }

  time_hack("Loop done!");
}

/// This is a quick and easy way to get a time-hack at any point in the program.
pub fn time_hack(msg: &str) {
  let _time = screeps::game::cpu::get_used();
  info!("{} CPU: {}", msg, _time);
}

fn manage_spawn(spawn: StructureSpawn) {
  debug!("running spawn {}", spawn.name());
  let body = [Part::Move, Part::Move, Part::Carry, Part::Work];

  if spawn.energy() >= body.iter().map(|p| p.cost()).sum() {
    // create a unique name, spawn.
    let name_base = screeps::game::time();
    let mut additional = 0;
    let res = loop {
      let name = format!("{}-{}", name_base, additional);
      let res = spawn.spawn_creep(&body, &name);

      if res == ReturnCode::NameExists {
        additional += 1;
      } else {
        break res;
      }
    };

    if res != ReturnCode::Ok {
      warn!("couldn't spawn: {:?}", res);
    }
  }
}

fn manage_creep(creep: Creep) {
  if creep.memory().bool("harvesting") {
    if creep.store_free_capacity(Some(ResourceType::Energy)) == 0 {
      creep.memory().set("harvesting", false);
    }
  } else if creep.store_used_capacity(None) == 0 {
    creep.memory().set("harvesting", true);
  }

  if creep.memory().bool("harvesting") {
    let source = &creep.room().find(find::SOURCES)[0];
    if creep.pos().is_near_to(source) {
      let r = creep.harvest(source);
      if r != ReturnCode::Ok {
        warn!("couldn't harvest: {:?}", r);
      }
    } else {
      creep.move_to(source);
    }
  } else if let Some(c) = creep.room().controller() {
    let r = creep.upgrade_controller(&c);
    if r == ReturnCode::NotInRange {
      creep.move_to(&c);
    } else if r != ReturnCode::Ok {
      warn!("couldn't upgrade: {:?}", r);
    }
  } else {
    warn!("creep room has no controller!");
  }
}

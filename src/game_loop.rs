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

  debug!("running towers");
  for tower in screeps::game::structures::values().into_iter().filter_map(|s| {
    if let Structure::Tower(t) = s {
      Some(t)
    } else {
      None
    }
  }) {
    if let Some(target) =
      tower.pos().find_closest_by_range(find::HOSTILE_CREEPS)
    {
      tower.attack(&target);
    }
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
  let spawn = Spawner::new(spawn);

  let r = spawn.spawn_as_needed();
  debug!("Spawn returned: {:?}", r);
}

fn manage_creep(creep: Creep) {
  let mut creep = Creeper::new(creep);

  creep.run();
}

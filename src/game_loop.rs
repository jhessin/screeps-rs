use crate::*;
use std::collections::BTreeMap;

/// This is the main game loop that runs the rest of the game.
/// Try to keep it slim and trim.
pub fn game_loop() {
  time_hack("==============loop starting!==============");

  for room in game::rooms::values() {
    dump_info(&room);
    manage_room(room);
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

/// This dumps some info that is useful to the console
pub fn dump_info(room: &Room) {
  time_hack("Starting info dump");
  info!("Room {}:", room.name());

  let mut creeps: BTreeMap<Option<Role>, Vec<Creep>> = BTreeMap::new();
  for creep in room.find(find::MY_CREEPS) {
    if let Some(Values::Role(role)) = creep.memory().get_value(Keys::Role) {
      let vec = creeps.entry(Some(role)).or_insert(vec![]);
      vec.push(creep);
    } else {
      let vec = creeps.entry(None).or_insert(vec![]);
      vec.push(creep);
    }
  }

  for (role, creeps) in creeps {
    let creeps = creeps.into_iter().map(|s| s.name()).collect::<Vec<String>>();
    match role {
      Some(role) => info!("{} Creeps: {:?}", role, creeps),
      None => info!("Creeps without a role: {:?}", creeps),
    }
  }

  // break down current energy
  let energy = room.energy_available();
  let capacity = room.energy_capacity_available();

  info!("{} of {} Energy available", energy, capacity);
  info!("{} is required for mining", Role::Miner.cost());
  time_hack("Finished info dump");
}

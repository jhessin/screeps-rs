use crate::*;

/// This is the main game loop that runs the rest of the game.
/// Try to keep it slim and trim.
pub fn game_loop() {
  time_hack("==============loop starting!==============");

  for room in game::rooms::values() {
    dump_info(&room);
    manage_room(room);
  }

  let time = screeps::game::time();

  for flag in game::flags::values() {
    flag.run();
  }

  if time % 32 == 3 {
    info!("running memory cleanup");
    cleanup_memory()
      .expect("expected Memory.creeps format to be a regular memory object");
  }

  time_hack("Loop done!");
}

/// This is a quick and easy way to get a time-hack at any point in the program.
pub fn time_hack(msg: &str) {
  let _time = game::cpu::get_used();
  info!("{} CPU: {}", msg, _time);
}

/// This dumps some info that is useful to the console
pub fn dump_info(room: &Room) {
  time_hack("Starting info dump");
  info!("Room {}:", room.name());

  let mut creeps: BTreeMap<Role, Vec<Creeper>> = BTreeMap::new();
  for creep in room.find(find::MY_CREEPS) {
    let creep = Creeper::new(creep);
    let role = creep.role();
    let vec = creeps.entry(role).or_insert(vec![]);
    vec.push(creep);
  }

  for (role, creeps) in creeps {
    let creeps = creeps.into_iter().map(|s| s.name()).collect::<Vec<String>>();
    info!("{} Creeps: {:?}", role, creeps);
  }

  // break down current energy
  let energy = room.energy_available();
  let capacity = room.energy_capacity_available();

  info!("{} of {} Energy available", energy, capacity);
  info!("{} is required for mining", Role::Miner.cost());
  time_hack("Finished info dump");
}

use crate::*;

/// Manage the room
pub fn manage_room(room: Room) -> ReturnCode {
  info!("Running room: {}", room.name_local());

  // run creeps
  for creep in room.find(find::MY_CREEPS) {
    let creep = Creeper::new(creep);
    creep.run();
  }

  // run structures
  for structure in room.find(find::STRUCTURES) {
    structure.run();
  }

  ReturnCode::Ok
}

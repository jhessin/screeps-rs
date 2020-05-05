use crate::*;

/// The Director is a master wrapper that holds all global data and manages all creeps,
/// rooms etc. It will get saved/loaded to/from memory each tick.
/// It will also Own everything!
#[derive(Serialize, Deserialize)]
pub struct Director {
  /// The player's username
  username: String,
  /// A collection of each cell that is owned by us
  owned_rooms: HashMap<RoomName, RoomData>,
  /// A collection of each cell that has been scouted
  scouted_rooms: HashMap<RoomName, RoomData>,
  task_queue: VecDeque<Task>,
}

const DIRECTOR_KEY: &str = "Director";

impl Display for Director {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    // TODO update this to show everything.
    writeln!(f, "Owned rooms: ")?;
    for (name, cell) in &self.owned_rooms {
      writeln!(f, "\t{}:{}", name, cell)?;
    }

    writeln!(f, "Scouted rooms: ")?;
    for (name, cell) in &self.scouted_rooms {
      writeln!(f, "\t{}:{}", name, cell)?;
    }
    Ok(())
  }
}

impl Default for Director {
  fn default() -> Self {
    // initialize from current data or memory
    if let Ok(Some(data)) = root().arr(DIRECTOR_KEY) {
      if let Ok(data) = deserialize(&data) {
        trace!("\nDirector successfully deserialize from memory: {}", &data);
        return data;
      }
    }
    // This should never panic as we always have at least 1 spawn in the game
    let username = game::spawns::values().get(0).unwrap().owner_name().unwrap();
    let mut owned_cells = HashMap::new();
    let mut scouted_cells = HashMap::new();
    for room in game::rooms::values() {
      if let Some(ctrl) = room.controller() as Option<StructureController> {
        if ctrl.my() {
          owned_cells.insert(room.name(), room.into());
          continue;
        }
      }
      scouted_cells.insert(room.name(), room.into());
    }

    let task_queue = VecDeque::new();

    Director {
      username,
      owned_rooms: owned_cells,
      scouted_rooms: scouted_cells,
      task_queue,
    }
  }
}

impl Director {
  /// Update the director
  pub fn update(&mut self) {
    for room in game::rooms::values() {
      if let Some(ctrl) = room.controller() as Option<StructureController> {
        if ctrl.my() {
          self.scouted_rooms.remove(&room.name());
          self.owned_rooms.insert(room.name(), room.into());
          continue;
        }
      }
      self.owned_rooms.remove(&room.name());
      self.scouted_rooms.insert(room.name(), room.into());
    }
  }

  /// Save the director
  pub fn save(&self) -> bool {
    if let Ok(data) = serialize(self) {
      root().set(DIRECTOR_KEY, data);
      true
    } else {
      false
    }
  }
}

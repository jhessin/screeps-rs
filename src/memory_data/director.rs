use crate::*;

/// The Director is a master wrapper that holds all global data and manages all creeps,
/// rooms etc. It will get saved/loaded to/from memory each tick.
/// It will also Own everything!
#[derive(Serialize, Deserialize)]
pub struct Director {
  /// The player's username
  username: String,
  /// A collection of each cell that is owned by us
  owned_cells: HashMap<RoomName, RoomData>,
  /// A collection of each cell that has been scouted
  scouted_cells: HashMap<RoomName, RoomData>,
  // TODO Add task_queue: VecQueue<Task>
}

const DIRECTOR_KEY: &str = "Director";

impl Display for Director {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    // TODO update this to show everything.
    writeln!(f, "Owned rooms: ")?;
    for (name, cell) in &self.owned_cells {
      writeln!(f, "\t{}:{}", name, cell)?;
    }

    writeln!(f, "Scouted rooms: ")?;
    for (name, cell) in &self.scouted_cells {
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
    let username = game::spawns::values().get(0).unwrap().owner_name().unwrap();
    let mut owned_cells = HashMap::new();
    let mut scouted_cells = HashMap::new();
    for room in game::rooms::values() {
      if let Some(ctrl) = room.controller() as Option<StructureController> {
        if ctrl.owner_name() == Some(username.clone()) {
          owned_cells.insert(room.name(), RoomData::new(room));
          continue;
        }
      }
      scouted_cells.insert(room.name(), RoomData::new(room));
    }
    Director { username, owned_cells, scouted_cells }
  }
}

impl Director {
  /// Update the director
  pub fn update(&mut self) {
    for room in game::rooms::values() {
      if let Some(ctrl) = room.controller() as Option<StructureController> {
        if ctrl.my() {
          self.scouted_cells.remove(&room.name());
          self.owned_cells.insert(room.name(), RoomData::new(room));
          continue;
        }
      }
      self.owned_cells.remove(&room.name());
      self.scouted_cells.insert(room.name(), RoomData::new(room));
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

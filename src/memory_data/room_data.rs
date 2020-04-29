use crate::*;
use std::collections::HashMap;

/// The AgentCell is a single room and manages all of the info for that cell
#[derive(Serialize, Deserialize, Eq, PartialEq)]
pub struct RoomData {
  name: RoomName,
  level: u8,
  construction: HashMap<StructureType, HashSet<Position>>,
  structures: HashMap<StructureType, HashSet<StructureData>>,
}

impl Display for RoomData {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    writeln!(f, "{} Level: {}", self.name, self.level)?;
    writeln!(f, "Construction Sites:")?;
    for (structure_type, sites) in &self.construction {
      writeln!(f, "\t{:?}:", structure_type)?;
      for site in sites {
        writeln!(f, "\t\t{}", site)?;
      }
    }
    writeln!(f, "Structures:")?;
    for (structure_type, sites) in &self.structures {
      writeln!(f, "\t{:?}:", structure_type)?;
      for site in sites {
        writeln!(f, "\t\t{}", site)?;
      }
    }

    Ok(())
  }
}

impl RoomData {
  /// Generate a new AgentCell from a room
  pub fn new(room: Room) -> Self {
    let name = room.name();
    let level =
      if let Some(ctrl) = room.controller() as Option<StructureController> {
        ctrl.level() as u8
      } else {
        0
      };

    let mut construction: HashMap<StructureType, HashSet<Position>> =
      HashMap::new();
    let mut structures: HashMap<StructureType, HashSet<StructureData>> =
      HashMap::new();

    for site in room.find(find::CONSTRUCTION_SITES) {
      let entry = construction.entry(site.structure_type()).or_default();
      entry.insert(site.pos());
    }

    for s in room.find(find::MY_STRUCTURES) {
      let entry = structures.entry(s.structure_type()).or_default();
      entry.insert(StructureData::new(s.as_structure()));
    }

    RoomData { name, level, construction, structures }
  }
}

use crate::*;

/// The AgentCell is a single room and manages all of the info for that cell
#[derive(Serialize, Deserialize, Eq, PartialEq)]
pub struct RoomData {
  name: RoomName,
  level: u8,
  construction: HashMap<StructureType, Vec<ConstructionData>>,
  structures: HashMap<StructureType, Vec<StructureData>>,
  sources: Vec<SourceData>,
  mineral: Option<MineralData>,
  deposit: Option<DepositData>,
  // TODO: add creeps: BTreeMap<Role, Vec<CreepData>>
}

impl Display for RoomData {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    // TODO: Update this to show everything
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

impl From<Room> for RoomData {
  fn from(room: Room) -> Self {
    // store the name of the room.
    let name = room.name();

    // determine the level of the room
    let level =
      if let Some(ctrl) = room.controller() as Option<StructureController> {
        ctrl.level() as u8
      } else {
        0
      };

    // initialize collection variables
    let mut construction: HashMap<StructureType, Vec<ConstructionData>> =
      HashMap::new();
    let mut structures: HashMap<StructureType, Vec<StructureData>> =
      HashMap::new();
    let mut sources = Vec::<SourceData>::new();

    // get the mineral data
    let mineral = if let Some(m) = room.find(find::MINERALS).get(0) {
      Some(m.into())
    } else {
      None
    };

    let deposit = if let Some(m) = room.find(find::DEPOSITS).get(0) {
      Some(m.into())
    } else {
      None
    };

    for s in room.find(find::SOURCES) {
      sources.push(s.into());
    }

    for site in room.find(find::CONSTRUCTION_SITES) {
      let entry = construction.entry(site.structure_type()).or_default();
      entry.push(site.into());
    }

    for s in room.find(find::MY_STRUCTURES) {
      let entry = structures.entry(s.structure_type()).or_default();
      entry.push(s.as_structure().into());
    }

    RoomData {
      name,
      level,
      construction,
      structures,
      sources,
      mineral,
      deposit,
    }
  }
}

impl RoomData {
  /// Determine if this room is currently visible
  pub fn is_visible(&self) -> bool {
    game::rooms::get(self.name).is_some()
  }
}

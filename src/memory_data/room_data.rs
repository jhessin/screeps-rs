use crate::*;
use screeps::Density;

/// This serializes source data
#[derive(Serialize, Deserialize, Eq, PartialEq)]
pub struct SourceData {
  pos: Position,
  amount: u32,
  capacity: u32,
}

/// This serializes mineral data
#[derive(Serialize, Deserialize, Eq, PartialEq)]
pub struct MineralData {
  pos: Position,
  mineral_type: ResourceType,
  amount: u32,
  density: Density,
}

/// This serializes and wraps creeps
/// TODO make roles and add that to this
#[derive(Serialize, Deserialize, Eq, PartialEq)]
pub struct CreepData {
  name: String,
  hits: u32,
  max_hits: u32,
  parts: HashSet<Part>,
}

/// This serializes deposit data
#[derive(Serialize, Deserialize, Eq, PartialEq)]
pub struct DepositData {
  pos: Position,
  deposit_type: ResourceType,
  cooldown: u32,
}

/// The AgentCell is a single room and manages all of the info for that cell
#[derive(Serialize, Deserialize, Eq, PartialEq)]
pub struct RoomData {
  name: RoomName,
  level: u8,
  construction: HashMap<StructureType, HashSet<Position>>,
  structures: HashMap<StructureType, Vec<StructureData>>,
  sources: Vec<SourceData>,
  mineral: Option<MineralData>,
  deposit: Option<DepositData>,
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
    let mut construction: HashMap<StructureType, HashSet<Position>> =
      HashMap::new();
    let mut structures: HashMap<StructureType, Vec<StructureData>> =
      HashMap::new();
    let mut sources = Vec::<SourceData>::new();

    // get the mineral data
    let mineral = if let Some(m) = &room.find(find::MINERALS).get(0) {
      Some(MineralData {
        pos: m.pos(),
        mineral_type: m.mineral_type(),
        amount: m.mineral_amount(),
        density: m.density(),
      })
    } else {
      None
    };

    let deposit = if let Some(m) = &room.find(find::DEPOSITS).get(0) {
      Some(DepositData {
        pos: m.pos(),
        deposit_type: m.deposit_type(),
        cooldown: m.cooldown(),
      })
    } else {
      None
    };

    for s in room.find(find::SOURCES) {
      sources.push(SourceData {
        pos: s.pos(),
        amount: s.energy(),
        capacity: s.energy_capacity(),
      });
    }

    for site in room.find(find::CONSTRUCTION_SITES) {
      let entry = construction.entry(site.structure_type()).or_default();
      entry.insert(site.pos());
    }

    for s in room.find(find::MY_STRUCTURES) {
      let entry = structures.entry(s.structure_type()).or_default();
      entry.push(StructureData::new(s.as_structure()));
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

  /// Determine if this room is currently visible
  pub fn is_visible(&self) -> bool {
    game::rooms::get(self.name).is_some()
  }
}

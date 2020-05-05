use crate::*;

/// This is all the info required to identify a target given a RawObjectId
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub enum Target {
  /// Holds a standard structure type
  Structure(StructureData),
  /// Holds a tombstone
  Tombstone(TombstoneData),
  /// Holds a Ruin
  Ruin(RuinData),
  /// Holds a construction site
  ConstructionSite(ConstructionData),
  /// Holds a Source
  Source(SourceData),
  /// Holds a Deposit
  Deposit(DepositData),
  /// Holds a Mineral
  Mineral(MineralData),
  /// Holds a Creep
  Creep(CommonCreepData),
  /// Holds a basic target for simple scout tasks
  Path([Position; 2]),
}

impl HasPosition for Target {
  fn pos(&self) -> Position {
    match self {
      Target::Structure(s) => s.pos(),
      Target::Tombstone(s) => s.pos(),
      Target::Ruin(s) => s.pos(),
      Target::ConstructionSite(s) => s.pos(),
      Target::Source(s) => s.pos(),
      Target::Deposit(s) => s.pos(),
      Target::Mineral(s) => s.pos(),
      Target::Creep(s) => s.pos(),
      Target::Path(s) => s[0],
    }
  }
}

impl Target {
  /// Move a creep toward the target
  pub fn move_to(&self, c: &Creep) -> ReturnCode {
    let pos = self.pos();
    c.move_to(&pos)
  }

  /// Returns true if the creep is in the same room as the target
  pub fn same_room<T: HasPosition>(&self, p: &T) -> bool {
    p.pos().room_name() == self.pos().room_name()
  }

  /// Returns a harvestable reference
  pub fn as_harvestable(&self) -> Option<Box<dyn Harvestable>> {
    match self {
      Target::Source(s) => Some(Box::new(s.unwrap())),
      Target::Deposit(s) => Some(Box::new(s.unwrap())),
      Target::Mineral(s) => Some(Box::new(s.unwrap())),
      _ => None,
    }
  }
}

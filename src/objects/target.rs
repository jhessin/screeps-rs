//! A wrapper around the different types of objects that can be targets of creep actions.
use crate::*;

/// This is a full fledged target that can easily be used for different actions.
#[derive(Clone)]
pub enum Target {
  /// A source object
  Source(Source),
  /// A mineral object
  Mineral(Mineral),
  /// A Deposit
  Deposit(Deposit),
  /// A Structure
  Structure(Structure),
  /// A Tombstone
  Tombstone(Tombstone),
  /// A Ruin
  Ruin(Ruin),
  /// A dropped resource
  Resource(Resource),
  /// A construction Site
  ConstructionSite(ConstructionSite),
  /// A Creep
  Creep(Creep),
  /// A Power Creep
  PowerCreep(PowerCreep),
  /// A simple flag
  Flag(Flag),
}

impl Display for Target {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    match self {
      Target::Source(_) => write!(f, "Source"),
      Target::Structure(_) => write!(f, "Structure"),
      Target::Tombstone(_) => write!(f, "Tombstone"),
      Target::Ruin(_) => write!(f, "Ruin"),
      Target::Resource(_) => write!(f, "Resource"),
      Target::ConstructionSite(_) => write!(f, "ConstructionSite"),
      Target::Creep(_) => write!(f, "Creep"),
      Target::Mineral(_) => write!(f, "Mineral"),
      Target::Deposit(_) => write!(f, "Deposit"),
      Target::PowerCreep(_) => write!(f, "PowerCreep"),
      Target::Flag(_) => write!(f, "Flag"),
    }
  }
}

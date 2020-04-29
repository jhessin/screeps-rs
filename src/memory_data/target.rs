use crate::*;

/// This is all the info required to identify a target given a RawObjectId
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub enum TargetType {
  /// Holds a standard structure type
  Structure(StructureType),
  /// Holds a tombstone
  Tombstone,
  /// Holds a Ruin
  Ruin,
  /// Holds a construction site
  ConstructionSite,
  /// Holds a Source
  Source,
  /// Holds a Deposit
  Deposit,
  /// Holds a Mineral
  Mineral,
}

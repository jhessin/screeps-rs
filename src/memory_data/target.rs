use crate::*;

/// This is all the info required to identify a target given a RawObjectId
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub enum TargetType {
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
}

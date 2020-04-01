//! Holds global constants that can be used throughout the program.

/// Holds all the names we will rotate through.
pub const NAMES: [&str; 20] = [
  "Jim",
  "Crystal",
  "Nathan",
  "Samuel",
  "Anna",
  "Tom",
  "Diane",
  "Tina",
  "David",
  "Andy",
  "Joey",
  "Gary",
  "Pat",
  "Chris",
  "Sarah",
  "Elizabeth",
  "Luke",
  "Bethany",
  "Deborah",
  "Ezra",
];

/// The role key constants used to indicate a role.
/// Harvester key
pub const HARVESTER: &str = "harvester";
/// Miner key
pub const MINER: &str = "miner";
/// Upgrader key
pub const UPGRADER: &str = "upgrader";
/// builder key
pub const BUILDER: &str = "builder";
/// repairer key
pub const REPAIRER: &str = "repairer";
/// wall-repairer key
pub const WALL_REPAIRER: &str = "wallRepairer";
/// lorry key
pub const LORRY: &str = "lorry";
/// specialist key
pub const SPECIALIST: &str = "specialist";

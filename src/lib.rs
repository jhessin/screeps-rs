//! This crate provides a rust based foundation for playing the screeps game.
#![warn(missing_docs)]

pub use std::collections::HashMap;
/// collections
pub use std::collections::HashSet;
/// Standard display format for saving to memory.
pub use std::fmt::{Display, Formatter, Result};
pub use std::str::FromStr;

/// Basic logging through debug!, info!, warn!, and error! macros.
pub use log::*;
/// The screeps imports
pub use screeps::{
  find, game,
  memory::MemoryReference,
  objects::*,
  pathfinder::{search, SearchOptions},
  prelude::*,
  ObjectId, Part, Position, RawObjectId, ResourceType, ReturnCode,
  RoomObjectProperties,
};
pub use serde::{Deserialize, Serialize};
pub use serde_json::{from_str, to_string};
/// If we want to use js
pub use stdweb::js;

/// My modules
pub use constants::*;
pub use game_loop::*;
pub use init::*;
pub use memory::*;
pub use objects::*;
pub use traits::*;

/// access the logging module
pub mod logging;

mod constants;
mod game_loop;
mod init;
mod memory;
mod objects;
mod traits;

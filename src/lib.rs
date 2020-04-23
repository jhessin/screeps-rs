//! This crate provides a rust based foundation for playing the screeps game.
#![warn(missing_docs)]

pub use std::collections::HashMap;
/// collections
pub use std::collections::HashSet;
/// Standard display format for saving to memory.
pub use std::fmt::{Display, Formatter, Result};
pub use std::str::FromStr;
pub use stdweb::Reference;

/// Basic logging through debug!, info!, warn!, and error! macros.
pub use log::*;
pub use ron::{de::from_str, ser::to_string};
/// The screeps imports
pub use screeps::{
  find, game,
  memory::MemoryReference,
  objects::*,
  pathfinder::{search, SearchOptions},
  prelude::*,
  traits::{IntoExpectedType, TryFrom, TryInto},
  ObjectId, Part, Position, RawObjectId, ResourceType, ReturnCode,
  RoomObjectProperties,
};
pub use serde::{Deserialize, Serialize};
/// If we want to use js
pub use stdweb::js;

pub use enums::*;
pub use game_loop::*;
pub use init::*;
pub use memory::*;
/// My modules
pub use names::*;
pub use objects::*;
pub use rooms::*;
pub use traits::*;

/// access the logging module
pub mod logging;

mod enums;
mod game_loop;
mod init;
mod memory;
mod names;
mod objects;
mod rooms;
mod traits;

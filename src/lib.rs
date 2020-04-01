//! This crate provides a rust based foundation for playing the screeps game.
#![warn(missing_docs)]

/// collections
pub use std::collections::HashSet;
/// Standard display format for saving to memory.
pub use std::fmt::{Display, Formatter, Result};
pub use std::str::FromStr;

/// Basic logging through debug!, info!, warn!, and error! macros.
pub use log::*;
/// The screeps imports
pub use screeps::{
  find, game, memory::MemoryReference, ObjectId, objects::*, Part, prelude::*,
  RawObjectId, ResourceType, ReturnCode, RoomObjectProperties,
};
/// If we want to use js
pub use stdweb::js;

pub use constants::*;
pub use game_loop::*;
pub use init::*;
pub use memory::*;
pub use objects::*;

/// access the logging module
pub mod logging;

mod constants;
mod game_loop;
mod init;
mod memory;
mod objects;

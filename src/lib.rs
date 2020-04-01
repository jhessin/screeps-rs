//! This crate provides a rust based foundation for playing the screeps game.
#![warn(missing_docs)]

/// collections
pub use std::collections::HashSet;

/// Basic logging through debug!, info!, warn!, and error! macros.
pub use log::*;
/// The screeps imports
pub use screeps::{
  find, objects::*, Part, prelude::*, ResourceType, ReturnCode,
  RoomObjectProperties,
};
/// If we want to use js
pub use stdweb::js;

pub use game_loop::*;
pub use init::*;
pub use memory::*;

/// access the logging module
pub mod logging;

mod game_loop;
mod init;
mod memory;

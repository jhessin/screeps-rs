//! This crate provides a rust based foundation for playing the screeps game.
#![warn(missing_docs)]

pub use std::{
  collections::BTreeMap,
  collections::HashSet,
  fmt::{Display, Formatter, Result},
  str::FromStr,
};
pub use stdweb::Reference;

/// Basic logging through debug!, info!, warn!, and error! macros.
pub use log::*;
pub use ron::{de::from_str, ser::to_string};
/// The screeps imports
pub use screeps::{
  find,
  game::{
    self,
    market::{Order, OrderType},
  },
  memory::root,
  memory::MemoryReference,
  objects::*,
  pathfinder::{search, SearchOptions},
  prelude::*,
  traits::{IntoExpectedType, TryFrom, TryInto},
  MarketResourceType, ObjectId,
  Part::{self, *},
  Position, RawObjectId,
  ResourceType::{self, *},
  ReturnCode, RoomName, RoomObjectProperties, StructureType,
};
pub use serde::{Deserialize, Serialize};
/// If we want to use js
pub use stdweb::js;

pub use game_loop::*;
pub use init::*;
pub use memory::*;
/// My modules
pub use names::*;
pub use rooms::*;
pub use traits::*;

/// access the logging module
pub mod logging;

mod game_loop;
mod init;
mod memory;
mod names;
mod rooms;
mod traits;

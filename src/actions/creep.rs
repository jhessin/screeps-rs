//! Bring Creep Actions into a collection.

mod default;

use screeps::Creep;
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::types::GeneralError;

pub use default::Default;

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
enum ActionStates {
    Undefined = 0,
    Idle = 1,

    Harvesting = 10,
    Offloading = 11,
}

/// Define the standardised behaviours for a creep action.
///
/// The primary behaviour needed for a creep action is tick.  This is the method which will
/// provide the core functionality of the implemented action.
pub trait CreepAction {
    ///Execute the core behaviour of a CreepAction
    fn tick(creep: Creep) -> Result<(), GeneralError>;
}

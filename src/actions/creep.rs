//! Bring Creep Actions into a collection.

mod harvester;

use screeps::Creep;

use crate::types::GeneralError;

pub use harvester::Harvester;


/// Define the standardised behaviours for a creep action.
///
/// The primary behaviour needed for a creep action is tick.  This is the method which will
/// provide the core functionality of the implemented action.
pub trait CreepAction {
    ///Execute the core behaviour of a CreepAction
    fn tick(creep: Creep) -> Result<(), GeneralError>;
}

use serde_repr::{Deserialize_repr, Serialize_repr};

pub type GeneralError = Box<dyn std::error::Error>;

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum ActionStates {
    Undefined = 0,
    Idle = 1,

    Harvesting = 10,
    Offloading = 11,
}

//! A wrapper around the different types of objects that can be targets of creep actions.
use crate::*;

/// This identifies what type of target we have.
/// It is serialized for being held in RoleData
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum SerializedTarget {
  /// A Source game object
  Source(String),
  /// A Structure game object
  Structure(String),
  /// A Tombstone game object
  Tombstone(String),
  /// A Ruin
  Ruin(String),
  /// A dropped resource
  Resource(String),
  /// A Creep
  Creep(String),
}

impl Display for SerializedTarget {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    match self {
      SerializedTarget::Source(_) => write!(f, "Source"),
      SerializedTarget::Structure(_) => write!(f, "Structure"),
      SerializedTarget::Tombstone(_) => write!(f, "Tombstone"),
      SerializedTarget::Ruin(_) => write!(f, "Ruin"),
      SerializedTarget::Resource(_) => write!(f, "Resource"),
      SerializedTarget::Creep(_) => write!(f, "Creep"),
    }
  }
}

impl SerializedTarget {
  /// Upgrades the TargetType to a full fledged Target
  pub fn upgrade(&self) -> Option<Target> {
    match self {
      SerializedTarget::Source(id) => {
        if let Ok(source) = ObjectId::<Source>::from_str(id) {
          if let Some(source) = source.resolve() {
            return Some(Target::Source(source));
          }
        }
      }
      SerializedTarget::Structure(id) => {
        if let Ok(source) = ObjectId::<Structure>::from_str(id) {
          if let Some(source) = source.resolve() {
            return Some(Target::Structure(source));
          }
        }
      }
      SerializedTarget::Tombstone(id) => {
        if let Ok(source) = ObjectId::<Tombstone>::from_str(id) {
          if let Some(source) = source.resolve() {
            return Some(Target::Tombstone(source));
          }
        }
      }
      SerializedTarget::Ruin(id) => {
        if let Ok(source) = ObjectId::<Ruin>::from_str(id) {
          if let Some(source) = source.resolve() {
            return Some(Target::Ruin(source));
          }
        }
      }
      SerializedTarget::Resource(id) => {
        if let Ok(source) = ObjectId::<Resource>::from_str(id) {
          if let Some(source) = source.resolve() {
            return Some(Target::Resource(source));
          }
        }
      }
      SerializedTarget::Creep(id) => {
        if let Ok(source) = ObjectId::<Creep>::from_str(id) {
          if let Some(source) = source.resolve() {
            return Some(Target::Creep(source));
          }
        }
      }
    }

    None
  }
}

/// This is a full fledged target that can easily be used for different actions.
pub enum Target {
  /// A source object
  Source(Source),
  /// A Structure
  Structure(Structure),
  /// A Tombstone
  Tombstone(Tombstone),
  /// A Ruin
  Ruin(Ruin),
  /// A dropped resource
  Resource(Resource),
  /// A Creep
  Creep(Creep),
}

impl Target {
  /// Downgrades to a TargetType for serialization
  pub fn downgrade(&self) -> SerializedTarget {
    match self {
      Target::Source(obj) => {
        let id = obj.id().to_string();
        SerializedTarget::Source(id)
      }
      Target::Structure(obj) => {
        let id = obj.id().to_string();
        SerializedTarget::Structure(id)
      }
      Target::Tombstone(obj) => {
        let id = obj.id().to_string();
        SerializedTarget::Tombstone(id)
      }
      Target::Ruin(obj) => {
        let id = obj.id().to_string();
        SerializedTarget::Ruin(id)
      }
      Target::Resource(obj) => {
        let id = obj.id().to_string();
        SerializedTarget::Source(id)
      }
      Target::Creep(obj) => {
        let id = obj.id().to_string();
        SerializedTarget::Source(id)
      }
    }
  }
}

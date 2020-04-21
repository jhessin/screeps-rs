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
  /// A construction site
  ConstructionSite(String),
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
      SerializedTarget::ConstructionSite(_) => write!(f, "Construction Site"),
    }
  }
}

impl SerializedTarget {
  /// Upgrades the TargetType to a full fledged Target
  #[allow(clippy::cognitive_complexity)]
  pub fn upgrade(&self) -> Option<Target> {
    debug!("Upgrading Target");
    match self {
      SerializedTarget::Source(id) => {
        debug!("Target is a Source with id {}", &id);
        if let Ok(source) = ObjectId::<Source>::from_str(id) {
          if let Some(source) = source.resolve() {
            let target = Target::Source(source);
            debug!("Target is a {}", &target);
            return Some(target);
          }
        }
      }
      SerializedTarget::Structure(id) => {
        debug!("Target is a Structure with id {}", &id);
        if let Ok(source) = ObjectId::<Structure>::from_str(id) {
          if let Some(source) = source.resolve() {
            let target = Target::Structure(source);
            debug!("Target is a {}", &target);
            return Some(target);
          }
        }
      }
      SerializedTarget::Tombstone(id) => {
        debug!("Target is a Tombstone with id {}", &id);
        if let Ok(source) = ObjectId::<Tombstone>::from_str(id) {
          if let Some(source) = source.resolve() {
            let target = Target::Tombstone(source);
            debug!("Target is a {}", &target);
            return Some(target);
          }
        }
      }
      SerializedTarget::Ruin(id) => {
        debug!("Target is a Ruin with id {}", &id);
        if let Ok(source) = ObjectId::<Ruin>::from_str(id) {
          if let Some(source) = source.resolve() {
            let target = Target::Ruin(source);
            debug!("Target is a {}", &target);
            return Some(target);
          }
        }
      }
      SerializedTarget::Resource(id) => {
        debug!("Target is a Resource with id {}", &id);
        if let Ok(source) = ObjectId::<Resource>::from_str(id) {
          if let Some(source) = source.resolve() {
            let target = Target::Resource(source);
            debug!("Target is a {}", &target);
            return Some(target);
          }
        }
      }
      SerializedTarget::Creep(id) => {
        debug!("Target is a Creep with id {}", &id);
        if let Ok(source) = ObjectId::<Creep>::from_str(id) {
          if let Some(source) = source.resolve() {
            let target = Target::Creep(source);
            debug!("Target is a {}", &target);
            return Some(target);
          }
        }
      }
      SerializedTarget::ConstructionSite(id) => {
        debug!("Target is a ConstructionSite with id {}", &id);
        if let Ok(source) = ObjectId::<ConstructionSite>::from_str(id) {
          if let Some(source) = source.resolve() {
            let target = Target::ConstructionSite(source);
            debug!("Target is a {}", &target);
            return Some(target);
          }
        }
      }
    }

    debug!("Invalid target -> Returning None");
    None
  }
}

/// This is a full fledged target that can easily be used for different actions.
#[derive(Clone)]
pub enum Target {
  /// A source object
  Source(Source),
  /// A mineral object
  Mineral(Mineral),
  /// A Deposit
  Deposit(Deposit),
  /// A Structure
  Structure(Structure),
  /// A Tombstone
  Tombstone(Tombstone),
  /// A Ruin
  Ruin(Ruin),
  /// A dropped resource
  Resource(Resource),
  /// A construction Site
  ConstructionSite(ConstructionSite),
  /// A Creep
  Creep(Creep),
  /// A Power Creep
  PowerCreep(PowerCreep),
  /// A simple flag
  Flag(Flag),
}

impl Display for Target {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    match self {
      Target::Source(_) => write!(f, "Source"),
      Target::Structure(_) => write!(f, "Structure"),
      Target::Tombstone(_) => write!(f, "Tombstone"),
      Target::Ruin(_) => write!(f, "Ruin"),
      Target::Resource(_) => write!(f, "Resource"),
      Target::ConstructionSite(_) => write!(f, "ConstructionSite"),
      Target::Creep(_) => write!(f, "Creep"),
    }
  }
}

impl Target {
  /// Downgrades to a TargetType for serialization
  pub fn downgrade(&self) -> SerializedTarget {
    debug!("Downgrading Target");
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
        SerializedTarget::Resource(id)
      }
      Target::Creep(obj) => {
        let id = obj.id().to_string();
        SerializedTarget::Creep(id)
      }
      Target::ConstructionSite(obj) => {
        let id = obj.id().to_string();
        SerializedTarget::ConstructionSite(id)
      }
    }
  }
}

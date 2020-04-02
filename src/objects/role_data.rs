//! RoleData contains any data that a role may need to function.
use crate::*;

/// This contains any data a role may need. If it doesn't need it this may be None.
#[derive(Serialize, Deserialize, Clone)]
pub struct RoleData {
  /// This can be the id for a Source if we are a miner
  /// OR it could contain the id of a structure if we are a specialist
  source_id: Option<String>,
  /// Miners may have an assigned container that they go to.
  container_id: Option<String>,
  /// Specialists will keep the id of a structure for a target
  target_id: Option<String>,
  /// This is a ratio that a wall repairer will use to cut back on cpu
  pub ratio: Option<f64>,
}

impl Default for RoleData {
  fn default() -> Self {
    RoleData {
      source_id: None,
      container_id: None,
      target_id: None,
      ratio: None,
    }
  }
}

/// Basic data access
impl RoleData {
  /// This returns the source for a miner
  pub fn source(&self) -> Option<Source> {
    match &self.source_id {
      Some(id) => match ObjectId::<Source>::from_str(&id) {
        Ok(id) => id.resolve(),
        _ => None,
      },
      _ => None,
    }
  }

  /// This returns the container for a miner
  pub fn container(&self) -> Option<StructureContainer> {
    match &self.container_id {
      Some(id) => match ObjectId::<StructureContainer>::from_str(&id) {
        Ok(id) => id.resolve(),
        _ => None,
      },
      _ => None,
    }
  }

  /// This returns the source structure for a specialist
  pub fn source_structure(&self) -> Option<Structure> {
    match &self.source_id {
      Some(id) => match ObjectId::<Structure>::from_str(&id) {
        Ok(id) => id.resolve(),
        _ => None,
      },
      _ => None,
    }
  }

  /// This returns the target for a specialist, or a wall-repairer
  pub fn target(&self) -> Option<Structure> {
    match &self.target_id {
      Some(id) => match ObjectId::<Structure>::from_str(&id) {
        Ok(id) => id.resolve(),
        _ => None,
      },
      _ => None,
    }
  }
}

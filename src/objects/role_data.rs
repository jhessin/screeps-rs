//! RoleData contains any data that a role may need to function.
use crate::*;

/// This contains any data a role may need. If it doesn't need it this may be None.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RoleData {
  /// This contains the id of the object we are getting energy from.
  source_id: Option<SerializedTarget>,
  /// This contains the id of the object we are taking energy toward
  target_id: Option<SerializedTarget>,
  /// This is a ratio that a wall repairer will use to cut back on cpu
  pub ratio: Option<f64>,
}

impl Default for RoleData {
  fn default() -> Self {
    RoleData {
      source_id: None,
      target_id: None,
      ratio: None,
    }
  }
}

/// Basic data access
impl RoleData {
  /// This returns the source for a miner
  pub fn source(&self) -> Option<Target> {
    match &self.source_id {
      Some(id) => id.upgrade(),
      _ => None,
    }
  }

  /// Sets the source_id using the source
  pub fn set_source(&mut self, source: Target) {
    self.source_id = Some(source.downgrade());
  }

  /// This returns the target for a specialist, or a wall-repairer
  pub fn target(&self) -> Option<Target> {
    match &self.target_id {
      Some(id) => id.upgrade(),
      _ => None,
    }
  }

  /// Sets the target id from a specified structure.
  pub fn set_target(&mut self, target: Target) {
    self.target_id = Some(target.downgrade());
  }
}

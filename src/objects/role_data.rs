//! RoleData contains any data that a role may need to function.
use crate::*;

/// This contains any data a role may need. If it doesn't need it this may be None.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RoleData {
  /// This contains the id of the object we are getting energy from.
  pub source_id: Option<SerializedTarget>,
  /// This contains the id of the object we are taking energy toward
  pub target_id: Option<SerializedTarget>,
  /// This is a ratio that a wall repairer will use to cut back on cpu
  pub ratio: Option<f64>,
}

impl Default for RoleData {
  fn default() -> Self {
    RoleData { source_id: None, target_id: None, ratio: None }
  }
}

/// Basic data access
impl RoleData {
  /// This returns the source for a miner
  pub fn source(&mut self) -> Option<Target> {
    let result = match &self.source_id {
      Some(id) => id.upgrade(),
      _ => None,
    };

    if result.is_none() {
      self.source_id = None;
    }

    result
  }

  /// Sets the source_id using the source
  pub fn set_source(&mut self, source: Target) {
    self.source_id = Some(source.downgrade());
  }

  /// Resets the source_id to None
  pub fn reset_source(&mut self) {
    self.source_id = None;
  }

  /// This returns the target for a specialist, or a wall-repairer
  pub fn target(&mut self) -> Option<Target> {
    let result = match &self.target_id {
      Some(id) => id.upgrade(),
      _ => None,
    };

    if result.is_none() {
      self.target_id = None;
    }

    result
  }

  /// Resets the target_id to None
  pub fn reset_target(&mut self) {
    self.target_id = None;
  }

  /// Sets the target id from a specified structure.
  pub fn set_target(&mut self, target: Target) {
    self.target_id = Some(target.downgrade());
  }
}

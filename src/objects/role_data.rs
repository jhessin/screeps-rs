//! RoleData contains any data that a role may need to function.
use screeps::ResourceType::Energy;

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
  pub fn set_source(&mut self, source: &Target) {
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

  /// Validates target for delivery
  pub fn validate_deliver_target(&mut self) {
    if let Some(t) = self.target() {
      match t {
        Target::Source(_) => self.reset_target(),
        Target::Structure(s) => {
          if let Some(s) = s.as_has_store() {
            if s.store_free_capacity(Some(Energy)) == 0 {
              self.reset_target()
            }
          }
        }
        Target::Tombstone(_) => self.reset_target(),
        Target::Ruin(_) => self.reset_target(),
        Target::Resource(_) => self.reset_target(),
        Target::ConstructionSite(_) => self.reset_target(),
        Target::Creep(c) => {
          if c.store_free_capacity(Some(Energy)) == 0 {
            self.reset_target();
          }
        }
      }
    }
  }

  /// validates source for withdraw
  pub fn validate_gather_source(&mut self) {
    if let Some(t) = self.source() {
      match t {
        Target::Source(s) => {
          if s.energy() == 0 {
            self.reset_source();
          }
        }
        Target::Structure(s) => {
          if let Some(w) = s.as_has_store() {
            if w.store_used_capacity(Some(Energy)) == 0 {
              self.reset_source();
            }
          }
        }
        Target::Tombstone(t) => {
          if t.store_used_capacity(Some(Energy)) == 0 {
            self.reset_source();
          }
        }
        Target::Ruin(r) => {
          if r.store_used_capacity(Some(Energy)) == 0 {
            self.reset_source();
          }
        }
        Target::Resource(r) => {
          if r.amount() == 0 {
            self.reset_source();
          }
        }
        Target::ConstructionSite(_) => {
          self.reset_source();
        }
        Target::Creep(c) => {
          let creep = Creeper::new(c);
          if !(creep.role == Role::lorry() || creep.role == Role::harvester()) {
            self.reset_source();
          }
        }
      }
    }
  }

  /// Validate repair target
  pub fn validate_repair_target(&mut self) {
    if let Some(t) = self.target() {
      match t {
        Target::Structure(s) => {
          if let Some(a) = s.as_attackable() {
            if a.hits() == a.hits_max() {
              self.reset_target();
            }
          }
        }
        _ => self.reset_target(),
      }
    }
  }

  /// Validate build target
  pub fn validate_build_target(&mut self) {
    if let Some(Target::ConstructionSite(_)) = self.target() {
      // all good here.
    } else {
      // This is no good.
      self.reset_target();
    }
  }

  /// Sets the target id from a specified structure.
  pub fn set_target(&mut self, target: &Target) {
    self.target_id = Some(target.downgrade());
  }
}

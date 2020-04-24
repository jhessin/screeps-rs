use crate::*;

/// This holds all the memory keys that I will use. Simply use a to_str or json::from_str on them
#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub enum Keys {
  /// The action that a creep is taking
  Action,
  /// The target of the action
  TargetId,
  /// The resource that the creep is dealing in.
  Resource,
  /// The key for the role of the creep
  Role,
  /// The key for holding the username
  Username,
  /// The key for whether the creep is working or not
  Working,
}

impl Display for Keys {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    write!(f, "{}", to_string(self).unwrap())
  }
}

/// A wrapper for the appropriate values
pub enum Values {
  /// The action that a creep is taking
  Action(Actions),
  /// The target that the creep is using
  TargetId(String),
  /// The resource the creep is dealing with
  Resource(ResourceType),
  /// The Value for the Role
  Role(Role),
  /// Username
  Username(String),
  /// Is the creep working
  Working(bool),
}

/// Shortcuts for setting values in memory
pub trait ValueSet {
  /// Sets the value using the appropriate keys
  fn set_value(self, value: Values) -> Self;
  /// gets the value given the appropriate key
  fn get_value(&self, key: Keys) -> Option<Values>;
  /// Deletes the value at the specified key location
  fn rm_value(&self, key: Keys) -> bool;
}

impl ValueSet for MemoryReference {
  fn set_value(self, value: Values) -> Self {
    match value {
      Values::Action(d) => self.set(
        &to_string(&Keys::Action).expect("Invalid Key"),
        to_string(&d).expect("Invalid Action string"),
      ),
      Values::TargetId(d) => {
        self.set(&to_string(&Keys::TargetId).expect("Invalid Key"), d)
      }
      Values::Resource(d) => self.set(
        &to_string(&Keys::Resource).expect("Invalid Key"),
        to_string(&d).expect("Invalid resource type"),
      ),
      Values::Role(d) => self.set(
        &to_string(&Keys::Role).expect("Invalid Key"),
        to_string(&d).expect("Invalid Role string"),
      ),
      Values::Username(d) => {
        self.set(&to_string(&Keys::Username).expect("Invalid Key"), d)
      }
      Values::Working(d) => {
        self.set(&to_string(&Keys::Working).expect("Invalid Key"), d)
      }
    }
    self
  }

  fn get_value(&self, key: Keys) -> Option<Values> {
    let result_str = if let Ok(Some(r)) =
      self.string(&to_string(&key).expect("Invalid Key"))
    {
      r
    } else {
      return None;
    };
    match key {
      Keys::Action => {
        if let Ok(action) = from_str::<Actions>(&result_str) {
          return Some(Values::Action(action));
        }
      }
      Keys::TargetId => return Some(Values::TargetId(result_str)),
      Keys::Resource => {
        if let Ok(resource) = from_str::<ResourceType>(&result_str) {
          return Some(Values::Resource(resource));
        }
      }
      Keys::Role => {
        if let Ok(role) = from_str::<Role>(&result_str) {
          return Some(Values::Role(role));
        }
      }
      Keys::Username => return Some(Values::Username(result_str)),
      Keys::Working => {
        let data = self.bool(&to_string(&Keys::Working).expect("Invalid Key"));
        return Some(Values::Working(data));
      }
    }
    None
  }

  fn rm_value(&self, key: Keys) -> bool {
    if let Ok(key) = to_string(&key) {
      self.del(&key);
      true
    } else {
      false
    }
  }
}

/// This holds all the actions that a creep could take on a target.
#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub enum Actions {
  /// Attack the target
  Attack,
  /// Attack a controller
  AttackController,
  /// Build
  Build,
  /// Claim a controller
  ClaimController,
  /// Dismantle a structure
  Dismantle,
  /// Generate a Safe Mode
  GenerateSafeMode,
  /// Harvest
  Harvest,
  /// Heal the target
  Heal,
  /// Pickup a resource
  Pickup,
  /// Pull another creep
  Pull,
  /// Repair a structure
  Repair,
  /// Reserve a controller
  ReserveController,
  /// Sign a controller
  SignController,
  /// Upgrade a controller
  UpgradeController,
  /// Transfer the indicated resource
  Transfer,
  /// Withdraw the indicated resource
  Withdraw,
}

impl Display for Actions {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    write!(f, "{}", to_string(self).unwrap())
  }
}

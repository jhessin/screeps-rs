use crate::*;
use std::ops::DerefMut;

/// This hold all the info for a task te be given to a creep
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct Task {
  action_queue: VecDeque<(Action, Target)>,
}

impl Display for Task {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    for (a, t) in &self.action_queue {
      writeln!(f, "{:?} -> {:?}", a, t)?;
    }

    Ok(())
  }
}

impl Default for Task {
  fn default() -> Self {
    let action_queue = VecDeque::new();
    Task { action_queue }
  }
}

impl Deref for Task {
  type Target = VecDeque<(Action, Target)>;

  fn deref(&self) -> &Self::Target {
    &self.action_queue
  }
}

impl DerefMut for Task {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.action_queue
  }
}

impl Task {
  /// Get all associated actions for this task
  pub fn actions(&self) -> HashSet<&Action> {
    let mut actions = HashSet::new();

    for (a, _) in &self.action_queue {
      actions.insert(a);
    }

    actions
  }

  /// Get all the required body parts for a task
  pub fn parts_required(&self) -> HashSet<Part> {
    let mut parts = HashSet::new();
    for (a, _) in &self.action_queue {
      parts = parts.union(&a.req_parts()).cloned().collect();
    }

    parts
  }

  /// Get the required ticks to complete this task
  pub fn ticks_required(&self, _creep: &CommonCreepData) -> u32 {
    todo!()
  }
}

use crate::*;
use std::ops::DerefMut;

/// This hold all the info for a task te be given to a creep
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct Task(VecDeque<(Action, Target)>);

impl Display for Task {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    for (a, t) in &self.0 {
      writeln!(f, "{:?} -> {:?}", a, t)?;
    }

    Ok(())
  }
}

impl Default for Task {
  fn default() -> Self {
    Task(VecDeque::new())
  }
}

impl Deref for Task {
  type Target = VecDeque<(Action, Target)>;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DerefMut for Task {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}

impl Task {
  /// Get all associated actions for this task
  pub fn actions(&self) -> HashSet<&Action> {
    let mut actions = HashSet::new();

    for (a, _) in &self.0 {
      actions.insert(a);
    }

    actions
  }

  /// get all the associated targets
  pub fn targets(&self) -> HashSet<Position> {
    let mut targets = HashSet::new();

    for (_, t) in &self.0 {
      targets.insert(t.pos());
    }

    targets
  }

  /// Is the task paved?
  pub fn is_paved_from<T: HasPosition>(&self, source: T) -> bool {
    todo!("Return if this is paved from {} to all the targets", source.pos())
  }

  /// Get all the required body parts for a task
  pub fn parts_required(&self) -> HashSet<Part> {
    let mut parts = HashSet::new();
    for (a, _) in &self.0 {
      parts = parts.union(&a.req_parts()).cloned().collect();
    }

    parts
  }
}

use crate::*;

/// Extends common creep data to add tasks to creeps
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct MyCreepData {
  parent: CommonCreepData,
  task: Option<Task>,
}

impl Deref for MyCreepData {
  type Target = CommonCreepData;

  fn deref(&self) -> &Self::Target {
    &self.parent
  }
}

impl From<Creep> for MyCreepData {
  fn from(c: Creep) -> Self {
    let parent = c.into();
    let task = None;
    MyCreepData { parent, task }
  }
}

impl From<PowerCreep> for MyCreepData {
  fn from(c: PowerCreep) -> Self {
    let parent = c.into();
    let task = None;
    MyCreepData { parent, task }
  }
}

impl MyCreepData {
  /// Assign a task to this creep if there isn't one assigned
  pub fn assign(&mut self, task: Task) -> bool {
    if self.task.is_none() {
      self.task = Some(task);
      true
    } else {
      false
    }
  }
}

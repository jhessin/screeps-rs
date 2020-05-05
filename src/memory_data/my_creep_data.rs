use crate::*;

const TASK_KEY: &str = "task";

/// Extends common creep data to add tasks to creeps
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct MyCreepData {
  parent: CommonCreepData,
  task: Task,
}

impl Deref for MyCreepData {
  type Target = CommonCreepData;

  fn deref(&self) -> &Self::Target {
    &self.parent
  }
}

impl From<Creep> for MyCreepData {
  fn from(c: Creep) -> Self {
    let task = if let Ok(Some(t)) = c.memory().arr(TASK_KEY) {
      deserialize::<Task>(&t).unwrap_or_default()
    } else {
      Task::default()
    };
    let parent = c.into();
    MyCreepData { parent, task }
  }
}

impl From<PowerCreep> for MyCreepData {
  fn from(c: PowerCreep) -> Self {
    let task = if let Ok(Some(t)) = c.memory().arr(TASK_KEY) {
      deserialize::<Task>(&t).unwrap_or_default()
    } else {
      Task::default()
    };
    let parent = c.into();
    MyCreepData { parent, task }
  }
}

impl MyCreepData {
  /// Unwrap my creep
  pub fn unwrap_creep(&self) -> Creep {
    game::creeps::get(&self.name()).unwrap()
  }

  /// Unwrap power creep
  pub fn unwrap_power_creep(&self) -> AccountPowerCreep {
    game::power_creeps::get(&self.name()).unwrap()
  }

  /// Run assigned task or recycle this creep if the task is complete
  pub fn run(&mut self) {
    // TODO clean this up and split it up
    let creep = self.unwrap_creep();
    if let Some((a, target)) = self.task.pop_front() {
      match a {
        Action::Harvest => {
          if target.same_room(&self.parent) {
            // normal harvest
            if let Some(t) = target.as_harvestable() {
              let code = creep.harvest(t.as_ref());
              if code == ReturnCode::NotInRange {
                creep.move_to(t.as_ref());
                creep.harvest(t.as_ref());
              } else if code != ReturnCode::Ok {
                warn!("Problem harvesting @ {:?}", target);
              }
              // check if the task is completed
              if creep.store_free_capacity(None) == 0 {
                return;
              }
              match &target {
                Target::Source(s) => {
                  if s.unwrap().energy() == 0 {
                    return;
                  } else {
                    self.task.push_front((a, target));
                  }
                }
                Target::Deposit(_) => {
                  return;
                }
                Target::Mineral(m) => {
                  if m.unwrap().mineral_amount() == 0 {
                    return;
                  } else {
                    self.task.push_front((a, target));
                  }
                }
                _ => return,
              }
            }
          } else {
            target.move_to(&creep);
          }
        }
        Action::Mine => {}
        Action::Attack => {}
        Action::AttackController => {}
        Action::Build => {}
        Action::Claim => {}
        Action::Dismantle => {}
        Action::GenerateSafeMode => {}
        Action::Heal => {}
        Action::Pickup => {}
        Action::Repair => {}
        Action::Reserve => {}
        Action::Transfer => {}
        Action::Withdraw => {}
        Action::Scout => {}
        Action::Upgrade => {}
      }
    } else {
      self.recycle();
    }
  }

  /// Recycle this creep
  pub fn recycle(&self) -> Self {
    todo!()
  }
}

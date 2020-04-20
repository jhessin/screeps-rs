use crate::*;

/// This trait allows things to run!
pub trait Runner {
  /// Run the game object so it does what it is supposed to.
  fn run(&self) -> ReturnCode;
}

impl Runner for Creep {
  fn run(&self) -> ReturnCode {
    todo!()
  }
}

impl Runner for StructureTower {
  fn run(&self) -> ReturnCode {
    if let Some(target) = self.pos().find_closest_by_range(find::HOSTILE_CREEPS)
    {
      self.attack(&target)
    } else {
      for range in 0..79 {
        for creep in
          self.pos().find_in_range(find::MY_CREEPS, range) as Vec<Creep>
        {
          if creep.hits() < creep.hits_max() {
            return self.heal(&creep);
          }
        }
      }
      ReturnCode::Ok
    }
  }
}

impl Runner for StructureLink {
  fn run(&self) -> ReturnCode {
    todo!()
  }
}

impl Runner for StructureTerminal {
  fn run(&self) -> ReturnCode {
    todo!()
  }
}

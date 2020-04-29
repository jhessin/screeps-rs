use crate::*;

/// This hold all the info for a task te be given to a creep
#[derive(Serialize, Deserialize, Eq, PartialEq)]
pub struct Task {
  target: RawObjectId,
  target_type: TargetType,
  pos: Position,
  action: Action,
}

impl Display for Task {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    writeln!(f, "{:?} @ {:?}", self.action, self.target_type)?;

    writeln!(f, "\t({}, {})", self.pos.x(), self.pos.y())
  }
}

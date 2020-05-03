use crate::*;

/// This serializes source data
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct SourceData {
  pos: CommonData,
  amount: u32,
  capacity: u32,
}

impl HasPosition for SourceData {
  fn pos(&self) -> Position {
    self.pos.pos()
  }
}

impl Display for SourceData {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    writeln!(
      f,
      "Source {}: {} of {} available",
      self.pos, self.amount, self.capacity
    )
  }
}

impl From<Source> for SourceData {
  fn from(source: Source) -> Self {
    let pos = source.pos().into();
    let amount = source.energy();
    let capacity = source.energy_capacity();
    SourceData { pos, amount, capacity }
  }
}

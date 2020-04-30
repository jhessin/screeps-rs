use crate::*;

/// This serializes source data
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct SourceData {
  pos: Position,
  amount: u32,
  capacity: u32,
}

impl Display for SourceData {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    writeln!(
      f,
      "Source ({}, {}): {} of {} available",
      self.pos.x(),
      self.pos.y(),
      self.amount,
      self.capacity
    )
  }
}

impl SourceData {
  /// Generate a new SourceData from a source
  pub fn new(source: Source) -> Self {
    let pos = source.pos();
    let amount = source.energy();
    let capacity = source.energy_capacity();
    SourceData { pos, amount, capacity }
  }
}

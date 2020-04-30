use crate::*;

/// This serializes mineral data
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct MineralData {
  pos: Position,
  mineral_type: ResourceType,
  amount: u32,
  density: Density,
}

impl Display for MineralData {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    writeln!(
      f,
      "{:?}: ({}, {})",
      self.mineral_type,
      self.pos.x(),
      self.pos.y()
    )?;

    writeln!(f, "{:?}: {}", self.density, self.amount)
  }
}

impl MineralData {
  /// Generates MineralData from a Mineral
  pub fn new(mineral: &Mineral) -> Self {
    let pos = mineral.pos();
    let mineral_type = mineral.mineral_type();
    let amount = mineral.mineral_amount();
    let density = mineral.density();
    MineralData { pos, mineral_type, amount, density }
  }
}

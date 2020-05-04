use crate::*;

/// This serializes mineral data
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct MineralData {
  pos: CommonData,
  mineral_type: ResourceType,
  amount: u32,
  density: Density,
}

impl HasPosition for MineralData {
  fn pos(&self) -> Position {
    self.pos.pos()
  }
}

impl Display for MineralData {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    writeln!(f, "{:?}: {}", self.mineral_type, self.pos)?;

    writeln!(f, "{:?}: {}", self.density, self.amount)
  }
}

impl From<Mineral> for MineralData {
  fn from(mineral: Mineral) -> Self {
    let mineral_type = mineral.mineral_type();
    let amount = mineral.mineral_amount();
    let density = mineral.density();
    let pos = mineral.pos().into();
    MineralData { pos, mineral_type, amount, density }
  }
}

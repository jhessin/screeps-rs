use crate::*;

/// This serializes mineral data
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct MineralData {
  pos: CommonData,
  id: ObjectId<Mineral>,
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
    let id = mineral.id();
    let amount = mineral.mineral_amount();
    let density = mineral.density();
    let pos = mineral.pos().into();
    MineralData { pos, id, mineral_type, amount, density }
  }
}

impl MineralData {
  /// Unwrap the mineral
  pub fn unwrap(&self) -> Mineral {
    game::get_object_typed(self.id).unwrap().unwrap()
  }
}

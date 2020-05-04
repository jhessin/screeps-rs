use crate::*;

/// This serializes deposit data
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct DepositData {
  pos: CommonData,
  deposit_type: ResourceType,
  cooldown: u32,
}

impl HasPosition for DepositData {
  fn pos(&self) -> Position {
    self.pos.pos()
  }
}

impl Deref for DepositData {
  type Target = CommonData;

  fn deref(&self) -> &Self::Target {
    &self.pos
  }
}

impl Display for DepositData {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    writeln!(f, "{:?} - {}", self.deposit_type, self.pos,)?;

    writeln!(f, "Cooldown: {}", self.cooldown)
  }
}

impl From<Deposit> for DepositData {
  fn from(dep: Deposit) -> Self {
    let deposit_type = dep.deposit_type();
    let cooldown = dep.cooldown();
    let pos = dep.pos().into();

    DepositData { pos, deposit_type, cooldown }
  }
}

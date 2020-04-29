use crate::*;

/// This serializes deposit data
#[derive(Serialize, Deserialize, Eq, PartialEq)]
pub struct DepositData {
  pos: Position,
  deposit_type: ResourceType,
  cooldown: u32,
}

impl Display for DepositData {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    writeln!(
      f,
      "{:?} - ({},{})",
      self.deposit_type,
      self.pos.x(),
      self.pos.y()
    )?;

    writeln!(f, "Cooldown: {}", self.cooldown)
  }
}

impl DepositData {
  /// Create DepositData from a deposit
  pub fn new(dep: &Deposit) -> Self {
    let pos = dep.pos();
    let deposit_type = dep.deposit_type();
    let cooldown = dep.cooldown();

    DepositData { pos, deposit_type, cooldown }
  }
}

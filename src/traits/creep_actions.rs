use crate::*;

/// CreepActions gives Creeps superpowers.
pub trait CreepActions {
  /// This resets the memory of a creep - path data, target data, and action data
  fn reset_action(&self) -> ReturnCode;

  /// This will tell me if a creep is working as well as update the state
  fn working(&self) -> bool;

  /// This will travel to a target if it is out of range or report any other error.
  /// It also saves the target to the creeps memory.
  fn travel_or_report<T: RoomObjectProperties + HasId>(
    &self,
    code: ReturnCode,
    target: &T,
  ) -> ReturnCode;

  /// The following methods are all variations of existing creep methods
  /// They will both move toward a target as well as verify that it is valid for the action.
  /// - to avoid conflicts we will start them with go_*

  /// Attack
  fn go_attack<T: Attackable + HasId>(&self, target: &T) -> ReturnCode;

  /// Attack a structure
  fn go_attack_structure(&self, target: &Structure) -> ReturnCode;

  /// Attack Controller
  fn go_attack_controller(&self, target: &StructureController) -> ReturnCode;

  /// Build
  fn go_build(&self, target: &ConstructionSite) -> ReturnCode;

  /// Claim
  fn go_claim_controller(&self, target: &StructureController) -> ReturnCode;

  /// Dismantle
  fn go_dismantle(&self, target: &Structure) -> ReturnCode;

  /// GenerateSameMode
  fn go_generate_safe_mode(&self, target: &StructureController) -> ReturnCode;

  /// Harvest
  fn go_harvest<T: Harvestable + HasId>(&self, target: &T) -> ReturnCode;

  /// Heal
  fn go_heal_creep(&self, target: &Creep) -> ReturnCode;

  /// Heal Power Creep
  fn go_heal_power_creep(&self, target: &PowerCreep) -> ReturnCode;

  /// Pickup
  fn go_pickup(&self, target: &Resource) -> ReturnCode;

  /// Pull
  fn go_pull(&self, target: &Creep) -> ReturnCode;

  /// Repair
  fn go_repair(&self, target: &Structure) -> ReturnCode;

  /// ReserveController
  fn go_reserve_controller(&self, target: &StructureController) -> ReturnCode;

  /// SignController
  fn go_sign_controller(&self, target: &StructureController) -> ReturnCode;

  /// UpgradeController
  fn go_upgrade_controller(&self, target: &StructureController) -> ReturnCode;

  /// Transfer to structure
  fn go_transfer_to_structure(
    &self,
    target: &Structure,
    resource: ResourceType,
    amount: Option<u32>,
  ) -> ReturnCode;

  /// Transfer to anything else
  fn go_transfer<T: Transferable + HasStore + RoomObjectProperties + HasId>(
    &self,
    target: &T,
    resource: ResourceType,
    amount: Option<u32>,
  ) -> ReturnCode;

  /// Withdraw from structure
  fn go_withdraw_from_structure(
    &self,
    target: &Structure,
    resource: ResourceType,
    amount: Option<u32>,
  ) -> ReturnCode;

  /// Withdraw from anything else
  fn go_withdraw<T: Withdrawable + HasStore + RoomObjectProperties + HasId>(
    &self,
    target: &T,
    resource: ResourceType,
    amount: Option<u32>,
  ) -> ReturnCode;
}

impl CreepActions for Creep {
  fn reset_action(&self) -> ReturnCode {
    self.memory().del("_move");
    self.memory().rm_value(Keys::TargetId);
    self.memory().rm_value(Keys::Resource);
    self.memory().rm_value(Keys::Action);

    ReturnCode::InvalidTarget
  }

  fn working(&self) -> bool {
    let working = if let Some(Values::Working(w)) =
      self.memory().get_value(Keys::Working)
    {
      w
    } else {
      false
    };

    if working && self.store_used_capacity(Some(ResourceType::Energy)) == 0 {
      self.memory().set_value(Values::Working(false));
      false
    } else if !working
      && self.store_free_capacity(Some(ResourceType::Energy)) == 0
    {
      self.memory().set_value(Values::Working(true));
      true
    } else {
      working
    }
  }

  fn travel_or_report<T: RoomObjectProperties + HasId>(
    &self,
    code: ReturnCode,
    target: &T,
  ) -> ReturnCode {
    use ReturnCode::*;
    self.memory().set_value(Values::TargetId(target.id().to_string()));
    if code == NotInRange {
      return self.move_to(target);
    } else if code != Ok {
      let msg = format!("{} is having trouble: {:?}", self.name(), code);
      self.say(&msg, false);
      error!("{}", &msg);
    }
    code
  }

  fn go_attack<T: Attackable + HasId>(&self, target: &T) -> ReturnCode {
    self.memory().set_value(Values::Action(Actions::Attack));
    let code = self.attack(target);
    return self.travel_or_report(code, target);
  }

  fn go_attack_structure(&self, target: &Structure) -> ReturnCode {
    let attack = if let Some(target) = target.as_attackable() {
      target
    } else {
      return self.reset_action();
    };
    self.memory().set_value(Values::Action(Actions::Attack));
    return self.travel_or_report(self.attack(attack), target);
  }

  fn go_attack_controller(&self, target: &StructureController) -> ReturnCode {
    if target.my() || !target.has_owner() {
      return self.reset_action();
    }
    self.memory().set_value(Values::Action(Actions::AttackController));
    let code = self.attack_controller(target);
    self.travel_or_report(code, target)
  }

  fn go_build(&self, target: &ConstructionSite) -> ReturnCode {
    self.memory().set_value(Values::Action(Actions::Build));
    let code = self.build(target);
    self.travel_or_report(code, target)
  }

  fn go_claim_controller(&self, target: &StructureController) -> ReturnCode {
    if target.my() {
      return self.reset_action();
    }
    self.memory().set_value(Values::Action(Actions::ClaimController));
    self.travel_or_report(self.claim_controller(target), target)
  }

  fn go_dismantle(&self, target: &Structure) -> ReturnCode {
    self.memory().set_value(Values::Action(Actions::Dismantle));
    self.travel_or_report(self.dismantle(target), target)
  }

  fn go_generate_safe_mode(&self, target: &StructureController) -> ReturnCode {
    if !target.my() {
      return self.reset_action();
    }

    self.memory().set_value(Values::Action(Actions::GenerateSafeMode));
    self.travel_or_report(self.generate_safe_mode(target), target)
  }

  fn go_harvest<T: Harvestable + HasId>(&self, target: &T) -> ReturnCode {
    if self.get_active_bodyparts(Part::Carry) > 0
      && self.store_free_capacity(None) == 0
    {
      return self.reset_action();
    }
    self.memory().set_value(Values::Action(Actions::Harvest));
    self.travel_or_report(self.harvest(target), target)
  }

  fn go_heal_creep(&self, target: &Creep) -> ReturnCode {
    if target.hits() == target.hits_max() {
      return self.reset_action();
    }
    self.memory().set_value(Values::Action(Actions::Heal));
    self.travel_or_report(self.heal(target), target)
  }

  fn go_heal_power_creep(&self, target: &PowerCreep) -> ReturnCode {
    if target.hits() == target.hits_max() {
      return self.reset_action();
    }
    self.memory().set_value(Values::Action(Actions::Heal));
    self.travel_or_report(self.heal(target), target)
  }

  fn go_pickup(&self, target: &Resource) -> ReturnCode {
    if self.store_free_capacity(Some(target.resource_type())) == 0 {
      return self.reset_action();
    }
    self.memory().set_value(Values::Action(Actions::Pickup));
    self.travel_or_report(self.pickup(target), target)
  }

  fn go_pull(&self, target: &Creep) -> ReturnCode {
    if target.get_active_bodyparts(Part::Move) > 0
      || self.get_active_bodyparts(Part::Move) == 0
    {
      return self.reset_action();
    }
    if let Some(Values::TargetId(id_str)) =
      self.memory().get_value(Keys::TargetId)
    {
      if let Ok(target_id) = RawObjectId::from_str(&id_str) {
        if let Some(final_target) = game::get_object_erased(target_id) {
          // check if the job is done
          if target.pos().is_near_to(&final_target) {
            return self.reset_action();
          }
          self.memory().set_value(Values::Action(Actions::Pull));
          self.memory().set_value(Values::TargetId(target_id.to_string()));
          let code = self.pull(target);
          return if self.pos().is_near_to(target) {
            target.move_to(self);
            self.move_to(&final_target);
            code
          } else {
            self.travel_or_report(code, target)
          };
        }
      }
    }
    return self.reset_action();
  }

  fn go_repair(&self, target: &Structure) -> ReturnCode {
    if let Some(t) = target.as_attackable() {
      if t.hits() == t.hits_max() {
        return self.reset_action();
      }
      self.memory().set_value(Values::Action(Actions::Repair));
      return self.travel_or_report(self.repair(target), target);
    }
    self.reset_action()
  }

  fn go_reserve_controller(&self, target: &StructureController) -> ReturnCode {
    if target.my() {
      return self.reset_action();
    }
    if let Some(reservation) = target.reservation() {
      if reservation.username == self.owner_name() {
        return self.reset_action();
      }
    }

    self.memory().set_value(Values::Action(Actions::ReserveController));
    self.travel_or_report(self.reserve_controller(target), target)
  }

  fn go_sign_controller(&self, target: &StructureController) -> ReturnCode {
    if let Some(sign) = target.sign() {
      if sign.username == self.owner_name() {
        return self.reset_action();
      }
    }
    self.memory().set_value(Values::Action(Actions::SignController));
    self.travel_or_report(
      self.sign_controller(target, "Claimed by Grillbrick"),
      target,
    )
  }

  fn go_upgrade_controller(&self, target: &StructureController) -> ReturnCode {
    if !target.my() {
      return self.reset_action();
    }
    self.memory().set_value(Values::Action(Actions::UpgradeController));
    self.travel_or_report(self.upgrade_controller(target), target)
  }

  fn go_transfer_to_structure(
    &self,
    target: &Structure,
    resource: ResourceType,
    amount: Option<u32>,
  ) -> ReturnCode {
    let transferable = if let Some(target) = target.as_transferable() {
      target
    } else {
      return self.reset_action();
    };
    let store = if let Some(target) = target.as_has_store() {
      target
    } else {
      return self.reset_action();
    };
    if self.store_used_capacity(Some(resource)) == 0
      || store.store_free_capacity(Some(resource)) == 0
    {
      return self.reset_action();
    }
    self.memory().set_value(Values::Action(Actions::Transfer));
    self.memory().set_value(Values::Resource(resource));
    let code = if let Some(amount) = amount {
      self.transfer_amount(transferable, resource, amount)
    } else {
      self.transfer_all(transferable, resource)
    };
    self.travel_or_report(code, target)
  }

  fn go_transfer<T: Transferable + HasStore + RoomObjectProperties + HasId>(
    &self,
    target: &T,
    resource: ResourceType,
    amount: Option<u32>,
  ) -> ReturnCode {
    if self.store_used_capacity(Some(resource)) == 0
      || target.store_free_capacity(Some(resource)) == 0
    {
      return self.reset_action();
    }
    self.memory().set_value(Values::Action(Actions::Transfer));
    self.memory().set_value(Values::Resource(resource));
    let code = if let Some(amount) = amount {
      self.transfer_amount(target, resource, amount)
    } else {
      self.transfer_all(target, resource)
    };
    self.travel_or_report(code, target)
  }

  fn go_withdraw_from_structure(
    &self,
    target: &Structure,
    resource: ResourceType,
    amount: Option<u32>,
  ) -> ReturnCode {
    let withdrawable = if let Some(target) = target.as_withdrawable() {
      target
    } else {
      return self.reset_action();
    };
    let store = if let Some(target) = target.as_has_store() {
      target
    } else {
      return self.reset_action();
    };
    if self.store_free_capacity(Some(resource)) == 0
      || store.store_used_capacity(Some(resource)) == 0
    {
      return self.reset_action();
    }
    self.memory().set_value(Values::Action(Actions::Withdraw));
    self.memory().set_value(Values::Resource(resource));
    let code = if let Some(amount) = amount {
      self.withdraw_amount(withdrawable, resource, amount)
    } else {
      self.withdraw_all(withdrawable, resource)
    };
    self.travel_or_report(code, target)
  }

  fn go_withdraw<T: Withdrawable + HasStore + RoomObjectProperties + HasId>(
    &self,
    target: &T,
    resource: ResourceType,
    amount: Option<u32>,
  ) -> ReturnCode {
    if self.store_free_capacity(Some(resource)) == 0
      || target.store_used_capacity(Some(resource)) == 0
    {
      return self.reset_action();
    }
    self.memory().set_value(Values::Action(Actions::Withdraw));
    self.memory().set_value(Values::Resource(resource));
    let code = if let Some(amount) = amount {
      self.withdraw_amount(target, resource, amount)
    } else {
      self.withdraw_all(target, resource)
    };
    self.travel_or_report(code, target)
  }
}

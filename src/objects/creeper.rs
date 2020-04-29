use crate::*;
use std::ops::Deref;

/// CreepActions gives Creeps superpowers.
pub struct Creeper {
  creep: Creep,
  /// The finder used for finding targets
  pub pos: Finder,
}

impl Display for Creeper {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    write!(f, "{}", self.creep.name())
  }
}
impl Deref for Creeper {
  type Target = Creep;

  fn deref(&self) -> &Self::Target {
    &self.creep
  }
}

impl Creeper {
  /// Create a new Creeper from a Creep
  pub fn new(creep: Creep) -> Creeper {
    let pos = Finder::new(&creep);
    Creeper { creep, pos }
  }

  /// Reset the creep's action
  pub fn reset_action(&self) -> ReturnCode {
    self.memory().del("_move");
    self.memory().rm_value(Keys::TargetId);
    self.memory().rm_value(Keys::Resource);
    self.memory().rm_value(Keys::Action);

    ReturnCode::InvalidTarget
  }

  /// Is this creep working?
  pub fn working(&self) -> bool {
    let working = self.memory().bool(&Keys::Working.to_string());

    // special case for lorries
    if let Some(Values::Role(Role::Lorry)) = self.memory().get_value(Keys::Role)
    {
      if working && self.store_used_capacity(None) == 0 {
        self.memory().set_value(Values::Working(false));
        false
      } else if !working && self.store_free_capacity(None) == 0 {
        self.memory().set_value(Values::Working(true));
        true
      } else {
        working
      }
    } else if working
      && self.store_used_capacity(Some(ResourceType::Energy)) == 0
    {
      self.memory().set_value(Values::Working(false));
      false
    } else if !working
      && self.store_free_capacity(Some(ResourceType::Energy)) == 0
    {
      self.memory().set_value(Values::Working(true));
      true
    } else {
      if working {
        trace!("{} is currently working", self.name());
      } else {
        trace!("{} is not working", self.name())
      }
      working
    }
  }

  /// This quickly gets the creeps role
  pub fn role(&self) -> Role {
    if let Some(Values::Role(r)) = self.memory().get_value(Keys::Role) {
      r
    } else {
      warn!("Creep found with no role! {}", self.name());

      use Part::*;
      let role =
        if self.get_active_bodyparts(Move) == 1 && self.body().len() == 1 {
          trace!("{} only has a move part assigning scout", self.name());
          Role::Scout
        } else if self.get_active_bodyparts(Carry) == 0
          && self.get_active_bodyparts(Work) > 0
        {
          trace!(
            "{} has no carry part but has work part assigning Miner",
            self.name()
          );
          Role::Miner
        } else if self.get_active_bodyparts(Work) == 0
          && self.get_active_bodyparts(Carry) > 0
        {
          trace!(
            "{} has no Work part but can carry assigning Lorry",
            self.name()
          );
          Role::Lorry
        } else if self.get_active_bodyparts(Work) > 0
          && self.get_active_bodyparts(Carry) > 0
        {
          trace!("{} has Work and Carry - Assigning Upgrader", self.name());
          Role::Upgrader
        } else if self.get_active_bodyparts(Heal) > 0 {
          trace!("{} has a heal part - Assigning Healer", self.name());
          Role::Healer
        } else if self.get_active_bodyparts(Attack) > 0
          || self.get_active_bodyparts(RangedAttack) > 0
        {
          trace!("{} has attack parts - Assigning Soldier", self.name());
          Role::Soldier
        } else if self.get_active_bodyparts(Claim) > 0 {
          trace!("{} has a claim part - Assigning Reserver", self.name());
          Role::Reserver
        } else {
          // Assign role based on first body part
          trace!("{} only has a move part assigning scout", self.name());
          match self.body()[0].part {
            Move => Role::Scout,
            Work => Role::Miner,
            Carry => Role::Lorry,
            Attack => Role::Soldier,
            RangedAttack => Role::Soldier,
            Tough => Role::Scout,
            Heal => Role::Healer,
            Claim => Role::Reserver,
          }
        };

      self.memory().set_value(Values::Role(role));
      role
    }
  }

  /// This is a quicker way to get TargetId
  pub fn target_id(&self) -> Option<String> {
    if let Some(Values::TargetId(id)) = self.memory().get_value(Keys::TargetId)
    {
      Some(id)
    } else {
      None
    }
  }

  /// This is a quickery way to get the target room
  pub fn target_room(&self) -> Option<RoomName> {
    if let Some(Values::TargetRoom(id)) =
      self.memory().get_value(Keys::TargetRoom)
    {
      Some(id)
    } else {
      None
    }
  }

  /// Get the Home Room
  pub fn home_room(&self) -> RoomName {
    if let Some(Values::HomeRoom(name)) =
      self.memory().get_value(Keys::HomeRoom)
    {
      name
    } else {
      let room = self.pos().room_name();
      self.memory().set_value(Values::HomeRoom(room));
      room
    }
  }

  /// This gets the resource that the creep is working in
  pub fn resource(&self) -> Option<ResourceType> {
    if let Some(Values::Resource(resource)) =
      self.memory().get_value(Keys::Resource)
    {
      Some(resource)
    } else {
      None
    }
  }

  /// This gets the assigned action more quickly
  pub fn action(&self) -> Option<Actions> {
    if let Some(Values::Action(action)) = self.memory().get_value(Keys::Action)
    {
      Some(action)
    } else {
      None
    }
  }

  /// TODO Make this more like Traveler
  pub fn travel_to<T: RoomObjectProperties + HasId>(
    &self,
    target: &T,
  ) -> ReturnCode {
    self.memory().set_value(Values::TargetId(target.id().to_string()));
    self.pos().create_construction_site(StructureType::Road);
    self.move_to(target)
  }

  /// Travel to or report on errors
  pub fn travel_or_report<T: RoomObjectProperties + HasId>(
    &self,
    code: ReturnCode,
    target: &T,
  ) -> ReturnCode {
    use ReturnCode::*;
    if code == NotInRange {
      return self.travel_to(target);
    } else if code != Ok && code != Tired {
      let msg = format!("{} is having trouble: {:?}", self.name(), code);
      error!("{}", &msg);
      self.say("Help me!", false);
      self.reset_action();
    }
    code
  }

  /// Go attack
  pub fn go_attack<T: Attackable + HasId>(&self, target: &T) -> ReturnCode {
    self.memory().set_value(Values::Action(Actions::Attack));
    let code = self.attack(target);
    return self.travel_or_report(code, target);
  }

  /// Go attack Structure
  pub fn go_attack_structure(&self, target: &Structure) -> ReturnCode {
    let attack = if let Some(target) = target.as_attackable() {
      target
    } else {
      return self.reset_action();
    };
    self.memory().set_value(Values::Action(Actions::Attack));
    return self.travel_or_report(self.attack(attack), target);
  }

  /// Go attack controller
  pub fn go_attack_controller(
    &self,
    target: &StructureController,
  ) -> ReturnCode {
    if target.my() || !target.has_owner() {
      return self.reset_action();
    }
    self.memory().set_value(Values::Action(Actions::AttackController));
    let code = self.attack_controller(target);
    self.travel_or_report(code, target)
  }

  /// Go build a construction site
  pub fn go_build(&self, target: &ConstructionSite) -> ReturnCode {
    self.memory().set_value(Values::Action(Actions::Build));
    if self.store_used_capacity(Some(Energy)) == 0 {
      return self.reset_action();
    }
    let code = self.build(target);
    self.travel_or_report(code, target)
  }

  /// Cleanup claim target
  pub fn cleanup_claim(&self) -> ReturnCode {
    if let Some(flag) = game::flags::get("claim") {
      // remove the flag if necessary
      flag.remove();
    }
    // cleanup the memory
    root().rm_value(Keys::Claim);
    ReturnCode::Ok
  }

  /// Go to room from RoomName
  pub fn go_to_room(&self, name: RoomName) -> ReturnCode {
    let room = self.room().unwrap();
    if let Ok(exit) = game::map::find_exit(room.name(), name) {
      let t = room.find(find::Exit::from(exit));
      return self.move_to(&t[0]);
    } else {
      // invalid exit?
      ReturnCode::InvalidArgs
    }
  }

  /// Go claim a controller
  pub fn go_claim_controller(
    &self,
    target: &StructureController,
  ) -> ReturnCode {
    if target.my() {
      return self.reset_action();
    }
    self.memory().set_value(Values::Action(Actions::ClaimController));
    let code = self.claim_controller(target);
    if code == ReturnCode::Ok {
      self.cleanup_claim();
    }
    self.travel_or_report(code, target)
  }

  /// go dismantle a target
  pub fn go_dismantle(&self, target: &Structure) -> ReturnCode {
    self.memory().set_value(Values::Action(Actions::Dismantle));
    self.travel_or_report(self.dismantle(target), target)
  }

  /// go generate a safe mode
  pub fn go_generate_safe_mode(
    &self,
    target: &StructureController,
  ) -> ReturnCode {
    if !target.my() {
      return self.reset_action();
    }

    self.memory().set_value(Values::Action(Actions::GenerateSafeMode));
    self.travel_or_report(self.generate_safe_mode(target), target)
  }

  /// go harvest a source/deposit/mineral
  pub fn go_harvest<T: Harvestable + HasId>(&self, target: &T) -> ReturnCode {
    if self.get_active_bodyparts(Part::Carry) > 0
      && self.store_free_capacity(None) == 0
    {
      return self.reset_action();
    }
    self.memory().set_value(Values::Action(Actions::Harvest));
    self.travel_or_report(self.harvest(target), target)
  }

  /// go heal a creep
  pub fn go_heal<T: Attackable + SharedCreepProperties + HasId>(
    &self,
    target: &T,
  ) -> ReturnCode {
    if target.hits() == target.hits_max() {
      return self.reset_action();
    }
    self.memory().set_value(Values::Action(Actions::Heal));
    self.travel_or_report(self.heal(target), target)
  }

  /// Go pickup a dropped resource
  pub fn go_pickup(&self, target: &Resource) -> ReturnCode {
    if self.store_free_capacity(Some(target.resource_type())) == 0 {
      return self.reset_action();
    }
    self.memory().set_value(Values::Action(Actions::Pickup));
    self.travel_or_report(self.pickup(target), target)
  }

  /// Go pull on a creep
  pub fn go_pull(&self, target: &Creep) -> ReturnCode {
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
            target.move_to(&self.creep);
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

  /// Go repair a structure
  pub fn go_repair(&self, target: &Structure) -> ReturnCode {
    if let Some(t) = target.as_attackable() {
      if t.hits() == t.hits_max() {
        return self.reset_action();
      }
      self.memory().set_value(Values::Action(Actions::Repair));
      return self.travel_or_report(self.repair(target), target);
    }
    self.reset_action()
  }

  /// Go reserve a controller
  pub fn go_reserve_controller(&self, target: RoomName) -> ReturnCode {
    if self.pos().room_name() == target {
      // in the right room
      if let Some(target) = self.room().unwrap().controller() {
        self.memory().set_value(Values::Action(Actions::ReserveController));
        self.travel_or_report(self.reserve_controller(&target), &target)
      } else {
        // room has no controller reset flag
        if let Some(flag) = game::flags::get("reserve") {
          flag.remove();
        }
        // get home_room
        let home_room = if let Some(Values::HomeRoom(hr)) =
          self.memory().get_value(Keys::HomeRoom)
        {
          hr
        } else {
          panic!()
        };
        self.go_to_room(home_room);
        self.reset_action()
      }
    } else {
      // not in the right room
      self.go_to_room(target)
    }
  }

  /// Go sign a controller
  pub fn go_sign_controller(&self, target: &StructureController) -> ReturnCode {
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

  /// Go upgrade a controller
  pub fn go_upgrade_controller(
    &self,
    target: &StructureController,
  ) -> ReturnCode {
    if !target.my() || self.store_used_capacity(Some(Energy)) == 0 {
      return self.reset_action();
    }
    self.memory().set_value(Values::Action(Actions::UpgradeController));
    self.travel_or_report(self.upgrade_controller(target), target)
  }

  /// Go transfer to a structure
  pub fn go_transfer_to_structure(
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

  /// Go transfer to a transferable target
  pub fn go_transfer<
    T: Transferable + HasStore + RoomObjectProperties + HasId,
  >(
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

  /// Go withdraw from a structure
  pub fn go_withdraw_from_structure(
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

  /// Go withdraw
  pub fn go_withdraw<
    T: Withdrawable + HasStore + RoomObjectProperties + HasId,
  >(
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

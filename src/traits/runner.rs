use crate::*;

/// This trait allows things to run!
pub trait Runner {
  /// Run the game object so it does what it is supposed to.
  fn run(&self) -> ReturnCode;
}

impl Runner for Creep {
  fn run(&self) -> ReturnCode {
    if self.spawning() {
      return ReturnCode::Busy;
    }

    if let (Some(Values::Action(action)), Some(Values::TargetId(target))) = (
      self.memory().get_value(Keys::Action),
      self.memory().get_value(Keys::TargetId),
    ) {
      return run_creep_action(self, action, &target);
    }

    // time_hack(format!("Running creep: {}", creep.creep.name()).as_str());
    if let Some(Values::Role(role)) = self.memory().get_value(Keys::Role) {
      return role.run(self);
    }

    // INVALID ROLE!
    use Part::*;
    let role = if self.get_active_bodyparts(Move) == 1 && self.body().len() == 1
    {
      Role::Scout
    } else if self.get_active_bodyparts(Carry) == 0 {
      Role::Miner
    } else if self.get_active_bodyparts(Work) == 0 {
      Role::Lorry
    } else if self.get_active_bodyparts(Work) > 0
      && self.get_active_bodyparts(Carry) > 0
    {
      Role::Upgrader
    } else if self.get_active_bodyparts(Attack) > 0
      || self.get_active_bodyparts(RangedAttack) > 0
    {
      Role::Soldier
    } else if self.get_active_bodyparts(Heal) > 0 {
      Role::Healer
    } else if self.get_active_bodyparts(Claim) > 0 {
      Role::Claimer
    } else {
      // Assign role based on first body part
      match self.body()[0].part {
        Move => Role::Scout,
        Work => Role::Miner,
        Carry => Role::Lorry,
        Attack => Role::Soldier,
        RangedAttack => Role::Soldier,
        Tough => Role::Scout,
        Heal => Role::Healer,
        Claim => Role::Claimer,
      }
    };
    self.memory().set_value(Values::Role(role));

    role.run(self)
  }
}

fn run_creep_action(
  creep: &Creep,
  action: Actions,
  target: &str,
) -> ReturnCode {
  // use ReturnCode::*;
  let target = String::from(target);

  match action {
    Actions::Attack => {
      if let Some(creep) = target.as_creep() {
        creep.go_attack(&creep)
      } else if let Some(target) = target.as_power_creep() {
        creep.go_attack(&target)
      } else if let Some(target) = target.as_structure() {
        creep.go_attack_structure(&target)
      } else {
        creep.reset_action()
      }
    }
    Actions::AttackController => {
      if let Some(Structure::Controller(target)) = target.as_structure() {
        creep.go_attack_controller(&target)
      } else {
        creep.reset_action()
      }
    }
    Actions::Build => {
      if let Some(target) = target.as_construction_site() {
        creep.go_build(&target)
      } else {
        creep.reset_action()
      }
    }
    Actions::ClaimController => {
      if let Some(Structure::Controller(target)) = target.as_structure() {
        creep.go_claim_controller(&target)
      } else {
        creep.reset_action()
      }
    }
    Actions::Dismantle => {
      if let Some(target) = target.as_structure() {
        creep.go_dismantle(&target)
      } else {
        creep.reset_action()
      }
    }
    Actions::GenerateSafeMode => {
      if let Some(Structure::Controller(target)) = target.as_structure() {
        creep.go_generate_safe_mode(&target)
      } else {
        creep.reset_action()
      }
    }
    Actions::Harvest => {
      if let Some(target) = target.as_source() {
        creep.go_harvest(&target)
      } else if let Some(target) = target.as_mineral() {
        creep.go_harvest(&target)
      } else if let Some(target) = target.as_deposit() {
        creep.go_harvest(&target)
      } else {
        creep.reset_action()
      }
    }
    Actions::Heal => {
      if let Some(target) = target.as_creep() {
        creep.go_heal_creep(&target)
      } else if let Some(target) = target.as_power_creep() {
        creep.go_heal_power_creep(&target)
      } else {
        creep.reset_action()
      }
    }
    Actions::Pickup => {
      if let Some(target) = target.as_resource() {
        creep.go_pickup(&target)
      } else {
        creep.reset_action()
      }
    }
    Actions::Pull => {
      if let Some(target) = target.as_creep() {
        creep.go_pull(&target)
      } else {
        creep.reset_action()
      }
    }
    Actions::Repair => {
      if let Some(target) = target.as_structure() {
        creep.go_repair(&target)
      } else {
        creep.reset_action()
      }
    }
    Actions::ReserveController => {
      if let Some(Structure::Controller(target)) = target.as_structure() {
        creep.go_reserve_controller(&target)
      } else {
        creep.reset_action()
      }
    }
    Actions::SignController => {
      if let Some(Structure::Controller(target)) = target.as_structure() {
        creep.go_sign_controller(&target)
      } else {
        creep.reset_action()
      }
    }
    Actions::UpgradeController => {
      if let Some(Structure::Controller(target)) = target.as_structure() {
        creep.go_upgrade_controller(&target)
      } else {
        creep.reset_action()
      }
    }
    Actions::Transfer => {
      if let Some(Values::Resource(resource)) =
        creep.memory().get_value(Keys::Resource)
      {
        if let Some(target) = target.as_structure() {
          return creep.go_transfer(&target, resource, None);
        }
      }
      creep.reset_action()
    }
    Actions::Withdraw => {
      if let Some(Values::Resource(resource)) =
        creep.memory().get_value(Keys::Resource)
      {
        if let Some(target) = target.as_structure() {
          return creep.go_withdraw(&target, resource, None);
        }
      }
      creep.reset_action()
    }
  }
}

impl Runner for Structure {
  fn run(&self) -> ReturnCode {
    match self {
      Structure::Container(_) => ReturnCode::Ok,
      Structure::Controller(_) => ReturnCode::Ok,
      Structure::Extension(_) => ReturnCode::Ok,
      Structure::Extractor(_) => ReturnCode::Ok,
      Structure::Factory(_) => ReturnCode::Ok,
      Structure::InvaderCore(_) => ReturnCode::Ok,
      Structure::KeeperLair(_) => ReturnCode::Ok,
      Structure::Lab(_) => ReturnCode::Ok,
      Structure::Link(link) => {
        if link.store_free_capacity(None) > 0 {
          // Don't worry about links that aren't full
          return ReturnCode::Ok;
        }

        let room = link.room().expect("All links should have a room");

        let mut others: Vec<StructureLink> = room
          .find(find::STRUCTURES)
          .into_iter()
          .filter_map(|s| {
            if s.id().to_string() == link.id().to_string() {
              None
            } else {
              if let Structure::Link(l) = s {
                Some(l)
              } else {
                None
              }
            }
          })
          .collect();

        if others.len() == 0 {
          return ReturnCode::Ok;
        }
        let mut target = others.pop().unwrap();

        while !others.is_empty() {
          let next = others.pop().unwrap();
          if next.store_used_capacity(None) < target.store_used_capacity(None) {
            target = next;
          }
        }
        let amount = link.store_used_capacity(None) / 2;
        link.transfer_energy(&target, Some(amount))
      }
      Structure::Nuker(_) => ReturnCode::Ok,
      Structure::Observer(_) => ReturnCode::Ok,
      Structure::PowerBank(_) => ReturnCode::Ok,
      Structure::PowerSpawn(_) => ReturnCode::Ok,
      Structure::Portal(_) => ReturnCode::Ok,
      Structure::Rampart(_) => ReturnCode::Ok,
      Structure::Road(_) => ReturnCode::Ok,
      Structure::Spawn(spawn) => {
        use ReturnCode::*;
        if spawn.is_spawning() {
          return Busy;
        }
        if Role::spawn_emergencies(spawn) {
          Ok
        } else if Role::spawn_min(spawn) {
          Ok
        } else {
          Role::spawn_extras(spawn)
        }
      }
      Structure::Storage(_) => ReturnCode::Ok,
      Structure::Terminal(_) => todo!("Run Terminal here"),
      Structure::Tower(tower) => {
        if let Some(target) =
          tower.pos().find_closest_by_range(find::HOSTILE_CREEPS)
        {
          tower.attack(&target)
        } else if let Some(target) =
          tower.pos().find_closest_by_range(find::HOSTILE_POWER_CREEPS)
        {
          tower.attack(&target)
        } else if let Some(target) =
          tower.pos().find_closest_by_range(find::HOSTILE_STRUCTURES)
        {
          tower.attack(&target)
        } else {
          for range in 0..79 {
            for creep in
              tower.pos().find_in_range(find::MY_CREEPS, range) as Vec<Creep>
            {
              if creep.hits() < creep.hits_max() {
                return tower.heal(&creep);
              }
            }
          }
          ReturnCode::Ok
        }
      }
      Structure::Wall(_) => ReturnCode::Ok,
    }
  }
}

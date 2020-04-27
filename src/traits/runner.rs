use crate::*;

/// This trait allows things to run!
pub trait Runner {
  /// Run the game object so it does what it is supposed to.
  fn run(&self) -> ReturnCode;
}

impl Runner for Creeper {
  fn run(&self) -> ReturnCode {
    trace!("Running creep: {}", self.name());
    if self.spawning() {
      trace!("{} is busy", self.name());
      return ReturnCode::Busy;
    }

    if let (Some(action), Some(target)) = (self.action(), self.target_id()) {
      trace!("{} has an action already assigned: {}", self.name(), action);
      return run_creep_action(self, action, &target);
    }

    // time_hack(format!("Running creep: {}", creep.creep.name()).as_str());
    let role = self.role();
    trace!("{} is a {}: running role", self.name(), role);
    role.run(self)
  }
}

fn run_creep_action(
  creep: &Creeper,
  action: Actions,
  target: &str,
) -> ReturnCode {
  // use ReturnCode::*;
  let target = String::from(target);
  trace!("Running preexisting action on target with id {}", &target);

  match action {
    Actions::Attack => {
      if let Some(target) = target.as_creep() {
        creep.go_attack(&target)
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
        // check for container
        if let Some(c) = target.container() {
          if creep.pos() == c.pos() {
            creep.go_harvest(&target)
          } else {
            creep.move_to(&c)
          }
        } else {
          creep.go_harvest(&target)
        }
      } else if let Some(target) = target.as_mineral() {
        if let Some(t) = target.extractor() {
          if t.cooldown() == 0 {
            if let Some(c) = t.container() {
              if creep.pos() == c.pos() {
                creep.go_harvest(&target)
              } else {
                creep.move_to(&c)
              }
            } else {
              creep.go_harvest(&target)
            }
          } else {
            ReturnCode::Busy
          }
        } else {
          creep.reset_action()
        }
      } else if let Some(target) = target.as_deposit() {
        if let Some(c) = target.container() {
          if creep.pos() == c.pos() {
            creep.go_harvest(&target)
          } else {
            creep.move_to(&c)
          }
        } else {
          creep.go_harvest(&target)
        }
      } else {
        creep.reset_action()
      }
    }
    Actions::Heal => {
      if let Some(target) = target.as_creep() {
        creep.go_heal(&target)
      } else if let Some(target) = target.as_power_creep() {
        creep.go_heal(&target)
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
        creep.go_reserve_controller(target.pos().room_name())
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
      if let Some(resource) = creep.resource() {
        if let Some(t) = target.as_creep() {
          return creep.go_transfer(&t, resource, None);
        }
        if let Some(target) = target.as_structure() {
          return creep.go_transfer_to_structure(&target, resource, None);
        }
      }
      creep.reset_action()
    }
    Actions::Withdraw => {
      if let Some(resource) = creep.resource() {
        {
          if let Some(t) = target.as_tombstone() {
            return creep.go_withdraw(&t, resource, None);
          }
          if let Some(t) = target.as_ruin() {
            return creep.go_withdraw(&t, resource, None);
          }
          if let Some(t) = target.as_structure() {
            return creep.go_withdraw_from_structure(&t, resource, None);
          }
        }
        creep.reset_action()
      } else {
        creep.reset_action()
      }
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
        if link.store_free_capacity(Some(Energy)) > 0 {
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
        trace!("Running Spawn {}", spawn.name());
        if spawn.is_spawning() {
          trace!("{} is busy spawning", spawn.name());
          return Busy;
        }
        if Role::spawn_emergencies(spawn) {
          trace!("{} spawning an emergency spawn", spawn.name());
          Ok
        } else if Role::spawn_min(spawn) {
          Ok
        } else {
          Role::spawn_extras(spawn)
        }
      }
      Structure::Storage(_) => ReturnCode::Ok,
      Structure::Terminal(t) => {
        if t.cooldown() > 0 {
          return ReturnCode::Busy;
        }
        let room_name = t.room().unwrap().name();

        let resources = t.store_types();
        for r in resources {
          if r == Energy {
            continue;
          }
          if t.store_used_capacity(Some(r)) == 0 {
            warn!("Skipping empty store in terminal of room: {}", room_name);
            continue;
          }

          let amount = t.store_used_capacity(Some(r));
          let mut orders: Vec<Order> = game::market::get_all_orders()
            .into_iter()
            .filter(|s| {
              s.order_type == OrderType::Buy
                && s.resource_type == MarketResourceType::Resource(r)
            })
            .collect();

          if orders.len() == 0 {
            // there are no orders for this resource
            continue;
          }
          let mut best = orders.pop().unwrap();
          while orders.len() > 0 {
            let order = orders.pop().unwrap();
            // calculate the cost of the order
            if let (Some(order_room), Some(best_room)) =
              (order.room_name, best.room_name)
            {
              let order_cost = game::market::calc_transaction_cost(
                amount, room_name, order_room,
              );
              let best_cost = game::market::calc_transaction_cost(
                amount, room_name, best_room,
              );
              if order.price > best.price || order_cost < best_cost {
                best = order;
              }
            }
          }

          // all orders are in now deal with the best
          return game::market::deal(&best.id, amount, Some(room_name));
        }

        ReturnCode::Ok
      }
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

impl Runner for Flag {
  fn run(&self) -> ReturnCode {
    trace!("Flag found with name: {}", self.name());

    match self.name().as_str() {
      "claim" => {
        for creep in game::creeps::values() {
          let creep = Creeper::new(creep);
          let room_name = self.pos().room_name();
          if creep.role() == Role::Claimer {
            creep.memory().set_value(Values::TargetRoom(room_name));
          }
          if creep.role() == Role::RemoteBuilder {
            creep.memory().set_value(Values::TargetRoom(room_name));
          }
        }
      }
      "remote" => {
        for creep in game::creeps::values() {
          let room_name = self.pos().room_name();
          if let Some(Values::Role(Role::RemoteHarvester)) =
            creep.memory().get_value(Keys::Role)
          {
            creep.memory().set_value(Values::TargetRoom(room_name));
          }
        }
      }
      "attack" => {
        for creep in game::creeps::values() {
          let room_name = self.pos().room_name();
          if let Some(Values::Role(Role::Soldier)) =
            creep.memory().get_value(Keys::Role)
          {
            root().set_value(Values::TargetRoom(room_name));
            creep.memory().set_value(Values::TargetRoom(room_name));
          }
        }
      }
      "reserve" => {
        for creep in game::creeps::values() {
          let room_name = self.pos().room_name();
          if let Some(Values::Role(Role::Reserver)) =
            creep.memory().get_value(Keys::Role)
          {
            root().set_value(Values::TargetRoom(room_name));
            creep.memory().set_value(Values::TargetRoom(room_name));
          }
        }
      }
      _ => (),
    }

    ReturnCode::Ok
  }
}

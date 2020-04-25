//! The role is the role that a creep will take.
use crate::Role::Harvester;
use crate::*;
use screeps::ResourceType::Energy;
use screeps::StructureType;
use std::collections::BTreeMap;

/// This is an enum that lists the different roles
#[derive(
  Serialize, Deserialize, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash,
)]
pub enum Role {
  /// Harvest energy and place it into Extensions, Spawns, Towers, Storage
  /// fallback: -> Upgrader
  Harvester,
  /// Mine from source and drop on the ground on into a container.
  Miner,
  /// Upgrade the room controller
  Upgrader,
  /// Builds anything it finds except walls or ramparts
  /// fallback: -> Repair -> Upgrader
  Builder,
  /// Repairs anything damaged except walls or ramparts
  /// fallback: -> Upgrader
  Repairer,
  /// Builds walls and ramparts
  /// Repairs the most damaged wall or rampart
  /// fallback: -> Upgrader
  WallRepairer,
  /// Ferries resources from containers or the ground and places it in
  /// Extensions, Spawns, Towers, or Storage
  /// fallback: -> Repair -> Upgrader
  Lorry,
  /// Ferries resources to links
  /// fallback: -> Repair -> Upgrader
  LinkLoader,
  /// This is a claimer to claim new rooms
  Claimer,
  /// A reserve new rooms
  Reserver,
  /// A scout
  Scout,
  /// A soldier
  Soldier,
  /// A healer
  Healer,
}

/// This gives me to_string functionality for easy debugging
impl Display for Role {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    write!(f, "{}", to_string(self).unwrap())
  }
}

/// General helper methods
impl Role {
  /// Returns the appropriate body for this role as well as if it should be expanded.
  pub fn body(&self) -> Vec<Part> {
    use Part::*;
    match self {
      Role::Harvester => vec![Work, Carry, Move, Move],
      Role::Miner => vec![Move, Work, Work, Work, Work, Work],
      Role::Upgrader => vec![Work, Carry, Move, Move],
      Role::Builder => vec![Work, Carry, Move, Move],
      Role::Repairer => vec![Work, Carry, Move, Move],
      Role::WallRepairer => vec![Work, Carry, Move, Move],
      Role::Lorry => vec![Carry, Move],
      Role::LinkLoader => vec![Carry, Carry, Move, Move],
      Role::Claimer => vec![Move, Move, Move, Move, Move, Claim],
      Role::Reserver => vec![Move, Move, Move, Move, Move, Claim],
      Role::Scout => vec![Move],
      Role::Soldier => vec![Attack, RangedAttack, Move, Move, Move],
      Role::Healer => vec![RangedAttack, Heal, Move, Move],
    }
  }

  /// Returns the minimum cost to spawn this creep
  pub fn cost(&self) -> u32 {
    let body = self.body();
    cost(&body)
  }

  /// Returns the creeps body expanded to the appropriate amount of energy
  pub fn expand(&self, energy: u32) -> Vec<Part> {
    let base = self.body();
    use Role::*;

    match self {
      // exceptions for roles that don't expand
      Miner => {
        let mut body = base.clone();
        while cost(&body) > energy {
          body.pop();
          if body.len() == 2 {
            return body;
          }
        }
        body
      }
      Claimer => base,
      Scout => base,
      _ => {
        let mut body = vec![];
        let num_parts = energy / self.cost();
        if num_parts == 0 {
          base
        } else {
          for part in base {
            for _ in 0..num_parts {
              body.push(part);
            }
          }
          body
        }
      }
    }
  }

  /// This works the role
  pub fn run(&self, creep: &Creeper) -> ReturnCode {
    match self {
      Role::Harvester => {
        if creep.working() {
          // get a target to fill
          if let Some(t) = creep.pos.find_transfer_target_primary(Some(Energy))
          {
            return creep.go_transfer_to_structure(&t, Energy, None);
          } else if let Some(t) =
            creep.pos.find_transfer_target_secondary(Some(Energy))
          {
            return creep.go_transfer_to_structure(&t, Energy, None);
          }
          // if we can't find a fill target go ahead and upgrade
          upgrade(creep)
        } else {
          get_energy(creep)
        }
      }
      Role::Miner => {
        // if there is a miner all harvesters must DIE
        let room = creep.room().unwrap();
        for creep in room.creeps_with_role(Harvester) {
          creep.suicide();
        }

        // first we find a minable source
        if let Some(source) =
          creep.pos.find_harvest_target::<Source>(Some(Energy))
        {
          // see if there is a container near it
          if let Some(target) = source.container() {
            return if creep.pos().eq(&target.pos()) {
              creep.harvest(&source)
            } else {
              creep.move_to(&target)
            };
          }
          return creep.go_harvest(&source);
        }

        // If there is no source try for other resources
        if let Some(source) = creep.pos.find_harvest_target::<Mineral>(None) {
          // see if there is a container near it
          if let Some(target) = source.container() {
            return if creep.pos().eq(&target.pos()) {
              creep.harvest(&source)
            } else {
              creep.move_to(&target)
            };
          }
          return creep.go_harvest(&source);
        }
        // Or deposits?
        if let Some(source) = creep.pos.find_harvest_target::<Deposit>(None) {
          // see if there is a container near it
          if let Some(target) = source.container() {
            return if creep.pos().eq(&target.pos()) {
              creep.harvest(&source)
            } else {
              creep.move_to(&target)
            };
          }
          return creep.go_harvest(&source);
        }
        // If all else fails report the issue
        error!("Miner has nowhere to mine!");
        return ReturnCode::NotEnough;
      }
      Role::Upgrader => {
        if creep.working() {
          upgrade(creep)
        } else {
          get_energy(creep)
        }
      }
      Role::Builder => {
        if creep.working() {
          // find build target
          if let Some(t) =
            creep.pos.find_build_target(Some(StructureType::Extension))
          {
            creep.go_build(&t)
          } else if let Some(t) =
            creep.pos.find_build_target(Some(StructureType::Container))
          {
            creep.go_build(&t)
          } else if let Some(t) = creep.pos.find_repair_target() {
            // repair next
            creep.go_repair(&t)
          } else {
            // then upgrade
            upgrade(creep)
          }
        } else {
          get_energy(creep)
        }
      }
      Role::Repairer => {
        if creep.working() {
          // find repair target
          if let Some(t) = creep.pos.find_repair_target() {
            creep.go_repair(&t)
          } else {
            upgrade(creep)
          }
        } else {
          get_energy(creep)
        }
      }
      Role::WallRepairer => {
        if creep.working() {
          // find wall to build
          if let Some(t) =
            creep.pos.find_build_target(Some(StructureType::Wall))
          {
            creep.go_build(&t)
          // find rampart to build
          } else if let Some(t) =
            creep.pos.find_build_target(Some(StructureType::Rampart))
          {
            creep.go_build(&t)
          // find wall repair
          } else if let Some(t) = creep.pos.find_wall_repair_target() {
            creep.go_repair(&t)
          } else {
            upgrade(creep)
          }
        } else {
          get_energy(creep)
        }
      }
      Role::Lorry => {
        if creep.working() {
          // find a transfer target
          // Should lorries deal in all resource types?
          let resources = creep.store_types();

          for resource in resources {
            if let Some(t) =
              creep.pos.find_transfer_target_primary(Some(resource))
            {
              return creep.go_transfer_to_structure(&t, resource, None);
            }
            if let Some(t) =
              creep.pos.find_transfer_target_secondary(Some(resource))
            {
              return creep.go_transfer_to_structure(&t, resource, None);
            }
          }

          // see if we can upgrade?
          warn!("Lorry can't find anywhere to put it's resources");
          if creep.store_used_capacity(Some(Energy)) > 0 {
            upgrade(creep)
          } else {
            warn!("lorry has no energy to upgrade");
            ReturnCode::NotEnough
          }
        } else {
          // find dropped resources of any type
          trace!("Lorry looking for resources");
          if let Some(t) = creep.pos.find_pickup_target(None) {
            trace!("Lorry found a pickup target");
            creep.go_pickup(&t)
          } else if let Some(t) = creep.pos.find_withdraw_target_primary(None)
          // find withdraw targets
          {
            trace!("Lorry found a withdraw target");
            if let Some(t) = t.as_tombstone() {
              let resource = t.store_types()[0];
              trace!("Tombstone detected");
              creep.go_withdraw(&t, resource, None)
            } else if let Some(t) = t.as_ruin() {
              let resource = t.store_types()[0];
              trace!("Ruin detected");
              creep.go_withdraw(&t, resource, None)
            } else if let Some(Structure::Container(t)) = t.as_structure() {
              let resource = t.store_types()[0];
              trace!("Container detected");
              creep.go_withdraw(&t, resource, None)
            } else {
              // Invalid item returned from withdraw target
              error!("Invalid item from withdraw target");
              ReturnCode::NotFound
            }
          } else if let Some(t) = creep.pos.find_withdraw_target_secondary(None)
          {
            trace!("Lorry found secondary withdraw target");
            let store = t.as_has_store().expect("find_withdraw_target_secondary returning a target without a store");
            let resource = store.store_types()[0];
            creep.go_withdraw_from_structure(&t, resource, None)
          } else {
            // creep cannot find any usable energy
            error!("Lorry can't find energy - are there miners?");
            ReturnCode::NotFound
          }
        }
      }
      Role::LinkLoader => {
        if creep.working() {
          // find a link to load
          let targets = creep
            .pos
            .find(find::MY_STRUCTURES)
            .into_iter()
            .filter_map(|s| {
              let s = s.as_structure();
              if let Structure::Link(s) = s {
                if s.store_free_capacity(None) > 0 {
                  return Some(s);
                }
              }
              None
            })
            .collect::<Vec<_>>();
          if let Some(t) = creep.pos.find_closest_by_path(targets) {
            creep.go_transfer(&t, Energy, None)
          } else {
            // No links
            warn!("Link loader can't find any links to load");
            ReturnCode::Full
          }
        } else {
          get_energy(creep)
        }
      }
      Role::Claimer => {
        // find a claim target
        if let Some(t) = creep.pos.find_claim_target() {
          creep.go_claim_controller(&t)
        } else {
          // Nothing to claim
          Role::Scout.run(creep)
        }
      }
      Role::Reserver => {
        // find a reserve target
        if let Some(t) = creep.pos.find_reserve_target() {
          creep.go_reserve_controller(&t)
        } else {
          Role::Scout.run(creep)
        }
      }
      Role::Scout => {
        // go to a scout flag
        if let Some(flag) = game::flags::get("scout") {
          creep.move_to(&flag)
        } else {
          // Try to keep a scout flag
          ReturnCode::NotFound
        }
      }
      Role::Soldier => {
        // find any enemies in the room
        // attack them
        // if there are no enemies find a rally point
        if let Some(t) = creep.pos.find_attack_target::<Creep>() {
          creep.go_attack(&t)
        } else if let Some(t) = creep.pos.find_attack_target::<PowerCreep>() {
          creep.go_attack(&t)
        } else if let Some(t) = creep.pos.find_attack_structure() {
          creep.go_attack(&t)
        } else {
          // go to rally
          if let Some(t) = creep.pos.find_rally_point() {
            return creep.move_to(&t);
          }
          warn!("Make sure you have a valid rally target");
          ReturnCode::NotFound
        }
      }
      Role::Healer => {
        // find any heal target
        // heal them
        // find any enemies to attack if still have a ranged attack part
        // attack them
        if let Some(t) = creep.pos.find_heal_target() {
          return creep.go_heal_creep(&t);
        } else if let Some(t) = creep.pos.find_heal_target() {
          return creep.go_heal_power_creep(&t);
        }

        if creep.get_active_bodyparts(Part::RangedAttack) > 0 {
          if let Some(t) = creep.pos.find_attack_target::<Creep>() {
            return creep.go_attack(&t);
          }
          if let Some(t) = creep.pos.find_attack_target::<PowerCreep>() {
            return creep.go_attack(&t);
          }
          if let Some(t) = creep.pos.find_attack_structure() {
            return creep.go_attack(&t);
          }
        }

        if let Some(t) = creep.pos.find_rally_point() {
          return creep.move_to(&t);
        }
        warn!("Make sure you have a rally");
        ReturnCode::NotFound
      }
    }
  }

  /// Spawn the specified role
  pub fn spawn(&self, spawn: &StructureSpawn) -> ReturnCode {
    let energy = spawn.room().unwrap().energy_capacity_available();

    let body = self.expand(energy);

    let name = get_random_name(spawn.room().unwrap());

    let opts = SpawnOptions::new()
      .memory(MemoryReference::new().set_value(Values::Role(*self)));

    spawn.spawn_creep_with_options(&body, &name, &opts)
  }

  /// Spawn the role NOW (emergency mode)
  pub fn spawn_now(&self, spawn: &StructureSpawn) -> ReturnCode {
    let energy = spawn.room().unwrap().energy_available();

    let body = self.expand(energy);

    let name = get_random_name(spawn.room().unwrap());

    let opts = SpawnOptions::new()
      .memory(MemoryReference::new().set_value(Values::Role(*self)));

    spawn.spawn_creep_with_options(&body, &name, &opts)
  }

  /// Spawn all emergency situations
  /// Return true if something is spawned
  pub fn spawn_emergencies(spawn: &StructureSpawn) -> bool {
    let room = spawn.room().unwrap();

    use Role::*;
    if room.creeps_with_role(Lorry).len() == 0
      && room.creeps_with_role(Harvester).len() == 0
    {
      // No harvesters or lorries
      if room.creeps_with_role(Miner).len() == 0 {
        Harvester.spawn_now(spawn);
      } else {
        Lorry.spawn_now(spawn);
      }
      true
    } else {
      false
    }
  }

  /// Spawn the minimum of each role
  pub fn spawn_min(spawn: &StructureSpawn) -> bool {
    use Role::*;
    let room = spawn.room().unwrap();
    let total_energy = room.energy_capacity_available();
    // Enumerate the roles to have at least 1 of
    let mut roles: BTreeMap<Role, usize> = BTreeMap::new();
    let gatherers_needed = room.find(find::SOURCES).len()
      + room
        .find(find::STRUCTURES)
        .into_iter()
        .filter(|s| s.structure_type() == StructureType::Extractor)
        .collect::<Vec<Structure>>()
        .len();
    if total_energy >= Miner.cost() {
      roles.insert(Miner, gatherers_needed);
      roles.insert(Lorry, (total_energy / 500) as usize);
    } else {
      roles.insert(Harvester, gatherers_needed);
    }
    roles.insert(Upgrader, 1);
    roles.insert(Repairer, 1);
    roles.insert(Builder, 1);
    roles.insert(WallRepairer, 1);
    roles.insert(Soldier, if total_energy >= Soldier.cost() { 1 } else { 0 });
    roles.insert(Healer, if total_energy >= Healer.cost() { 1 } else { 0 });

    for (role, count) in roles {
      let creeps = room.creeps_with_role(role);
      if creeps.len() < count {
        role.spawn(spawn);
        return true;
      }
    }

    false
  }

  /// Spawn any specials or extras
  pub fn spawn_extras(spawn: &StructureSpawn) -> ReturnCode {
    // These are case by case extra creeps that need spawned
    use Role::*;
    let room = spawn.room().unwrap();

    // calculate how many creeps we need
    let creep_load = room.find(find::SOURCES).len()
      + room
        .find(find::STRUCTURES)
        .into_iter()
        .filter(|s| s.structure_type() == StructureType::Extractor)
        .collect::<Vec<Structure>>()
        .len();

    let roles = if Miner.cost() <= room.energy_capacity_available() {
      vec![
        Miner,
        Lorry,
        Upgrader,
        Repairer,
        Builder,
        WallRepairer,
        Soldier,
        Healer,
      ]
    } else {
      vec![
        Harvester,
        Upgrader,
        Repairer,
        Builder,
        WallRepairer,
        Soldier,
        Healer,
      ]
    };

    for role in roles {
      if room.creeps_with_role(role).len() < creep_load {
        return role.spawn(spawn);
      }
    }

    // Scouting: We should scout if there is a scout flag out
    if let Some(_) = game::flags::get("scout") {
      // And if we haven't already spawned a scout
      if room.creeps_with_role(Scout).len() == 0 {
        return Scout.spawn(spawn);
      }
    }

    ReturnCode::Full
  }
}

fn cost(body: &Vec<Part>) -> u32 {
  let mut cost = 0;
  for part in body {
    cost += part.cost();
  }
  cost
}

fn upgrade(creep: &Creeper) -> ReturnCode {
  let ctrl = creep.room().unwrap().controller().unwrap() as StructureController;
  if let Some(sign) = ctrl.sign() {
    if sign.username == creep.owner_name() {
      return creep.go_upgrade_controller(&ctrl);
    }
  }

  creep.go_sign_controller(&ctrl)
}

fn get_energy(creep: &Creeper) -> ReturnCode {
  // first pickup loose resources
  if let Some(t) = creep.pos.find_pickup_target(Some(Energy)) {
    return creep.go_pickup(&t);
  }

  // next find a withdraw target
  if let Some(t) = creep.pos.find_withdraw_target_primary(Some(Energy)) {
    // Tombstone?
    if let Some(t) = t.as_tombstone() {
      return creep.go_withdraw(&t, Energy, None);
    }
    // how about a Ruin?
    if let Some(t) = t.as_ruin() {
      return creep.go_withdraw(&t, Energy, None);
    }
    // is it a structure?
    if let Some(t) = t.as_structure() {
      return creep.go_withdraw_from_structure(&t, Energy, None);
    }
  }

  // finally go to an active source only if there are no miners and you have a work part
  let room = creep.room().unwrap();
  if room.creeps_with_role(Role::Miner).len() > 0
    || creep.get_active_bodyparts(Part::Work) == 0
  {
    error!(
      "Creep can't find energy without encroaching on a miner: {}",
      creep.name()
    );
    return ReturnCode::NoBodypart;
  }

  if let Some(t) = creep.pos.find_harvest_target::<Source>(Some(Energy)) {
    return creep.go_harvest(&t);
  }
  // if we can't find a source try salvaging
  if let Some(t) = creep.pos.find_dismantle_target() {
    return creep.go_dismantle(&t);
  }
  error!("Creep can't find any energy!");
  ReturnCode::NotEnough
}
